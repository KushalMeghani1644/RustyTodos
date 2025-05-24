use std::io::{self, Write};
#[derive(Debug)]
struct Todo {
    id: u32,
    description: String,
    done: bool,
}

fn get_new_id(todos: &Vec<Todo>) -> u32 {
    let mut id = 1;
    loop {
        if !todos.iter().any(|t| t.id == id) {
            return id;
        }
        id += 1;
    }
}

fn main() {
    let mut todos: Vec<Todo> = Vec::new();
    let mut next_id = 1;

    loop {
        println!("\n=== Todo List ===");
        println!("1. Add Todo");
        println!("2. List Todo(s)");
        println!("3. Mark Todo as Done");
        println!("4. Delete Todo");
        println!("5. Quit");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "1" => {
                print!("Enter todo description: ");
                io::stdout().flush().unwrap();
                let mut desc = String::new();
                io::stdin().read_line(&mut desc).unwrap();
                let desc = desc.trim().to_string();
                let id = get_new_id(&todos);

                todos.push(Todo {
                    id,
                    description: desc,
                    done: false,
                });
            }
            "2" => {
                if todos.is_empty() {
                    println!("No todos yet!");
                } else {
                    println!("\nCurrent Todos:");
                    for todo in &todos {
                        println!(
                            "{}. [{}] {}",
                            todo.id,
                            if todo.done { "x" } else { " " },
                            todo.description
                        );
                    }
                }
            }
            "3" => {
                print!("Enter id to mark an todo as done: ");
                io::stdout().flush().unwrap();
                let mut id_str = String::new();
                io::stdin().read_line(&mut id_str).unwrap();
                let id: u32 = match id_str.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid id!");
                        continue;
                    }
                };

                let mut found = false;
                for todo in &mut todos {
                    if todo.id == id {
                        todo.done = true;
                        println!("Marked todo {} as done", id);
                        found = true;
                        break;
                    }
                }
                if !found {
                    println!("Todo with id {} not found", id);
                }
            }
            "4" => {
                print!("Enter id to remove: ");
                io::stdout().flush().unwrap();
                let mut id_str = String::new();
                io::stdin().read_line(&mut id_str).unwrap();
                let id: u32 = match id_str.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid id!");
                        continue;
                    }
                };

                let orig_len = todos.len();
                todos.retain(|todo| todo.id != id);
                if todos.len() < orig_len {
                    println!("Removed todo {}", id);
                } else {
                    println!("Todo with id {} not found", id);
                }
            }
            "5" => {
                println!("Goodbye");
                break;
            }
            _ => {
                println!("Invalid command!");
            }
        }
    }
}
