use clap::{Parser, Subcommand};
use std::collections::HashMap;
use std::fs;
use std::process::Command;

mod commands;

#[derive(Parser)]
#[clap(
    version = "0.1.5",
    author = "HormigaDev <hormigadev7@gmail.com>",
    about = "A CLI tool for managing and executing predefined commands from configuration files"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[clap(
        about = "Installs packages using pip. If no package is specified, it installs from requirements.txt."
    )]
    Install {
        #[clap(required = false)]
        package: Option<String>,
    },

    #[clap(
        about = "Initializes a project with basic files like project.properties, requirements.txt, and main.py."
    )]
    Init,

    #[clap(
        about = "Creates a new project with the specified name. It checks if the directory exists and creates necessary files."
    )]
    New {
        #[clap(required = true)]
        name: String,
    },
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Some(Commands::Install { package }) => {
            commands::install(package);
        }
        Some(Commands::Init) => {
            commands::initialize_project();
        }
        Some(Commands::New { name }) => {
            commands::new_project(name);
        }
        None => {
            let cmd_file = "project.properties";
            match fs::read_to_string(cmd_file) {
                Ok(content) => {
                    let commands: HashMap<_, _> = content
                        .lines()
                        .filter_map(|line| {
                            let mut parts = line.splitn(2, "=");
                            Some((
                                parts.next()?.trim().to_string(),
                                parts.next()?.trim().to_string(),
                            ))
                        })
                        .collect();

                    let command_key = std::env::args().nth(1);
                    if let Some(command_key) = command_key {
                        if let Some(command) = commands.get(&command_key) {
                            println!("Executing: {}", command);

                            let status = Command::new("sh")
                                .arg("-c")
                                .arg(command)
                                .status()
                                .expect("Error executing command");

                            if status.success() {
                                println!("Command executed successfully!");
                            } else {
                                eprintln!("Command failed!");
                            }
                        } else {
                            eprintln!("Command '{}' not found in project.properties.", command_key);
                        }
                    } else {
                        eprintln!("No command specified.");
                    }
                }
                Err(_) => {
                    eprintln!("Failed to read project.properties.");
                }
            }
        }
    }
}
