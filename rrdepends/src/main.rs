use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


#[derive(Debug)]
struct PackageInfo {
    name: String,
    version: String,
    dependencies: Vec<String>,
}


fn main() {
    let filename = "/workspaces/rrdepends/deb_security";

    match read_and_parse_file(filename) {
        Ok(packages) => {
            for package in packages {
                println!("Package: {}, Version: {}, Dependencies: {:?}", package.name, package.version, package.dependencies);
            }
        },
        Err(e) => println!("Error reading file: {}", e),
    }
}

fn read_and_parse_file(filename: &str) -> io::Result<Vec<PackageInfo>> {
    let path = Path::new(filename);
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut packages = Vec::new();
    let mut current_package = String::new();
    let mut current_version = String::new();
    let mut dependencies = Vec::new();

    for line in reader.lines() {
        let line = line?;

        if line.starts_with("Package: ") {
            if !current_package.is_empty() {
                packages.push(PackageInfo {
                    name: current_package,
                    dependencies,
                    version: current_version.clone(),
                });
                dependencies = Vec::new(); // Clear dependencies for the next package
            }
            current_package = line["Package: ".len()..].trim().to_string();
        } else if line.starts_with("Depends: ") {
            dependencies = line["Depends: ".len()..].trim()
                .split(", ")
                .map(|s| s.split_whitespace().next().unwrap_or("").to_string())
                .collect();
        } else if line.starts_with("Version: ") {
            current_version = line["Version: ".len()..].trim().to_string();
        }
    }

    // Don't forget to add the last package in the file
    if !current_package.is_empty() {
        packages.push(PackageInfo {
            name: current_package,
            dependencies,
            version: current_version.clone(),
        });
    }

    Ok(packages)
}