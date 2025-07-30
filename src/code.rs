use clap::{arg, command};
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    title: String,
    description: String,
}

const FILE: &str = "tasks.json";

/// Load tasks from the file, or return an empty Vec if file not found
fn load_tasks() -> Vec<Task> {
    if Path::new(FILE).exists() {
        let data = fs::read_to_string(FILE).unwrap_or_else(|_| "[]".to_string());
        serde_json::from_str(&data).unwrap_or_else(|_| Vec::new())
    } else {
        Vec::new()
    }
}

/// Save tasks into the file
fn save_tasks(tasks: &Vec<Task>) {
    let data = serde_json::to_string_pretty(tasks).unwrap();
    fs::write(FILE, data).expect("Unable to save tasks");
}

fn main() {
    let matches = command!()
        .arg(arg!(-t --title <TITLE> "Title of the task"))
        .arg(arg!(-d --description <DESCRIPTION> "Description of the task"))
        .get_matches();

    let title = matches.get_one::<String>("title").unwrap();
    let description = matches.get_one::<String>("description").unwrap();

    let mut tasks = load_tasks();
    tasks.push(Task {
        title: title.to_string(),
        description: description.to_string(),
    });

    save_tasks(&tasks);

    println!("Task saved successfully!");
}
