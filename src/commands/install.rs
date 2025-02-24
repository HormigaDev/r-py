use std::fs;
use std::io::BufRead;
use std::process::Command;

pub fn install(packages: Option<Vec<String>>) {
    if !is_pip_installed() {
        println!("Pip not installed. Installing now...");
        if !install_pip() {
            eprintln!("Error installing pip.");
            return;
        }
    }

    match packages {
        Some(pkgs) if !pkgs.is_empty() => {
            println!("Installing packages: {:?}", pkgs);

            let status = Command::new("pip")
                .arg("install")
                .args(&pkgs)
                .status()
                .expect("Error executing pip install");

            if status.success() {
                println!("Packages installed successfully!");

                for pkg in pkgs {
                    let output = Command::new("pip")
                        .arg("show")
                        .arg(&pkg)
                        .output()
                        .expect("Failed to get package info");

                    if output.status.success() {
                        let output_str = String::from_utf8_lossy(&output.stdout);
                        if let Some(version_line) =
                            output_str.lines().find(|line| line.starts_with("Version:"))
                        {
                            let version = version_line.replace("Version: ", "");
                            let package_entry = format!("{}={}\n", pkg, version);

                            let properties_file = "project.properties";
                            update_dependencies_in_properties(properties_file, &package_entry);

                            println!("Added {} to project.properties", package_entry.trim());
                        }
                    }
                }
            } else {
                eprintln!("Failed to install packages {:?}", pkgs);
            }
        }
        _ => {
            let properties_file = "project.properties";
            if fs::metadata(properties_file).is_ok() {
                println!("Installing packages from project.properties...");
                let pkgs = read_dependencies_from_properties(properties_file);
                if !pkgs.is_empty() {
                    let status = Command::new("pip")
                        .arg("install")
                        .args(&pkgs)
                        .status()
                        .expect("Error executing pip install");

                    if status.success() {
                        println!("Packages installed successfully from project.properties!");
                    } else {
                        eprintln!("Failed to install packages from project.properties");
                    }
                } else {
                    println!("No dependencies found in project.properties.");
                }
            } else {
                println!("No project.properties found. Creating one...");
                fs::File::create(properties_file).expect("Failed to create project.properties");
                println!("project.properties created. You can add packages to it.");
            }
        }
    }
}

fn is_pip_installed() -> bool {
    Command::new("pip").arg("--version").output().is_ok()
}

fn install_pip() -> bool {
    let status = Command::new("python3").arg("-m").arg("ensurepip").status();
    status.is_ok()
}

fn read_dependencies_from_properties(properties_file: &str) -> Vec<String> {
    let mut dependencies = Vec::new();
    if let Ok(file) = fs::OpenOptions::new().read(true).open(properties_file) {
        let reader = std::io::BufReader::new(file);
        let mut inside_dependencies_section = false;

        for line in reader.lines() {
            let line = line.expect("Failed to read line");

            if inside_dependencies_section {
                if line.starts_with('[') || line.trim().is_empty() {
                    break;
                }
                let parts: Vec<&str> = line.split('=').collect();
                if parts.len() == 2 {
                    let package_name = parts[0].trim();
                    let version = parts[1].trim();
                    dependencies.push(format!("{}=={}", package_name, version));
                }
            }

            if line.starts_with("[dependencies]") {
                inside_dependencies_section = true;
            }
        }
    }
    dependencies
}

fn update_dependencies_in_properties(properties_file: &str, package_entry: &str) {
    let mut content = String::new();
    let mut found_dependency = false;

    if let Ok(file) = fs::OpenOptions::new().read(true).open(properties_file) {
        let reader = std::io::BufReader::new(file);
        let mut inside_dependencies_section = false;

        for line in reader.lines() {
            let line = line.expect("Failed to read line");

            if inside_dependencies_section {
                if let Some((pkg_name, _)) = line.split_once('=') {
                    let pkg_name = pkg_name.trim();
                    if package_entry.starts_with(pkg_name) {
                        content.push_str(package_entry);
                        content.push('\n');
                        found_dependency = true;
                        continue;
                    }
                }
                if line.starts_with('[') || line.trim().is_empty() {
                    break;
                }
            }

            if line.starts_with("[dependencies]") {
                inside_dependencies_section = true;
                content.push_str(&line);
                content.push('\n');
            } else {
                content.push_str(&line);
                content.push('\n');
            }
        }

        if !found_dependency {
            if !inside_dependencies_section {
                content.push_str("\n[dependencies]\n");
            }
            content.push_str(package_entry);
            content.push('\n');
        }

        fs::write(properties_file, content).expect("Failed to write to project.properties");
    } else {
        println!("Error: project.properties not found.");
    }
}
