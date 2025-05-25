use crate::todo::{Todo, load_todos, save_todos};
use chrono::Local;

pub struct App {
    pub todos: Vec<Todo>,
    pub selected: usize,
    pub input_mode: InputMode,
    pub input_description: String,
    pub input_due_date: String,
}

pub enum InputMode {
    Normal,
    EditingDescription,
    EditingDueDate,
}

impl App {
    pub fn new() -> Self {
        Self {
            todos: load_todos(),
            selected: 0,
            input_mode: InputMode::Normal,
            input_description: String::new(),
            input_due_date: String::new(),
        }
    }

    pub fn add_todo(&mut self) {
        let id = self.todos.len() as u32 + 1;
        let created_at = Local::now().format("%Y-%m-%d").to_string();
        let due_date = if self.input_due_date.is_empty() {
            None
        } else {
            Some(self.input_due_date.clone())
        };
        let todo = Todo {
            id,
            description: self.input_description.clone(),
            done: false,
            created_at,
            due_date,
        };
        self.todos.push(todo);
        save_todos(&self.todos);
        self.input_description.clear();
        self.input_due_date.clear();
    }

    pub fn mark_done(&mut self) {
        if let Some(todo) = self.todos.get_mut(self.selected) {
            todo.done = true;
            save_todos(&self.todos);
        }
    }

    pub fn delete_todo(&mut self) {
        if self.selected < self.todos.len() {
            self.todos.remove(self.selected);
            if self.selected > 0 {
                self.selected -= 1;
            }
            save_todos(&self.todos);
        }
    }
}
