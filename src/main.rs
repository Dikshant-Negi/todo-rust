use clap::{arg, command};
use std::fs;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
struct Task {
    title: String,
    description: String,
}

const FILE : &str = "tasks.json";

fn load_task()->Vec<Task>{
    // here we check if the file exists, if it does we read the content and returning empty vector if it fails.
    let data = match fs::read_to_string(FILE){
        Ok(content)=>content,
        Err(_)=>String::from("[]")
    };

    //parse it using serde_json
    match serde_json::from_str::<Vec<Task>>(&data) {
        Ok(tasks) => tasks,  
        Err(_) => Vec::new(), 
    }
}

fn save_task(tasks: Vec<Task>){
    let data = serde_json::to_string(&tasks).expect("Failed to serialize tasks");
    fs::write(FILE,data).expect("Failed to write tasks to file");
}

fn main() {
    let result = command!()
        .arg(arg!(-t --title <TITLE> "This is the title of the task"))
        .arg(arg!(-d --description <DESCRIPTION> "This is the discription of the task"))
        .get_matches();

    let title = result.get_one::<String>("title").expect("Title is required");
    let description = match result.get_one::<String>("description"){
        Some(desc)=> desc.to_string(),
        None => String::from("No description provided"),
    };

    let mut tasks = load_task();

    tasks.push(Task {
        title:title.to_string(),
        description:description.to_string()
    });

    save_task(tasks);
    println!("Task saved successfully!");

    


}


