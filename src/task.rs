
use colored::Colorize;


#[derive(Debug)]
pub struct Task {
    pub id: i32,
    pub name: String,
    pub completed: bool,
    pub description: String,
}



impl Task {

    pub fn colored_display(&self) {
        let status = if self.completed {
            "Completed".green()
        } else {
            "Incomplete".red()
        };
        println!("Task #{}:", self.id.to_string().cyan());
        println!("Name: {}", self.name.yellow());
        println!("Status: {}", status);
        println!("Description: {}", self.description);
        println!();
    }
}