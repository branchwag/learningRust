use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, put},
};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool, sqlite::SqlitePoolOptions};
use std::{error::Error, fmt};
use tokio::net::TcpListener;

// The model returned to API clients and read from SQLite.
#[derive(Debug, Serialize, FromRow)]
struct Task {
    id: i64,
    title: String,
    completed: bool,
}

// JSON body accepted by POST /tasks.
#[derive(Debug, Deserialize)]
struct CreateTask {
    title: String,
}

// JSON body accepted by PUT /tasks/{id}.
#[derive(Debug, Deserialize)]
struct UpdateTask {
    title: String,
    completed: bool,
}

// Shared application state.
#[derive(Clone)]
struct AppState {
    db: SqlitePool,
}

// Error type used by the HTTP handlers.
#[derive(Debug)]
enum AppError {
    NotFound,
    BadRequest(String),
    Database(sqlx::Error),
}

impl From<sqlx::Error> for AppError {
    fn from(error: sqlx::Error) -> Self {
        Self::Database(error)
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound => write!(formatter, "resource not found"),
            Self::BadRequest(message) => write!(formatter, "{message}"),
            Self::Database(error) => write!(formatter, "database error: {error}"),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::NotFound => (StatusCode::NOT_FOUND, "Task not found".to_string()),
            Self::BadRequest(message) => (StatusCode::BAD_REQUEST, message),
            Self::Database(error) => {
                eprintln!("Database error: {error}");

                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
        };

        (status, Json(ErrorResponse { error: message })).into_response()
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let database_url = "sqlite://tasks.db?mode=rwc";

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let state = AppState { db: pool };

    let app = Router::new()
        .route("/tasks", get(list_tasks).post(create_task))
        .route(
            "/tasks/{id}",
            get(get_task).put(update_task).delete(delete_task),
        )
        .with_state(state);

    let listener = TcpListener::bind("127.0.0.1:3000").await?;

    println!("Server running at http://127.0.0.1:3000");

    axum::serve(listener, app).await?;

    Ok(())
}

// CREATE
async fn create_task(
    State(state): State<AppState>,
    Json(payload): Json<CreateTask>,
) -> Result<(StatusCode, Json<Task>), AppError> {
    let title = payload.title.trim();

    if title.is_empty() {
        return Err(AppError::BadRequest("Title cannot be empty".to_string()));
    }

    let result = sqlx::query(
        r#"
        INSERT INTO tasks (title, completed)
        VALUES (?, FALSE)
        "#,
    )
    .bind(title)
    .execute(&state.db)
    .await?;

    let task_id = result.last_insert_rowid();

    let task = find_task(&state.db, task_id).await?;

    Ok((StatusCode::CREATED, Json(task)))
}

// READ ALL
async fn list_tasks(State(state): State<AppState>) -> Result<Json<Vec<Task>>, AppError> {
    let tasks = sqlx::query_as::<_, Task>(
        r#"
        SELECT id, title, completed
        FROM tasks
        ORDER BY id ASC
        "#,
    )
    .fetch_all(&state.db)
    .await?;

    Ok(Json(tasks))
}

// READ ONE
async fn get_task(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<Task>, AppError> {
    let task = find_task(&state.db, id).await?;

    Ok(Json(task))
}

// UPDATE
async fn update_task(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateTask>,
) -> Result<Json<Task>, AppError> {
    let title = payload.title.trim();

    if title.is_empty() {
        return Err(AppError::BadRequest("Title cannot be empty".to_string()));
    }

    let result = sqlx::query(
        r#"
        UPDATE tasks
        SET title = ?, completed = ?
        WHERE id = ?
        "#,
    )
    .bind(title)
    .bind(payload.completed)
    .bind(id)
    .execute(&state.db)
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    let task = find_task(&state.db, id).await?;

    Ok(Json(task))
}

// DELETE
async fn delete_task(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    let result = sqlx::query(
        r#"
        DELETE FROM tasks
        WHERE id = ?
        "#,
    )
    .bind(id)
    .execute(&state.db)
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    Ok(StatusCode::NO_CONTENT)
}

// Shared database lookup used by multiple handlers.
async fn find_task(pool: &SqlitePool, id: i64) -> Result<Task, AppError> {
    let task = sqlx::query_as::<_, Task>(
        r#"
        SELECT id, title, completed
        FROM tasks
        WHERE id = ?
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?
    .ok_or(AppError::NotFound)?;

    Ok(task)
}
