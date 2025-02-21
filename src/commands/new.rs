use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::process::{self, Command};

pub fn new_project(name: String) {
    let project_path = Path::new(&name);

    if project_path.exists() {
        if project_path.read_dir().unwrap().count() > 0 {
            eprintln!(
                "Error: The directory '{}' already exists and is not empty.",
                name
            );
            process::exit(1);
        } else {
            println!(
                "The directory '{}' exists and is empty. Creating files...",
                name
            );

            create_files(&project_path, &name);
            init_git_repo(&project_path);
        }
    } else {
        println!(
            "The directory '{}' does not exist. Creating the directory and files...",
            name
        );
        fs::create_dir_all(&name).expect("Error creating the directory");

        create_files(&project_path, &name);
        init_git_repo(&project_path);
    }
}

fn create_files(project_path: &Path, name: &str) {
    let template_content = include_str!("../project.properties.template");

    let content = template_content
        .replace("NAME", name)
        .replace("VERSION", "0.0.1");

    let project_properties_path = project_path.join("project.properties");
    let mut project_properties =
        File::create(project_properties_path).expect("Error creating project.properties");

    project_properties
        .write_all(content.as_bytes())
        .expect("Error writing to project.properties");

    let requirements_path = project_path.join("requirements.txt");
    File::create(requirements_path).expect("Error creating requirements.txt");

    let main_py_path = project_path.join("main.py");
    let mut main_py = File::create(main_py_path).expect("Error creating main.py");

    let main_py_content = r#"print("Hello world!")"#;
    main_py
        .write_all(main_py_content.as_bytes())
        .expect("Error writing to main.py");

    let gitignore_path = project_path.join(".gitignore");
    let mut gitignore = File::create(gitignore_path).expect("Error creating .gitignore");
    let gitignore_content = "__pycache__/\n*.pyc\n.env\n";
    gitignore
        .write_all(gitignore_content.as_bytes())
        .expect("Error writing to .gitignore");

    println!("Files successfully created in the directory '{}'.", name);
}

fn init_git_repo(project_path: &Path) {
    if Command::new("git").arg("--version").output().is_ok() {
        println!("Initializing a new Git repository...");
        let status = Command::new("git")
            .arg("init")
            .current_dir(project_path)
            .status()
            .expect("Failed to execute git init");

        if status.success() {
            println!("Git repository initialized successfully.");
        } else {
            eprintln!("Failed to initialize Git repository.");
        }
    } else {
        println!("Git is not installed. Skipping repository initialization.");
    }
}
