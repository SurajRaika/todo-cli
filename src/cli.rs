// src/cli.rs
use std::path::PathBuf;
use clap::{arg, command, value_parser, ArgAction, Command, ArgMatches};
use crate::task::Task;

use crate::localStorage;

pub fn run_cli() {
    let matches = create_cli().get_matches();
    if let Some(category) = matches.get_one::<String>("category") {
        println!("Value for category: {category}");
    }

    if let Some(matches) = matches.subcommand_matches("add") {
        if let Some(title) = matches.get_one::<String>("title") {
            if let Some(discription) = matches.get_one::<String>("discription") {
            let task=Task{
                id:0,
                name:title.clone(),
                completed:false,
                description:discription.clone()
            };
            localStorage::add_todo_list(task);
            }
        }
    }

    if let Some(matches) = matches.subcommand_matches("show") {
        localStorage::extract_todo_list();

    }

    // Rest of your CLI logic remains here...
}

pub fn create_cli() -> Command {
    command!()
        .arg(arg!([category] "Optional name to operate on"))
        .subcommand(
            Command::new("test")
                .about("does testing things")
                .arg(arg!(-l --list "lists test values").action(ArgAction::SetTrue)),
        )
        .subcommand(
            Command::new("add")
                .about("add new task")
                .arg(
                    arg!([title]"title of task")
                    .required(true)
                ).arg(
                    arg!([discription]"discription of task")
                    .required(true)
                )
        )
        .subcommand(
            Command::new("remove")
                .about("remove task ")
                .arg(
                    arg!([title]"title of task")
                    .required(true)
                ).arg(
                    arg!([discription]"discription of task")
                    .required(true)
                )
        )
        .subcommand(
            Command::new("show")
                .about("show all task ")
        )

}
    