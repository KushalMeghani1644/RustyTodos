// tui.rs

use crate::app::{App, InputMode};
use chrono::{
    Datelike, Duration as Dur, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Timelike,
    Weekday,
};
use crossterm::event::{self, Event as CEvent, KeyCode, KeyEventKind};
use ratatui::{
    Terminal,
    backend::Backend,
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
                        KeyCode::Char('/') => {
                            app.input_mode = InputMode::Searching;
                            app.search_query.clear();
                        }
                        KeyCode::Down => {
                            if app.selected < filtered_todos(app).len().saturating_sub(1) {
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
                        KeyCode::Enter => match app.add_todo() {
                            Ok(_) => app.input_mode = InputMode::Normal,
                            Err(e) => {
                                app.error_message = Some(e);
                            }
                        },
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
                    InputMode::Searching => match key.code {
                        KeyCode::Esc => {
                            app.input_mode = InputMode::Normal;
                            app.search_query.clear();
                        }
                        KeyCode::Char(c) => {
                            app.search_query.push(c);
                            app.selected = 0;
                        }
                        KeyCode::Backspace => {
                            app.search_query.pop();
                            app.selected = 0;
                        }
                        KeyCode::Down => {
                            if app.selected < filtered_todos(app).len().saturating_sub(1) {
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
                }
            }
        }
    }
}

fn filtered_todos(app: &App) -> Vec<&crate::todo::Todo> {
    if app.search_query.is_empty() {
        app.todos.iter().collect()
    } else {
        let q = app.search_query.to_lowercase();
        app.todos
            .iter()
            .filter(|t| {
                t.description.to_lowercase().contains(&q)
                    || t.due_date
                        .as_ref()
                        .map(|d| d.to_lowercase().contains(&q))
                        .unwrap_or(false)
            })
            .collect()
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
                Constraint::Length(3), // search input
                Constraint::Length(5), // description input
                Constraint::Length(3), // due date input
            ]
            .as_ref(),
        )
        .split(size);

    let title = Paragraph::new(Span::styled(
        "🌟 RustyTodos! 🌟",
        Style::default().add_modifier(Modifier::BOLD),
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
        Span::styled("/", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" to search, "),
        Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" to quit."),
    ]))
    .alignment(Alignment::Center);
    f.render_widget(help, chunks[1]);

    let todos: Vec<ListItem> = filtered_todos(app)
        .iter()
        .map(|t| {
            let status = if t.done { "[x]" } else { "[ ]" };
            let due_date_str = t
                .due_date
                .clone()
                .unwrap_or_else(|| "No due date".to_string());
            let desc_color = if t.done {
                Color::Green
            } else if is_overdue(&due_date_str) {
                Color::Red
            } else {
                Color::Yellow
            };
            let spans = Spans::from(vec![
                Span::raw(format!("{} ", status)),
                Span::styled(&t.description, Style::default().fg(desc_color)),
                Span::raw(format!(
                    " (Due: {}) [Created: {}]",
                    due_date_str, t.created_date
                )),
            ]);
            ListItem::new(spans)
        })
        .collect();

    let mut list_state = ratatui::widgets::ListState::default();
    if !todos.is_empty() {
        list_state.select(Some(app.selected.min(todos.len() - 1)));
    }

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

    // Search input
    let search_style = if matches!(app.input_mode, InputMode::Searching) {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };
    let caret = "|";
    let search_with_caret = if matches!(app.input_mode, InputMode::Searching) {
        format!("Search: {}{}", app.search_query, caret)
    } else if !app.search_query.is_empty() {
        format!("Search: {}", app.search_query)
    } else {
        "".to_string()
    };
    let search_input = Paragraph::new(search_with_caret)
        .block(Block::default().borders(Borders::ALL).title("Search"))
        .style(search_style)
        .wrap(Wrap { trim: true });
    f.render_widget(search_input, chunks[3]);

    // Description and due date input fields (unchanged)
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
                .title("Due Date (flexible format)"),
        )
        .style(due_date_style)
        .wrap(Wrap { trim: true });
    f.render_widget(input_desc, chunks[4]);
    f.render_widget(input_due, chunks[5]);

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

// Check if a due date string represents an overdue task
fn is_overdue(due_date_str: &str) -> bool {
    if due_date_str == "No due date" {
        return false;
    }

    let now = Local::now();

    // Try to parse as datetime first
    if let Ok(dt) = NaiveDateTime::parse_from_str(due_date_str, "%Y-%m-%d %H:%M") {
        if let Some(dt_local) = Local.from_local_datetime(&dt).single() {
            return dt_local < now;
        }
    }

    // Try to parse as date only
    if let Ok(date) = NaiveDate::parse_from_str(due_date_str, "%Y-%m-%d") {
        let today = now.date_naive();
        return date < today;
    }

    false
}

pub fn parse_due_date(input: &str) -> Result<String, String> {
    let input = input.trim().to_lowercase();
    let now = Local::now();
    let today = now.date_naive();

    // Handle empty input
    if input.is_empty() {
        return Err("Please enter a due date".to_string());
    }

    let words: Vec<&str> = input.split_whitespace().collect();

    let raw_date = match words.as_slice() {
        // Immediate times
        ["now"] => Ok(now.format("%Y-%m-%d %H:%M").to_string()),

        // Relative days
        ["today"] => Ok(today.format("%Y-%m-%d").to_string()),
        ["tomorrow"] | ["tmr"] => Ok((today + Dur::days(1)).format("%Y-%m-%d").to_string()),
        ["yesterday"] => Ok((today - Dur::days(1)).format("%Y-%m-%d").to_string()),

        // Day of week (this week or next week)
        [day] if is_weekday(day) => parse_weekday(day, today),

        // Next/this + day
        ["next", day] if is_weekday(day) => parse_next_weekday(day, today),
        ["this", day] if is_weekday(day) => parse_this_weekday(day, today),

        // Relative periods
        ["week"] | ["next", "week"] => Ok((today + Dur::days(7)).format("%Y-%m-%d").to_string()),
        ["month"] | ["next", "month"] => Ok((today + Dur::days(30)).format("%Y-%m-%d").to_string()),
        ["year"] | ["next", "year"] => Ok((today + Dur::days(365)).format("%Y-%m-%d").to_string()),

        // "in X unit" patterns
        ["in", num, unit] => parse_offset(num, unit, &now),

        // "in X unit Y unit" patterns (e.g., "in 1 day 3 hours")
        ["in", num1, unit1, num2, unit2] => parse_compound_offset(num1, unit1, num2, unit2, &now),

        // "X unit" patterns (e.g., "3 days", "2 hours")
        [num, unit] => parse_offset(num, unit, &now),

        // "X unit Y unit" patterns (e.g., "1 day 3 hours")
        [num1, unit1, num2, unit2] => parse_compound_offset(num1, unit1, num2, unit2, &now),

        // Full date or time
        [date_or_time] => try_parse_date_or_time(date_or_time, today, now),

        // Date + Time
        [date_str, time_str] => parse_date_time_combo(date_str, time_str),

        // Weekday + time (e.g., "friday 15:30")
        [day, time] if is_weekday(day) => parse_weekday_time(day, time, today),

        // "next/this weekday time"
        ["next", day, time] if is_weekday(day) => parse_next_weekday_time(day, time, today),
        ["this", day, time] if is_weekday(day) => parse_this_weekday_time(day, time, today),

        _ => Err("Unrecognized due date format".to_string()),
    };

    raw_date.and_then(|date| validate_not_past(&date))
}

fn is_weekday(s: &str) -> bool {
    matches!(
        s,
        "monday"
            | "mon"
            | "tuesday"
            | "tue"
            | "wednesday"
            | "wed"
            | "thursday"
            | "thu"
            | "friday"
            | "fri"
            | "saturday"
            | "sat"
            | "sunday"
            | "sun"
    )
}

fn parse_weekday_name(s: &str) -> Option<Weekday> {
    match s {
        "monday" | "mon" => Some(Weekday::Mon),
        "tuesday" | "tue" => Some(Weekday::Tue),
        "wednesday" | "wed" => Some(Weekday::Wed),
        "thursday" | "thu" => Some(Weekday::Thu),
        "friday" | "fri" => Some(Weekday::Fri),
        "saturday" | "sat" => Some(Weekday::Sat),
        "sunday" | "sun" => Some(Weekday::Sun),
        _ => None,
    }
}

fn parse_weekday(day: &str, today: NaiveDate) -> Result<String, String> {
    let target_weekday = parse_weekday_name(day).ok_or("Invalid weekday")?;

    let days_until = days_until_weekday(today, target_weekday);
    let target_date = today + Dur::days(days_until);

    Ok(target_date.format("%Y-%m-%d").to_string())
}

fn parse_next_weekday(day: &str, today: NaiveDate) -> Result<String, String> {
    let target_weekday = parse_weekday_name(day).ok_or("Invalid weekday")?;

    let days_until = days_until_next_weekday(today, target_weekday);
    let target_date = today + Dur::days(days_until);

    Ok(target_date.format("%Y-%m-%d").to_string())
}

fn parse_this_weekday(day: &str, today: NaiveDate) -> Result<String, String> {
    let target_weekday = parse_weekday_name(day).ok_or("Invalid weekday")?;

    let days_until = days_until_this_week(today, target_weekday);
    let target_date = today + Dur::days(days_until);

    Ok(target_date.format("%Y-%m-%d").to_string())
}

fn parse_weekday_time(day: &str, time: &str, today: NaiveDate) -> Result<String, String> {
    let target_weekday = parse_weekday_name(day).ok_or("Invalid weekday")?;

    let target_time =
        NaiveTime::parse_from_str(time, "%H:%M").map_err(|_| "Invalid time format. Use HH:MM")?;

    let days_until = days_until_weekday(today, target_weekday);
    let target_date = today + Dur::days(days_until);
    let target_datetime = NaiveDateTime::new(target_date, target_time);

    Ok(target_datetime.format("%Y-%m-%d %H:%M").to_string())
}

fn parse_next_weekday_time(day: &str, time: &str, today: NaiveDate) -> Result<String, String> {
    let target_weekday = parse_weekday_name(day).ok_or("Invalid weekday")?;

    let target_time =
        NaiveTime::parse_from_str(time, "%H:%M").map_err(|_| "Invalid time format. Use HH:MM")?;

    let days_until = days_until_next_weekday(today, target_weekday);
    let target_date = today + Dur::days(days_until);
    let target_datetime = NaiveDateTime::new(target_date, target_time);

    Ok(target_datetime.format("%Y-%m-%d %H:%M").to_string())
}

fn parse_this_weekday_time(day: &str, time: &str, today: NaiveDate) -> Result<String, String> {
    let target_weekday = parse_weekday_name(day).ok_or("Invalid weekday")?;

    let target_time =
        NaiveTime::parse_from_str(time, "%H:%M").map_err(|_| "Invalid time format. Use HH:MM")?;

    let days_until = days_until_this_week(today, target_weekday);
    let target_date = today + Dur::days(days_until);
    let target_datetime = NaiveDateTime::new(target_date, target_time);

    Ok(target_datetime.format("%Y-%m-%d %H:%M").to_string())
}

fn days_until_weekday(from: NaiveDate, target: Weekday) -> i64 {
    let current_weekday = from.weekday();
    let days =
        (target.num_days_from_monday() as i64) - (current_weekday.num_days_from_monday() as i64);

    if days <= 0 {
        days + 7 // Next week
    } else {
        days // This week
    }
}

fn days_until_next_weekday(from: NaiveDate, target: Weekday) -> i64 {
    let current_weekday = from.weekday();
    let days =
        (target.num_days_from_monday() as i64) - (current_weekday.num_days_from_monday() as i64);

    if days <= 0 {
        days + 7 // Next week
    } else {
        days + 7 // Force next week
    }
}

fn days_until_this_week(from: NaiveDate, target: Weekday) -> i64 {
    let current_weekday = from.weekday();
    let days =
        (target.num_days_from_monday() as i64) - (current_weekday.num_days_from_monday() as i64);

    if days < 0 {
        0 // If the day has passed this week, return today
    } else {
        days
    }
}

fn parse_duration_component(num_str: &str, unit: &str) -> Result<chrono::TimeDelta, String> {
    let num: i64 = num_str.parse().map_err(|_| "Invalid number")?;

    if num < 0 {
        return Err("Duration cannot be negative".to_string());
    }

    match unit {
        "second" | "seconds" | "sec" | "s" => Ok(Dur::seconds(num)),
        "minute" | "minutes" | "min" | "m" => Ok(Dur::minutes(num)),
        "hour" | "hours" | "hr" | "h" => Ok(Dur::hours(num)),
        "day" | "days" | "d" => Ok(Dur::days(num)),
        "week" | "weeks" | "w" => Ok(Dur::days(num * 7)),
        "month" | "months" => Ok(Dur::days(num * 30)),
        "year" | "years" => Ok(Dur::days(num * 365)),
        _ => Err(format!("Unsupported time unit '{}'", unit)),
    }
}

fn parse_offset(num: &str, unit: &str, now: &chrono::DateTime<Local>) -> Result<String, String> {
    let duration = parse_duration_component(num, unit)?;
    Ok(((*now) + duration).format("%Y-%m-%d %H:%M").to_string())
}

fn parse_compound_offset(
    num1: &str,
    unit1: &str,
    num2: &str,
    unit2: &str,
    now: &chrono::DateTime<Local>,
) -> Result<String, String> {
    let delta1 = parse_duration_component(num1, unit1)?;
    let delta2 = parse_duration_component(num2, unit2)?;
    Ok(((*now) + delta1 + delta2)
        .format("%Y-%m-%d %H:%M")
        .to_string())
}

fn try_parse_date_or_time(
    input: &str,
    today: NaiveDate,
    now: chrono::DateTime<Local>,
) -> Result<String, String> {
    // Try full date (YYYY-MM-DD)
    if let Ok(date) = NaiveDate::parse_from_str(input, "%Y-%m-%d") {
        return Ok(date.format("%Y-%m-%d").to_string());
    }

    // Try date without year (MM-DD)
    if let Ok(parsed) =
        NaiveDate::parse_from_str(&format!("{}-{}", today.year(), input), "%Y-%m-%d")
    {
        return Ok(parsed.format("%Y-%m-%d").to_string());
    }

    // Try time for today (HH:MM)
    if let Ok(time) = NaiveTime::parse_from_str(input, "%H:%M") {
        let dt = NaiveDateTime::new(today, time);
        return Ok(dt.format("%Y-%m-%d %H:%M").to_string());
    }

    // Try 12-hour format (HH:MM AM/PM)
    if input.ends_with("am") || input.ends_with("pm") {
        let is_pm = input.ends_with("pm");
        let time_part = input.trim_end_matches("am").trim_end_matches("pm").trim();

        if let Ok(mut time) = NaiveTime::parse_from_str(time_part, "%H:%M") {
            if is_pm && time.hour() < 12 {
                time = time + Dur::hours(12);
            } else if !is_pm && time.hour() == 12 {
                time = time - Dur::hours(12);
            }
            let dt = NaiveDateTime::new(today, time);
            return Ok(dt.format("%Y-%m-%d %H:%M").to_string());
        }
    }

    Err("Invalid date or time format".to_string())
}

fn parse_date_time_combo(date_str: &str, time_str: &str) -> Result<String, String> {
    let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
        .map_err(|_| "Invalid date format. Use YYYY-MM-DD")?;

    let time = NaiveTime::parse_from_str(time_str, "%H:%M")
        .map_err(|_| "Invalid time format. Use HH:MM")?;

    let datetime = NaiveDateTime::new(date, time);
    Ok(datetime.format("%Y-%m-%d %H:%M").to_string())
}

fn validate_not_past(s: &str) -> Result<String, String> {
    let now = Local::now();

    // Try to parse as date+time first
    if let Ok(dt) = NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M") {
        let dt_local = Local
            .from_local_datetime(&dt)
            .single()
            .ok_or("Failed to convert due date to local time")?;

        if dt_local < now {
            return Err("Due date cannot be in the past".to_string());
        }
        return Ok(s.to_string());
    }

    // Try to parse as date only
    if let Ok(date) = NaiveDate::parse_from_str(s, "%Y-%m-%d") {
        let today = now.date_naive();
        if date < today {
            return Err("Due date cannot be in the past".to_string());
        }
        return Ok(s.to_string());
    }

    Err("Failed to parse due date".to_string())
}
