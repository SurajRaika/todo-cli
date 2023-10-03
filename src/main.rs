use colored::*;

struct Tasks {
    data: Vec<String>,
}

impl Tasks { 
    fn show_tasks(&self) {
        if self.data.is_empty() {
            println!("{}", "No tasks available.".yellow());
        } else {
            println!("{}", "Tasks:".green());
            for (index, task) in self.data.iter().enumerate() {
                println!("{}: {}", index, task);
            }
        }
    }

    fn remove_task(&mut self, id: usize) {
        if id < self.data.len() {
            let removed_task = self.data.remove(id);
            println!("Removed task: {}", removed_task.green());
        } else {
            println!("{} {}", "Invalid index:".red(), id);
        }
    }

    fn add_task(&mut self, task: String) {
        self.data.push(task);
        println!("{} {}", "Added task:".green(), self.data.last().unwrap_or(&String::from("")));
    }
}

fn line_jump() {
    println!();
}

fn user_input(message: &str) -> String {
    println!("{}", message.blue());
    let mut input: String = String::new();
    loop {
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
                println!();
                let lowerform = input.to_lowercase();
                return lowerform.trim().to_string();
            }
            Err(_) => {
                println!("{}", "Retry please".red());
            }
        }
    }
}

fn main() {
    line_jump();
    println!("{}", "TODO APP".green());
    line_jump();
    let mut task = Tasks { data: vec![] };

    loop {
        let command = user_input("Select option: add / remove / show / exit");

        match command.as_str() {
            "add" => {
                let new_task = user_input("Enter Task:");
                task.add_task(new_task);
            }
            "remove" => {
                let remove_task_index = user_input("Enter Index of Task You Want to Remove:");
                if let Ok(index) = remove_task_index.parse::<usize>() {
                    task.remove_task(index);
                } else {
                    println!("{} {}", "Invalid index:".red(), remove_task_index);
                }
            }
            "show" => {
                task.show_tasks();
            }
            "exit" => {
                return;
            }
            _ => {
                println!("{} {}", "Invalid Command:".red(), command);
            }
        }
    }
}