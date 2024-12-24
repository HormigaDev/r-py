use clap::Parser;
use std::fs;
use std::process::Command;

#[derive(Parser)]
#[clap(
    version = "0.1.4",
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

        // Detectamos el sistema operativo
        let shell = if cfg!(target_os = "windows") {
            "cmd"
        } else {
            "sh"
        };

        // Dependiendo del SO, el comando es diferente
        let shell_arg = if cfg!(target_os = "windows") {
            "/C"
        } else {
            "-c"
        };

        let status = Command::new(shell)
            .arg(shell_arg)
            .arg(command)
            .status()
            .expect("Error executing command!");

        if status.success() {
            println!("Command executed successfully!");
        } else {
            eprintln!("Command failed!");
        }
    } else {
        eprintln!("Key '{}' is not present in the file .cmd", args.key);
    }
}
