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

fn remove_task(title:String , tasks:&mut Vec<Task>){
    for i in 0..tasks.len(){
        if tasks[i].title == title {
            tasks.remove(i);
            println!("Task '{}' removed successfully!", title);
            return; 
        }
    }
}

fn main() {
    let result = command!()
        .arg(arg!(-t --title [TITLE] "This is the title of the task"))
        .arg(arg!(-d --description [DESCRIPTION] "This is the discription of the task")).arg(arg!(-r --remove [REMOVE] "This is to remove a task"))
        .get_matches();

    let title = match result.get_one::<String>("title"){
        Some(t)=>t.to_string(),
        None=>String::from("No title provided")
    };
    
    let description = match result.get_one::<String>("description"){
        Some(desc)=> desc.to_string(),
        None => String::from("No description provided"),
    };
    let remove = match result.get_one::<String>("remove"){
        Some(r)=>r.to_string(),
        None=>String::from("No task to remove")
    };

    let mut tasks = load_task();

    if tasks.len() == 0 && remove != "No task to remove"{
        println!("Error: No tasks available to remove.");
        return;
    }else{
        if remove != "No task to remove" {
            remove_task(remove,&mut tasks);
        }
    }

    tasks.push(Task {
        title:title.to_string(),
        description:description.to_string()
    });

    save_task(tasks);
    println!("Task saved successfully!");

    


}


