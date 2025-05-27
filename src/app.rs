// app.rs

use crate::todo::Todo;
use crate::tui::parse_due_date;
use chrono::Local;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};

#[derive(PartialEq, Deserialize, Serialize)]
pub enum InputMode {
    Normal,
    EditingDescription,
    EditingDueDate,
}

#[derive(Serialize, Deserialize)]
pub struct App {
    pub todos: Vec<Todo>,

    #[serde(skip)]
    pub input_mode: InputMode,
    #[serde(skip)]
    pub input_description: String,
    #[serde(skip)]
    pub input_due_date: String,
    #[serde(skip)]
    pub selected: usize,
    #[serde(skip)]
    pub error_message: Option<String>,
}

impl Default for InputMode {
    fn default() -> Self {
        InputMode::Normal
    }
}

impl Default for App {
    fn default() -> Self {
        App::new()
    }
}

impl App {
    pub fn new() -> Self {
        Self {
            todos: Vec::new(),
            input_mode: InputMode::Normal,
            input_description: String::new(),
            input_due_date: String::new(),
            selected: 0,
            error_message: None,
        }
    }

    pub fn add_todo(&mut self) -> Result<(), String> {
        if self.input_description.trim().is_empty() {
            return Err("Description cannot be empty.".to_string());
        }

        let due_date_str = if self.input_due_date.trim().is_empty() {
            None
        } else {
            Some(parse_due_date(&self.input_due_date)?)
        };

        self.todos.push(Todo {
            description: self.input_description.clone(),
            done: false,
            due_date: due_date_str,
            created_date: Local::now().format("%Y-%m-%d").to_string(),
        });

        // clear inputs after adding
        self.input_description.clear();
        self.input_due_date.clear();
        self.error_message = None;

        Ok(())
    }

    pub fn delete_todo(&mut self) {
        if !self.todos.is_empty() {
            self.todos.remove(self.selected);
            if self.selected > 0 {
                self.selected -= 1;
            }
        }
    }

    pub fn mark_done(&mut self) {
        if let Some(todo) = self.todos.get_mut(self.selected) {
            todo.done = !todo.done;
        }
    }

    pub fn save_to_file(&self, path: &str) -> Result<(), String> {
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

    pub fn load_from_file(path: &str) -> Self {
        let file = File::open(path);
        if let Ok(file) = file {
            let reader = BufReader::new(file);
            serde_json::from_reader(reader).unwrap_or_else(|_| App::new())
        } else {
            App::new()
        }
    }
}
