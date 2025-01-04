# RustyTasks

RustyTasks is a simple CLI-based to-do list manager built in Rust. It helps you keep track of your tasks, mark them as completed, and save them for future sessions.

## Features

- Add tasks with descriptions.
- View your tasks, categorized by their completion status.
- Mark tasks as completed.
- Persistent storage using JSON files in the user configuration directory.

## Installation

1. Clone the repository:
   ````bash
    git clone https://github.com/your-username/rustytasks.git
    cd rustytasks```
   ````
2. Build the project:

   ````bash
   cargo build --release```

   ````

3. Run the executable:
   ````bash
   ./target/release/rustytasks```
   ````

## Usage

1. Launch the program.
2. Use the menu options to:
   - Add a new task.
   - View your task list.
   - Mark tasks as completed.
   - Exit the program

## Data Storage

RustyTasks stores your to-do list in a JSON file located in: `~/.config/RustyTasks/todo.json`
