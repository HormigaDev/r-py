use std::fs::{self, File};
use std::io::Write;
use std::process::Command;

pub fn initialize_project() {
    println!("Initializing project in the current directory...");

    create_files();

    init_git_repo();
}

fn create_files() {
    let template_content = include_str!("../project.properties.template");

    if fs::metadata("project.properties").is_err() {
        let mut project_properties =
            File::create("project.properties").expect("Failed to create project.properties");

        project_properties
            .write_all(template_content.as_bytes())
            .expect("Failed to write to project.properties");

        println!("Created project.properties.");
    } else {
        println!("project.properties already exists.");
    }

    if fs::metadata("requirements.txt").is_err() {
        File::create("requirements.txt").expect("Failed to create requirements.txt");
        println!("Created requirements.txt.");
    } else {
        println!("requirements.txt already exists.");
    }

    if fs::metadata("main.py").is_err() {
        let mut main_py = File::create("main.py").expect("Failed to create main.py");
        main_py
            .write_all(b"print('Hello world!')\n")
            .expect("Failed to write to main.py");

        println!("Created main.py.");
    } else {
        println!("main.py already exists.");
    }

    if fs::metadata(".gitignore").is_err() {
        let mut gitignore = File::create(".gitignore").expect("Failed to create .gitignore");
        let gitignore_content = "__pycache__/\n*.pyc\n.env\n";
        gitignore
            .write_all(gitignore_content.as_bytes())
            .expect("Failed to write to .gitignore");
        println!("Created .gitignore.");
    } else {
        println!(".gitignore already exists.");
    }
}

fn init_git_repo() {
    if Command::new("git").arg("--version").output().is_ok() {
        println!("Initializing Git repository...");

        let status = Command::new("git")
            .arg("init")
            .status()
            .expect("Failed to initialize git repository");

        if status.success() {
            println!("Git repository initialized successfully.");
        } else {
            eprintln!("Failed to initialize Git repository.");
        }
    } else {
        println!("Git is not installed. Skipping repository initialization.");
    }
}
