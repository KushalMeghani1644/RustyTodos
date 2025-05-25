// src/todo.rs
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Todo {
    pub id: u32,
    pub description: String,
    pub done: bool,
    pub created_at: String,
    pub due_date: Option<String>,
}

pub fn load_todos() -> Vec<Todo> {
    match fs::read_to_string("todos.json") {
        Ok(data) => serde_json::from_str(&data).unwrap_or_else(|_| Vec::new()),
        Err(_) => Vec::new(),
    }
}

pub fn save_todos(todos: &Vec<Todo>) {
    let data = serde_json::to_string_pretty(todos).expect("Failed to serialize todos");
    fs::write("todos.json", data).expect("Failed to write todos to file!");
}
