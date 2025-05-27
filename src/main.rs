// main.rs

mod app;
mod daemon;
mod todo;
mod tui;

use crate::app::{App, get_data_file_path};
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io::{self};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::thread::spawn(|| {
        if let Err(e) = daemon::start_daemon() {
            eprintln!("Daemon error: {}", e);
        }
    });

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let data_path = get_data_file_path();

    // Load app state or start fresh
    let mut app = App::load_from_file(&data_path);

    // Run your TUI event loop (this should block until exit)
    let res = tui::run_app(&mut terminal, &mut app);

    // Restore terminal state
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    // Save app state on exit
    if let Err(e) = app.save_to_file(&data_path) {
        eprintln!("Failed to save todos: {}", e);
    }

    // Handle errors from the event loop if any
    if let Err(err) = res {
        eprintln!("Application error: {}", err);
    }

    Ok(())
}
