use std::process::Command;

pub fn list_all_installed() -> Option<Vec<String>> {
    let command = Command::new("pacman").args(&["-Qi"]).output();

    if command.as_ref().unwrap().status.success() {
        let mut stdout = String::from_utf8(command.as_ref().unwrap().stdout.clone()).unwrap();
        let stderr = String::from_utf8(command.as_ref().unwrap().stderr.clone()).unwrap();
        stdout.push_str(&stderr);

        let packages: Vec<String> = stdout
            .split("\n\n")
            .filter(|p| !p.is_empty())
            .map(|p| p.to_string())
            .collect();
        return Some(packages);
    }
    None
}

pub fn is_installed(package: &str) -> Option<String> {
    let command = Command::new("pacman").args(&["-Qi", package]).output();

    if command.as_ref().unwrap().status.success() {
        let mut stdout = String::from_utf8(command.as_ref().unwrap().stdout.clone()).unwrap();
        let stderr = String::from_utf8(command.as_ref().unwrap().stderr.clone()).unwrap();
        stdout.push_str(&stderr);
        return Some(stdout);
    }
    None
}

pub fn are_installed(packages: &[&str]) -> Option<Vec<String>> {
    let command = Command::new("pacman").arg("-Qi").args(packages).output();

    if command.as_ref().unwrap().status.success() {
        let mut stdout = String::from_utf8(command.as_ref().unwrap().stdout.clone()).unwrap();
        let stderr = String::from_utf8(command.as_ref().unwrap().stderr.clone()).unwrap();
        stdout.push_str(&stderr);
        let packages: Vec<String> = stdout
            .split("\n\n")
            .filter(|p| !p.is_empty())
            .map(|p| p.to_string())
            .collect();
        return Some(packages);
    }

    None
}
