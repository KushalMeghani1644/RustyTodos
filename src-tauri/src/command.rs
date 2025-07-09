// src-tauri/src/command.rs

use crate::app::{App, get_data_file_path};
use crate::date_parser::parse_due_date;
use crate::todo::Todo;
use serde::Deserialize;
use std::sync::Mutex;
use tauri::State;
/// Add a new todo from GUI (with stringified date input)
#[derive(Debug, Deserialize)]
pub struct AddTodoArgs {
    description: String,
    due_date: Option<String>,
}
#[tauri::command]
pub fn add_todo(app_state: State<'_, Mutex<App>>, args: AddTodoArgs) -> Result<(), String> {
    let mut app = app_state.lock().unwrap();

    if args.description.trim().is_empty() {
        return Err("Description cannot be empty.".into());
    }
    let due = if let Some(ref due_raw) = args.due_date {
        Some(parse_due_date(&due_raw)?)
    } else {
        None
    };

    app.todos.push(Todo::new(args.description, due));
    app.save_to_file(get_data_file_path()).ok(); // Try saving, but ignore errors here
    Ok(())
}

/// Toggle done/undone for the given index
#[tauri::command]
pub fn mark_done(app_state: State<'_, Mutex<App>>, index: usize) -> Result<(), String> {
    let mut app = app_state.lock().unwrap();
    if let Some(todo) = app.todos.get_mut(index) {
        todo.done = !todo.done;
        app.save_to_file(get_data_file_path()).ok();
    }
    Ok(())
}

/// Delete the todo at the selected index
#[tauri::command]
pub fn delete_todo(app_state: State<'_, Mutex<App>>, index: usize) -> Result<(), String> {
    let mut app = app_state.lock().unwrap();
    if index < app.todos.len() {
        app.todos.remove(index);
        app.save_to_file(get_data_file_path()).ok();
    }
    Ok(())
}

/// Return all todos to the frontend
#[tauri::command]
pub fn get_todos(app_state: State<'_, Mutex<App>>) -> Vec<Todo> {
    let app = app_state.lock().unwrap();
    app.todos.clone()
}

/// Save the current todo list
#[tauri::command]
pub fn save_tasks(app_state: State<'_, Mutex<App>>) -> Result<(), String> {
    let app = app_state.lock().unwrap();
    app.save_to_file(get_data_file_path())
}

/// Load todos from disk (used on app startup)
#[tauri::command]
pub fn load_tasks(app_state: State<'_, Mutex<App>>) -> Result<(), String> {
    let mut app = app_state.lock().unwrap();
    *app = App::load_from_file(get_data_file_path());
    Ok(())
}
