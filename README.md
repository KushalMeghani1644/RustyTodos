# RustyTodos™ 🦀 - A Modern TUI Todo App

RustyTodos™ is a fast, lightweight, and interactive terminal Todo app built in Rust.  
Manage your tasks visually with an intuitive TUI—add, list, mark as done, and delete todos—right from your terminal!
 
## Trademark Notice
The names “Rusty-Suite”, “RustyTodos”, “RustyBoot”, and “Rusty-Checker” are part of this project’s identity.  
See [TRADEMARK.md](TRADEMARK.md) for details.

**RustyTodos™** is a fast, lightweight, and interactive terminal Todo app built in Rust.  
Manage your tasks visually with an intuitive TUI—add, list, mark as done, and delete todos—right from your terminal!

![DEMO FOR MY APP](https://raw.githubusercontent.com/KushalMeghani1644/RustyTodos/0bed35d4fc2228ed3571e4c06aeb2edfda36e5f9/media/RustyTodos_demo.gif)

## Features

- [x] Interactive terminal UI (TUI)  
- [x] Add new todos with description & due date  
- [x] **Smart natural language date parsing**  
- [x] **Color-coded task status (overdue, pending, completed)**  
- [x] View todos in a scrollable list  
- [x] Mark todos as done  
- [x] Delete todos by ID  
- [x] Persistent storage in a JSON file  
- [x] Cross-platform binaries (Linux & Windows)  
- [x] Pre-built binaries for easy setup
- [x] Cross platform notification (Linux & Windows)
- [x] Search option to search for your todos!

---

## Download

Get the latest release from the [Releases page](https://github.com/KushalMeghani1644/RustyTodos/releases).

| Platform   | Download                                              |
|------------|-------------------------------------------------------|
<<<<<<< HEAD
| Linux      | `rustytodos-v2.1.3.tar.gz`                      |
| Windows    | `rustytodos-v2.1.3(windows).zip`                       |
=======
| Linux      | `rustytodos-v2.1.1.tar.gz`                      |
| Windows    | `rustytodos-v2.1.1(windows).zip`                       |

---

## How to Use

### Run Pre-Built Binary

1. Download the release for your platform.  
2. Extract the archive:
```bash
# For Linux
<<<<<<< HEAD
tar -xzf rustytodos-v2.1.3-linux.tar.gz
=======
tar -xzf rustytodos-v2.1.1-linux.tar.gz
>>>>>>> c931762491b113cfb94339e1ddbefe0f3d1ea14e

# For Windows
# Extract using your preferred archive manager (e.g., 7-Zip)

# Run
./rustytodos    # Linux
rustytodos.exe  # Windows
```

---

## TUI Interaction Guide

- **Add Todo**: Press `a`, enter description, press `Enter`, enter due date, press `Enter`
- **Navigate Todos**: Use arrow keys ↑↓ to navigate through your task list
- **Mark as Done**: Select a todo and press `m`
- **Delete Todo**: Select a todo and press `d`
- **Quit**: Press `q`

---

## Smart Date & Time Parsing

RustyTodos features an intelligent date parser that understands natural language! No need to remember complex date formats—just type what feels natural.

### Task Status Colors
- **Green**: Completed tasks (regardless of due date)
- **Red**: Overdue tasks (not completed and past due date)
- **Yellow**: Pending tasks (not completed but not overdue)

### Supported Date Formats

#### **Relative Times**
```
now                    # Right now with current time
today                  # Today (date only)
tomorrow, tmr          # Tomorrow
yesterday              # Yesterday
```

#### **Weekdays**
```
friday                 # Next Friday
monday                 # Next Monday
next friday            # Explicitly next Friday
this wednesday         # This Wednesday (if not passed)
```

#### **Weekdays with Time**
```
friday 15:30           # Next Friday at 3:30 PM
next monday 09:00      # Next Monday at 9:00 AM
this thursday 14:45    # This Thursday at 2:45 PM
```

#### **Time Offsets**
```
in 30 minutes          # 30 minutes from now
in 2 hours             # 2 hours from now
in 3 days              # 3 days from now
5 minutes              # 5 minutes from now (without "in")
2 hours 30 minutes     # Combined time periods
in 1 day 3 hours       # 1 day and 3 hours from now
```

#### **Specific Dates & Times**
```
2024-12-25             # Christmas Day 2024
12-25                  # December 25th (current year)
15:30                  # Today at 3:30 PM
9:00am                 # Today at 9:00 AM
11:45pm                # Today at 11:45 PM
2024-12-25 15:30       # Christmas Day 2024 at 3:30 PM
```

#### **Relative Periods**
```
week, next week        # Next week (7 days from now)
month, next month      # Next month (30 days from now)
year, next year        # Next year (365 days from now)
```

### Supported Time Units
- **Seconds**: `second`, `seconds`, `sec`, `s`
- **Minutes**: `minute`, `minutes`, `min`, `m`
- **Hours**: `hour`, `hours`, `hr`, `h`
- **Days**: `day`, `days`, `d`
- **Weeks**: `week`, `weeks`, `w`
- **Months**: `month`, `months` (30 days)
- **Years**: `year`, `years` (365 days)

### Examples in Action
```
# Quick tasks
"Call mom" → "in 2 hours"
"Weekly standup" → "monday 10:00am"
"Doctor appointment" → "friday 14:30"

# Project deadlines
"Submit report" → "next friday"
"Code review" → "in 3 days"
"Meeting prep" → "tomorrow 09:00"

# Long-term goals
"Vacation planning" → "next month"
"Annual review" → "2024-12-15"
```

---

## Build from Source

### Prerequisites
- Rust
- Git

### Steps
```bash
# Clone the repository
git clone https://github.com/Kushal_Meghani1644/RustyTodos.git
cd rustytodos

# Build and run
cargo run

# Or build release binary
cargo build --release
# Binary will be in target/release/rustytodos
```

---

## Configuration

RustyTodos stores your todos in a JSON file:
- **Linux/macOS**: `~/.local/share/rustytodos/todos.json`
- **Windows**: `%APPDATA%/rustytodos/todos.json`

The file is created automatically on first run.

---

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

---

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

**Built with ❤️ in Rust** 🦀

**Shout-out to [Kivooeo](https://github.com/Kivooeo) for contributing to the code!**
