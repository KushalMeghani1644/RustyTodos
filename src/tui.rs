use crate::app::{App, InputMode};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event as CEvent, KeyCode},
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
use std::{
    io,
    time::{Duration, Instant},
};

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if crossterm::event::poll(Duration::from_millis(100))? {
            if let CEvent::Key(key) = event::read()? {
                match app.input_mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Char('a') => {
                            app.input_mode = InputMode::EditingDescription;
                            app.input_description.clear();
                            app.input_due_date.clear();
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
                            app.add_todo();
                            app.input_mode = InputMode::Normal;
                        }
                        KeyCode::Esc => {
                            app.input_mode = InputMode::Normal;
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
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(1),
                Constraint::Length(5),
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
            ListItem::new(format!("{} {} (Due: {})", status, t.description, due))
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

    let input = Paragraph::new(vec![
        Spans::from(vec![
            Span::styled(
                "Description: ",
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::styled(&app.input_description, description_style),
        ]),
        Spans::from(vec![
            Span::styled("Due Date: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::styled(&app.input_due_date, due_date_style),
        ]),
    ])
    .block(Block::default().borders(Borders::ALL).title("Input"))
    .wrap(ratatui::widgets::Wrap { trim: false });

    f.render_widget(input, chunks[3]);
}
