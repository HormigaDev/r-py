use clap::Parser;
use std::fs;
use std::process::Command;

#[derive(Parser)]
#[clap(
    version = "0.1.3",
    author = "HormigaDev <hormigadev7@gmail.com>",
    about = "Una herramienta CLI para gestionar y ejecutar comandos predefinidos desde archivos de configuración"
)]
struct Cli {
    key: String,
}

fn main() {
    let args = Cli::parse();
    let cmd_file = "cmd.ini";
    let content = fs::read_to_string(cmd_file).expect("Cannot read file cmd.ini");

    let commands: std::collections::HashMap<_, _> = content
        .lines()
        .filter_map(|line| {
            let mut parts = line.splitn(2, "=");
            Some((
                parts.next()?.trim().to_string(),
                parts.next()?.trim().to_string(),
            ))
        })
        .collect();

    if let Some(command) = commands.get(&args.key) {
        println!("Executing: {}", command);

        let mut cmd_parts = command.split_whitespace();
        let program = cmd_parts.next().expect("Command is void!");
        let args: Vec<&str> = cmd_parts.collect();

        let status = Command::new(program)
            .args(&args)
            .status()
            .expect("Error executing command!");

        if status.success() {
            println!("Command is executed sucessfully!");
        } else {
            eprintln!("Command fail!");
        }
    } else {
        eprintln!("Key '{}' is not present in the file .cmd", args.key);
    }
}
