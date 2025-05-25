// tui.rs

use crate::app::{App, InputMode};
use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event as CEvent, KeyCode, KeyEventKind,
    },
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
};
use std::{io, time::Duration};

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if crossterm::event::poll(Duration::from_millis(100))? {
            if let CEvent::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                match app.input_mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Char('a') => {
                            app.input_mode = InputMode::EditingDescription;
                            app.input_description.clear();
                            app.input_due_date.clear();
                            app.error_message = None;
                        }
                        KeyCode::Char('d') => app.delete_todo(),
                        KeyCode::Char('m') => app.mark_done(),
                        KeyCode::Down => {
                            if app.selected < app.todos.len().saturating_sub(1) {
                                app.selected += 1;
                            }
                        }
                        KeyCode::Up => {
                            if app.selected > 0 {
                                app.selected -= 1;
                            }
                        }
                        _ => {}
                    },
                    InputMode::EditingDescription => match key.code {
                        KeyCode::Enter => {
                            app.input_mode = InputMode::EditingDueDate;
                        }
                        KeyCode::Esc => {
                            app.input_mode = InputMode::Normal;
                        }
                        KeyCode::Char(c) => {
                            app.input_description.push(c);
                        }
                        KeyCode::Backspace => {
                            app.input_description.pop();
                        }
                        _ => {}
                    },
                    InputMode::EditingDueDate => match key.code {
                        KeyCode::Enter => {
                            match app.add_todo() {
                                Ok(_) => app.input_mode = InputMode::Normal,
                                Err(e) => {
                                    app.error_message = Some(e);
                                    // Stay in due date input mode
                                }
                            }
                        }
                        KeyCode::Esc => {
                            app.input_mode = InputMode::Normal;
                            app.error_message = None;
                        }
                        KeyCode::Char(c) => {
                            app.input_due_date.push(c);
                        }
                        KeyCode::Backspace => {
                            app.input_due_date.pop();
                        }
                        _ => {}
                    },
                }
            }
        }
    }
}

fn ui<B: Backend>(f: &mut ratatui::Frame<B>, app: &App) {
    let size = f.size();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(3), // title
                Constraint::Length(3), // help
                Constraint::Min(1),    // todo list
                Constraint::Length(5), // description input
                Constraint::Length(3), // due date input (too small)
            ]
            .as_ref(),
        )
        .split(size);

    let title = Paragraph::new(Span::styled(
        "🌟 RustyTodos! 🌟",
        Style::default()
            .fg(Color::Magenta)
            .add_modifier(Modifier::BOLD),
    ))
    .alignment(Alignment::Center);
    f.render_widget(title, chunks[0]);

    let help = Paragraph::new(Spans::from(vec![
        Span::raw("Press "),
        Span::styled("a", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" to add, "),
        Span::styled("m", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" to mark done, "),
        Span::styled("d", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" to delete, "),
        Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" to quit."),
    ]))
    .alignment(Alignment::Center);
    f.render_widget(help, chunks[1]);

    let todos: Vec<ListItem> = app
        .todos
        .iter()
        .map(|t| {
            let status = if t.done { "[x]" } else { "[ ]" };
            let due = t
                .due_date
                .clone()
                .unwrap_or_else(|| "No due date".to_string());
            let created = t.created_date.clone();
            ListItem::new(format!(
                "{} {} (Due: {}) [Created: {}]",
                status, t.description, due, created
            ))
        })
        .collect();

    let mut list_state = ratatui::widgets::ListState::default();
    list_state.select(Some(app.selected));

    let todos_list = List::new(todos)
        .block(Block::default().borders(Borders::ALL).title("Todos"))
        .highlight_style(
            Style::default()
                .bg(Color::Blue)
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    f.render_stateful_widget(todos_list, chunks[2], &mut list_state);

    let description_style = if matches!(app.input_mode, InputMode::EditingDescription) {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };

    let due_date_style = if matches!(app.input_mode, InputMode::EditingDueDate) {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };

    // Show input with caret
    let caret = "|"; // caret symbol

    let desc_with_caret = if matches!(app.input_mode, InputMode::EditingDescription) {
        format!("{}{}", app.input_description, caret)
    } else {
        app.input_description.clone()
    };
    let due_with_caret = if matches!(app.input_mode, InputMode::EditingDueDate) {
        if app.input_due_date.is_empty() {
            caret.to_string()
        } else {
            format!("{}{}", app.input_due_date, caret)
        }
    } else {
        app.input_due_date.clone()
    };

    let input_desc = Paragraph::new(desc_with_caret)
        .block(Block::default().borders(Borders::ALL).title("Description"))
        .style(description_style)
        .wrap(Wrap { trim: true });

    let input_due = Paragraph::new(due_with_caret)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Due Date (YYYY-MM-DD)"),
        )
        .style(due_date_style)
        .wrap(Wrap { trim: true });

    f.render_widget(input_desc, chunks[3]);
    f.render_widget(input_due, chunks[4]);

    // Show error message if any
    if let Some(ref msg) = app.error_message {
        let error = Paragraph::new(msg.as_str())
            .style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center);
        let area = ratatui::layout::Rect {
            x: size.x,
            y: size.height.saturating_sub(2),
            width: size.width,
            height: 1,
        };
        f.render_widget(error, area);
    }
}
