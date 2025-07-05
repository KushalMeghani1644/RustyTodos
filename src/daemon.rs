//daemon.rs
use crate::app::{App, get_data_file_path};
use chrono::Local;
use std::{thread, time::Duration};

#[cfg(target_os = "linux")]
use notify_rust::Notification;

#[cfg(target_os = "windows")]
use notifica::notify;

#[cfg(target_os = "macos")]
use macos_notification_sys::*;

pub fn start_daemon() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let data_path = get_data_file_path();
        let app = App::load_from_file(&data_path);
        let today = Local::now().format("%Y-%m-%d").to_string();

        for todo in &app.todos {
            if !todo.done {
                if let Some(due) = &todo.due_date {
                    if due == &today {
                        #[cfg(target_os = "linux")]
                        Notification::new()
                            .summary("Todo Due today!")
                            .body(&format!(
                                "\"{}\" is due today! Don't forget!",
                                todo.description
                            ))
                            .show()?;
                        #[cfg(target_os = "windows")]
                        {
                            notify(
                                "RustyTodos",
                                &format!("\"{}\" is due today! Don't forget!", todo.description),
                            );
                        }
                        #[cfg(target_os = "macos")]
                        {
                            send_notification(
                                "RustyTodos",
                                &None,
                                &format!("\"{}\" is due today! Don't forget!", todo.description),
                                None,
                            )?;
                        }
                    }
                }
            }
        }
        thread::sleep(Duration::from_secs(60));
    }
}
