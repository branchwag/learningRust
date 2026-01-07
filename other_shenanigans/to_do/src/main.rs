use eframe::egui;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Default)]
struct Todo {
    text: String,
    done: bool,
}

#[derive(Default)]
struct TodoApp {
    todos: Vec<Todo>,
    new_todo: String,
}

fn load_todos() -> Vec<Todo> {
    fs::read_to_string("todos.json")
        .ok()
        .and_then(|data| serde_json::from_str(&data).ok())
        .unwrap_or_default()
}

fn save_todos(todos: &Vec<Todo>) {
    if let Ok(data) = serde_json::to_string_pretty(todos) {
        let _ = fs::write("todos.json", data);
    }
}

impl TodoApp {
    fn new() -> Self {
        Self {
            todos: load_todos(),
            new_todo: String::new(),
        }
    }
}

impl eframe::App for TodoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("📝 Todo App");

            ui.horizontal(|ui| {
                let input = ui.text_edit_singleline(&mut self.new_todo);
                if input.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    self.add_todo();
                }

                if ui.button("Add").clicked() {
                    self.add_todo();
                }
            });

            ui.separator();

            let mut remove_index: Option<usize> = None;

            for (i, todo) in self.todos.iter_mut().enumerate() {
                ui.horizontal(|ui| {
                    ui.checkbox(&mut todo.done, "");
                    ui.label(&todo.text);

                    if ui.button("❌").clicked() {
                        remove_index = Some(i);
                    }
                });
            }

            if let Some(i) = remove_index {
                self.todos.remove(i);
                save_todos(&self.todos);
            }
        });
    }
}

impl TodoApp {
    fn add_todo(&mut self) {
        if !self.new_todo.trim().is_empty() {
            self.todos.push(Todo {
                text: self.new_todo.trim().to_string(),
                done: false,
            });
            self.new_todo.clear();
            save_todos(&self.todos);
        }
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Todo App",
        options,
        Box::new(|_cc| Box::new(TodoApp::new())),
    )
}
