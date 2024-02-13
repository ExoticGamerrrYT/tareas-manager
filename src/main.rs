/*

MADE BY: EXOTICGAMERRRYT / EOXTICGAMERRR / EXOTIC DEV
                                : )
*/
use std::fs::File;
use std::io;
use std::io::Read;
use serde::{Deserialize, Serialize};
use dirs;
use colored::Colorize;

#[derive(Serialize, Deserialize)]
struct Task {
    description: String,
    amount: f64,
}
fn main() {
    let mut tasks: Vec<Task> = read_tasks();

    loop {
        println!("{}", "1. Add task".truecolor(255, 102, 255));
        println!("{}", "2. Show total amount".truecolor(255, 102, 255));
        println!("{}", "3. Clear data".truecolor(255, 51, 51));
        println!("{}", "4. Exit".truecolor(255, 0, 102));

        let mut option = String::new();
        io::stdin().read_line(&mut option).expect("Error reading input");

        match option.trim() {
            "1" => {
                let task = create_task();
                tasks.push(task);
                save_tasks(&tasks);
            }
            "2" => {
                let total_amount: f64 = tasks.iter().map(|t| t.amount).sum();
                println!("Total accumulated amount: {:.2} €\n", total_amount);
            }
            "3" => {
                tasks.clear();
                save_tasks(&tasks);
                println!("Data cleared\n");
            }
            "4" => {
                println!("Exiting the program");
                break;
            }
            _ => println!("Invalid option\n"),
        }
    }
}

fn create_task() -> Task {
    println!("Enter task description:");
    let mut description = String::new();
    io::stdin().read_line(&mut description).expect("Error reading input");

    println!("Enter task amount:");
    let mut amount_str = String::new();
    io::stdin().read_line(&mut amount_str).expect("Error reading input");
    let amount: f64 = amount_str.trim().parse().expect("Error converting amount");
    println!("\n"); // This is a line break
    Task {
        description: description.trim().to_string(),
        amount,
    }
}

fn read_tasks() -> Vec<Task> {
    let local_app_data = dirs::data_dir().expect("Failed to get local app data directory");
    let tasks_path = local_app_data.join("Ayudante de Tareas").join("tasks.json");

    match File::open(&tasks_path) {
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .expect("Error reading file");

            serde_json::from_str(&contents).unwrap_or_default()
        }
        Err(_) => Vec::new(),
    }
}

fn save_tasks(tasks: &Vec<Task>) {
    let local_app_data = dirs::data_dir().expect("Failed to get local app data directory");
    let tasks_path = local_app_data.join("Ayudante de Tareas").join("tasks.json");

    // Create the directory if it doesn't exist
    if let Some(parent) = tasks_path.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent).expect("Error creating directory");
        }
    }

    let file = File::create(&tasks_path).expect("Error creating file");

    serde_json::to_writer(file, tasks).expect("Error writing to file");
}
