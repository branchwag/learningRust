use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use sqlx::SqlitePool;

use crate::{
    error::{AppError, Result},
    models::{CreateItem, Item, UpdateItem},
};

pub async fn list_items(State(pool): State<SqlitePool>) -> Result<Json<Vec<Item>>> {
    let items = sqlx::query_as!(Item, "SELECT id, name, description FROM items")
        .fetch_all(&pool)
        .await?;

    Ok(Json(items))
}

pub async fn get_item(State(pool): State<SqlitePool>, Path(id): Path<i64>) -> Result<Json<Item>> {
    let item = sqlx::query_as!(
        Item,
        "SELECT id, name, description FROM items WHERE id = ?",
        id
    )
    .fetch_optional(&pool)
    .await?
    .ok_or(AppError::NotFound)?;

    Ok(Json(item))
}

pub async fn create_item(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateItem>,
) -> Result<impl IntoResponse> {
    if payload.name.trim().is_empty() {
        return Err(AppError::BadRequest("name cannot be empty".into()));
    }

    let item = sqlx::query_as!(
        Item,
        "INSERT INTO items (name, description) VALUES (?, ?) RETURNING id, name, description",
        payload.name,
        payload.description
    )
    .fetch_one(&pool)
    .await?;

    Ok((StatusCode::CREATED, Json(item)))
}

pub async fn update_item(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateItem>,
) -> Result<Json<Item>> {
    // verify it exists first
    let existing = sqlx::query_as!(
        Item,
        "SELECT id, name, description FROM items WHERE id = ?",
        id
    )
    .fetch_optional(&pool)
    .await?
    .ok_or(AppError::NotFound)?;

    let new_name = payload.name.unwrap_or(existing.name);
    let new_desc = payload.description.or(existing.description);

    let item = sqlx::query_as!(
        Item,
        "UPDATE items SET name = ?, description = ? WHERE id = ? RETURNING id, name, description",
        new_name,
        new_desc,
        id
    )
    .fetch_one(&pool)
    .await?;

    Ok(Json(item))
}

pub async fn delete_item(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<StatusCode> {
    let result = sqlx::query!("DELETE FROM items WHERE id = ?", id)
        .execute(&pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    Ok(StatusCode::NO_CONTENT)
}
