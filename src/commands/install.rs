use std::fs;
use std::io::Write;
use std::process::Command;

pub fn install(packages: Option<Vec<String>>) {
    match packages {
        Some(pkgs) if !pkgs.is_empty() => {
            println!("Installing packages: {:?}", pkgs);

            let status = Command::new("pip")
                .arg("install")
                .args(&pkgs) // Usamos .args() para pasar múltiples paquetes
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
                            let package_entry = format!("{}=={}\n", pkg, version);

                            let req_file = "requirements.txt";
                            let mut file = fs::OpenOptions::new()
                                .create(true)
                                .append(true)
                                .open(req_file)
                                .expect("Failed to open requirements.txt");

                            file.write_all(package_entry.as_bytes())
                                .expect("Failed to write to requirements.txt");

                            println!("Added {} to requirements.txt", package_entry.trim());
                        }
                    }
                }
            } else {
                eprintln!("Failed to install packages {:?}", pkgs);
            }
        }
        _ => {
            let req_file = "requirements.txt";
            if fs::metadata(req_file).is_ok() {
                println!("Installing from requirements.txt...");
                let status = Command::new("pip")
                    .arg("install")
                    .arg("-r")
                    .arg(req_file)
                    .status()
                    .expect("Error executing pip install -r");

                if status.success() {
                    println!("Packages installed successfully from requirements.txt!");
                } else {
                    eprintln!("Failed to install packages from requirements.txt");
                }
            } else {
                println!("No requirements.txt found. Creating one...");
                fs::File::create(req_file).expect("Failed to create requirements.txt");
                println!("requirements.txt created. You can add packages to it.");
            }
        }
    }
}
