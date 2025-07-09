mod app;
mod command;
mod daemon;
mod date_parser;
mod todo;

use crate::app::{App, get_data_file_path};
use crate::command::*;
use std::sync::Mutex;
use tauri::{Builder, generate_context, generate_handler};

fn main() {
    let app = App::load_from_file(get_data_file_path());
    let shared_app = Mutex::new(app);
    // let context = tauri::generate_context!();
    std::thread::spawn(|| {
        if let Err(e) = daemon::start_daemon() {
            eprintln!("Daemon error: {}", e);
        }
    });

    Builder::default()
        .manage(shared_app)
        .invoke_handler(tauri::generate_handler![
            add_todo,
            mark_done,
            delete_todo,
            get_todos,
            save_tasks,
            load_tasks,
        ])
        .run(generate_context!())
        .expect("error while running tauri application");
}
