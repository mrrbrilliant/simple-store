use std::process::Command;

pub fn smart_remove(packages: &[&str]) -> Result<(), String> {
    println!("Removing: {}", packages.join(" "));

    let command = Command::new("pkexec")
        .arg("pacman")
        .arg("-R")
        .arg("--noconfirm")
        .args(packages)
        .output();

    if !command.as_ref().unwrap().status.success() {
        let stderr = String::from_utf8(command.as_ref().unwrap().stderr.clone()).unwrap();

        return Err(stderr);
    }

    Ok(())
}

pub fn orphans() -> Option<Vec<String>> {
    let orphans = Command::new("pacman").arg("-Qqdt").output();

    if orphans.as_ref().unwrap().status.success() {
        let output = String::from_utf8(orphans.as_ref().unwrap().stdout.clone()).unwrap();
        let packages: Vec<String> = output.lines().map(|line| line.to_string()).collect();
        return Some(packages);
    }

    None
}

pub fn remove_orphans() -> Result<(), String> {
    let ops = orphans();

    if let Some(packages) = ops {
        let command = Command::new("pacman").arg("-Rsn").args(&packages).output();

        if !command.as_ref().unwrap().status.success() {
            // let mut stdout = String::from_utf8(command.as_ref().unwrap().stdout.clone()).unwrap();
            let stderr = String::from_utf8(command.as_ref().unwrap().stderr.clone()).unwrap();
            // stdout.push_str(&stderr);
            return Err(stderr);
        }
        return Ok(());
    }
    Ok(())
}
