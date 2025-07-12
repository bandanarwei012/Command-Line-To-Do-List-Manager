use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::io::{self, Write};

// Define the structure for a single To-Do item.
// The `#[derive(...)]` attribute automatically implements traits for our struct.
// - `Serialize`, `Deserialize`: Required by `serde` to convert this struct to/from JSON.
// - `Clone`: Allows us to create copies of a Todo item.
// - `Debug`: Allows us to print the struct for debugging purposes using `{:?}`.
#[derive(Serialize, Deserialize, Clone, Debug)]
struct Todo {
    task: String,
    completed: bool,
}

// Define the file path where the to-do list will be stored.
const DB_PATH: &str = "todos.json";

fn main() -> io::Result<()> {
    // Collect command-line arguments into a vector of strings.
    let args: Vec<String> = env::args().collect();

    // The first argument is the program name, so we need at least one more for a command.
    if args.len() < 2 {
        print_help();
        return Ok(());
    }

    // Match the command provided by the user (the second argument).
    let command = &args[1];
    match command.as_str() {
        "add" => add_task(&args)?,
        "list" => list_tasks()?,
        "done" => complete_task(&args)?,
        "help" => print_help(),
        _ => {
            println!("Error: Unknown command '{}'", command);
            print_help();
        }
    }

    Ok(())
}

/// Adds a new task to the list.
fn add_task(args: &[String]) -> io::Result<()> {
    if args.len() < 3 {
        println!("Error: Missing task description for 'add' command.");
        println!("Example: todo_cli add \"Buy milk\"");
        return Ok(());
    }
    let task_description = args[2..].join(" ");

    let mut todos = load_todos()?;

    let new_todo = Todo {
        task: task_description,
        completed: false,
    };

    println!("Adding task: {}", new_todo.task);
    todos.push(new_todo);
    save_todos(&todos)
}

/// Lists all current tasks, showing their status.
fn list_tasks() -> io::Result<()> {
    let todos = load_todos()?;

    if todos.is_empty() {
        println!("No tasks yet! Add one with the 'add' command.");
    } else {
        println!("--- To-Do List ---");
        for (i, todo) in todos.iter().enumerate() {
            let status = if todo.completed { "[x]" } else { "[ ]" };
            println!("{} {}. {}", status, i + 1, todo.task);
        }
        println!("------------------");
    }
    Ok(())
}

/// Marks a task as complete by its number.
fn complete_task(args: &[String]) -> io::Result<()> {
    if args.len() < 3 {
        println!("Error: Missing task number for 'done' command.");
        println!("Example: todo_cli done 2");
        return Ok(());
    }

    let task_number_str = &args[2];
    let task_number: usize = match task_number_str.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: '{}' is not a valid number.", task_number_str);
            return Ok(());
        }
    };

    if task_number == 0 {
        println!("Error: Task number must be 1 or greater.");
        return Ok(());
    }

    let mut todos = load_todos()?;
    let task_index = task_number - 1;

    if let Some(todo) = todos.get_mut(task_index) {
        if todo.completed {
            println!("Task {} was already completed.", task_number);
        } else {
            todo.completed = true;
            println!("Completed task {}: {}", task_number, todo.task);
            save_todos(&todos)?;
        }
    } else {
        println!("Error: No task found with number {}.", task_number);
    }

    Ok(())
}

/// Loads the list of todos from the JSON file.
fn load_todos() -> io::Result<Vec<Todo>> {
    // Try to read the file. If it doesn't exist, that's okay, just return an empty list.
    match fs::read_to_string(DB_PATH) {
        Ok(data) => {
            // If we read data, try to parse it as JSON into our Vec<Todo>.
            let todos = serde_json::from_str(&data)
                .expect("Failed to parse todos.json. The file might be corrupted.");
            Ok(todos)
        }
        Err(ref e) if e.kind() == io::ErrorKind::NotFound => {
            // If the file doesn't exist, return a new, empty vector.
            Ok(Vec::new())
        }
        Err(e) => {
            // For any other read error, propagate it up.
            Err(e)
        }
    }
}

/// Saves the current list of todos to the JSON file.
fn save_todos(todos: &[Todo]) -> io::Result<()> {
    // Serialize the `todos` vector into a nicely formatted JSON string.
    let json_data = serde_json::to_string_pretty(todos)
        .expect("Failed to serialize data to JSON.");
    
    // Write the JSON string to our file.
    fs::write(DB_PATH, json_data)
}

/// Prints the help message showing available commands.
fn print_help() {
    println!("\nRust To-Do List Manager");
    println!("Usage: todo_cli <COMMAND> [ARGUMENTS]");
    println!("\nCommands:");
    println!("  add \"<task>\"   - Adds a new task to the list.");
    println!("  list           - Lists all tasks.");
    println!("  done <number>  - Marks a task as complete.");
    println!("  help           - Shows this help message.");
}
