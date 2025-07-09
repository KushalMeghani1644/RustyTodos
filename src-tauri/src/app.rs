use crate::todo::Todo;
use chrono::Local;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};

pub fn get_data_file_path() -> PathBuf {
    let proj_dirs = ProjectDirs::from("com", "KushalMeghani", "RustyTodos")
        .expect("Failed to get project directories");
    let dir = proj_dirs.config_dir();
    std::fs::create_dir_all(dir).unwrap();
    dir.join("todos.json")
}

#[derive(Serialize, Deserialize)]
pub struct App {
    pub todos: Vec<Todo>,
}

impl Default for App {
    fn default() -> Self {
        Self { todos: Vec::new() }
    }
}

impl App {
    pub fn add_todo(&mut self, description: String, due_date: Option<String>) -> Result<(), String> {
        if description.trim().is_empty() {
            return Err("Description cannot be empty.".to_string());
        }

        self.todos.push(Todo {
            description,
            due_date,
            created_date: Local::now().format("%Y-%m-%d").to_string(),
            done: false,
        });

        Ok(())
    }

    pub fn delete_todo_at(&mut self, index: usize) {
        if index < self.todos.len() {
            self.todos.remove(index);
        }
    }

    pub fn toggle_done(&mut self, index: usize) {
        if let Some(todo) = self.todos.get_mut(index) {
            todo.done = !todo.done;
        }
    }

    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), String> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)
            .map_err(|e| format!("Failed to open file: {}", e))?;

        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, self)
            .map_err(|e| format!("Failed to write JSON!: {}", e))
    }

    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Self {
        let file = File::open(&path);
        if let Ok(file) = file {
            let reader = BufReader::new(file);
            serde_json::from_reader(reader).unwrap_or_else(|_| App::default())
        } else {
            App::default()
        }
    }
}
