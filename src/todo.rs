// todo.rs

use chrono::Local;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct Todo {
    pub description: String,
    pub due_date: Option<String>,
    pub created_date: String,
    pub done: bool,
}

impl Todo {
    pub fn new(description: String, due_date: Option<String>) -> Self {
        Self {
            description,
            due_date,
            created_date: Local::now().format("%Y-%m-%d").to_string(),
            done: false,
        }
    }
}
