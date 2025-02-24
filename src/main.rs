use clap::{Parser, Subcommand};
use std::collections::HashMap;
use std::fs;
use std::process::Command;

mod commands;

#[derive(Parser)]
#[clap(
    version = "0.1.8",
    author = "HormigaDev <hormigadev7@gmail.com>",
    about = "A CLI tool for managing and executing predefined commands from configuration files"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,

    #[clap(name = "command")]
    extra_commands: Vec<String>,
}

#[derive(Subcommand)]
enum Commands {
    #[clap(
        about = "Installs packages using pip. If no packages are specified, it installs from requirements.txt."
    )]
    Install {
        #[clap(required = false)]
        packages: Option<Vec<String>>,
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
        Some(Commands::Install { packages }) => {
            commands::install(packages);
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
                    let mut in_commands_section = false;
                    let mut commands: HashMap<String, String> = HashMap::new();

                    for line in content.lines() {
                        if line.trim() == "[commands]" {
                            in_commands_section = true;
                        } else if in_commands_section && line.starts_with('[') {
                            in_commands_section = false;
                        }

                        if in_commands_section && line.contains('=') {
                            let mut parts = line.splitn(2, "=");
                            if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
                                commands.insert(key.trim().to_string(), value.trim().to_string());
                            }
                        }
                    }

                    let command_key = args.extra_commands.get(0); // Obtenemos el primer argumento adicional

                    if let Some(command_key) = command_key {
                        if let Some(command) = commands.get(command_key) {
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
