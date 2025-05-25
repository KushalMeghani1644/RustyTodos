// app.rs
use crate::todo::Todo;
use chrono::{Local, NaiveDate};

#[derive(PartialEq)]
pub enum InputMode {
    Normal,
    EditingDescription,
    EditingDueDate,
}

pub struct App {
    pub todos: Vec<Todo>,
    pub input_mode: InputMode,
    pub input_description: String,
    pub input_due_date: String,
    pub selected: usize,
    pub error_message: Option<String>,
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

    // Return Result<(), String> to allow error handling in tui
    pub fn add_todo(&mut self) -> Result<(), String> {
        if self.input_description.trim().is_empty() {
            return Err("Description cannot be empty!".into());
        }
        let due_date_option = if self.input_due_date.trim().is_empty() {
            None
        } else {
            match NaiveDate::parse_from_str(&self.input_due_date, "%Y-%m-%d") {
                Ok(date) => {
                    let today = Local::now().date_naive();
                    if date < today {
                        return Err("Due date cannot be in the past!".into());
                    }
                    Some(date.format("%Y-%m-%d").to_string())
                }
                Err(_) => {
                    return Err("Invalid date format! Use YYYY-MM-DD.".into());
                }
            }
        };

        self.todos
            .push(Todo::new(self.input_description.clone(), due_date_option));
        self.error_message = None;
        self.input_description.clear();
        self.input_due_date.clear();
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
}
