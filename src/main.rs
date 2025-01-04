use colored::*;
use dirs::config_dir;
use serde::{Deserialize, Serialize};
use std::fs::{self, create_dir_all};
use std::io::{self, Write};
use std::path::PathBuf;

const APP_NAME: &str = "RustyTasks";

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: usize,
    description: String,
    completed: bool,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct TodoList {
    tasks: Vec<Task>,
}

impl TodoList {
    fn new() -> Self {
        TodoList { tasks: Vec::new() }
    }

    fn get_file_path() -> PathBuf {
        let mut path = config_dir().expect("Unable to find config directory");
        path.push(APP_NAME);
        create_dir_all(&path).expect("Failed to create app config directory");
        path.push("todo.json");
        path
    }

    fn load() -> Self {
        let path = TodoList::get_file_path();
        if path.exists() {
            let data = fs::read_to_string(&path).expect("Failed to read file");
            serde_json::from_str(&data).expect("Failed to deserialize data")
        } else {
            TodoList::new()
        }
    }

    fn save(&self) {
        let path = TodoList::get_file_path();
        let data = serde_json::to_string_pretty(&self).expect("Failed to serialize data");
        fs::write(path, data).expect("Failed to write to file");
    }

    fn add_task(&mut self, description: String) {
        let id = self.tasks.len() + 1;
        self.tasks.push(Task {
            id,
            description,
            completed: false,
        });
        println!("{}", "Task added successfully!".green());
    }

    fn view_tasks(&self) {
        if self.tasks.is_empty() {
            println!("{}", "No tasks available!".yellow());
            return;
        }

        println!("{}", "Your To-Do List:".bold().blue());
        for task in &self.tasks {
            let status = if task.completed {
                "[x]".green()
            } else {
                "[ ]".red()
            };
            println!(
                "{} {}: {}",
                status,
                task.id.to_string().bold().cyan(),
                task.description
            );
        }
    }

    fn mark_completed(&mut self, id: usize) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.completed = true;
            println!("{}", "Task marked as completed!".green());
        } else {
            println!("{}", format!("Task with ID {} not found!", id).bold().red());
        }
    }
}

fn main() {
    let mut todo_list = TodoList::load();

    loop {
        println!("\n{}", "=== To-Do List ===".bold().blue());
        println!("1. {}", "Add Task".bold().green());
        println!("2. {}", "View Tasks".bold().yellow());
        println!("3. {}", "Mark Task Completed".bold().cyan());
        println!("4. {}", "Exit".bold().red());
        print!("{}", "Choose an option: ".bold().white());
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");

        match choice.trim() {
            "1" => {
                print!("{}", "Enter task description: ".bold().white());
                io::stdout().flush().unwrap();
                let mut description = String::new();
                io::stdin()
                    .read_line(&mut description)
                    .expect("Failed to read input");
                todo_list.add_task(description.trim().to_string());
                todo_list.save();
            }
            "2" => todo_list.view_tasks(),
            "3" => {
                print!("{}", "Enter task ID to mark as completed: ".bold().white());
                io::stdout().flush().unwrap();
                let mut id = String::new();
                io::stdin()
                    .read_line(&mut id)
                    .expect("Failed to read input");
                if let Ok(id) = id.trim().parse::<usize>() {
                    todo_list.mark_completed(id);
                    todo_list.save();
                } else {
                    println!("{}", "Invalid ID!".bold().red());
                }
            }
            "4" => {
                println!("{}", "Exiting... Goodbye!".bold().green());
                break;
            }
            _ => println!("{}", "Invalid option! Try again.".bold().red()),
        }
    }
}
