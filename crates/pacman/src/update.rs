use std::process::Command;

pub fn smart_update(overwrite: bool) -> Result<(), String> {
    println!("Updating system");

    let command = if overwrite {
        Command::new("pkexec")
            .arg("pacman")
            .arg("-Syy")
            .arg("--needed")
            .arg("--noconfirm")
            .output()
    } else {
        Command::new("pkexec")
            .arg("pacman")
            .arg("-Syy")
            .arg("--needed")
            .arg("--noconfirm")
            .arg("--overwrite=\"*\"")
            .output()
    };

    let mut stdout = String::from_utf8(command.as_ref().unwrap().stdout.clone()).unwrap();
    let stderr = String::from_utf8(command.as_ref().unwrap().stderr.clone()).unwrap();
    stdout.push_str(&stderr);

    // Check if the command exits with error
    if !command.as_ref().unwrap().status.success() {
        let mut overwrite: bool = false;

        // Case: conflict files
        if stdout.contains("exists in filesystem") {
            println!("Conflict files detected:");
            println!("Enabling overwrite mode");

            overwrite = true;
        }

        // Case: conflict packages
        if stdout.contains("are in conflict") && stdout.contains("Remove ") {
            let mut conflict_packages: Vec<String> = Vec::new();

            let lines: Vec<_> = stdout.split("\n").collect();

            lines.iter().for_each(|line| {
                if line.contains(" are in conflict.") || line.contains(" are in conflict ") {
                    let words = line.split_whitespace();
                    words.for_each(|word| {
                        if word.contains("?") {
                            let pkg = word.trim_end_matches("?");
                            conflict_packages.push(pkg.to_string())
                        }
                    })
                }
            });

            if !conflict_packages.is_empty() {
                println!("REMOVING: {:?}", conflict_packages);
                let remove_command = Command::new("pacman")
                    .arg("-Rdd")
                    .arg("--noconfirm")
                    .args(conflict_packages)
                    .output();
                remove_command.unwrap();
            }
        }

        // Case: corrupted packages
        if stdout.contains("Failed to commit transaction (invalid or corrupted package)") {
            println!("Failed to commit transaction (invalid or corrupted package)");
            println!("REMOVING: corrupted package");

            let command = Command::new("find")
                .args(&["/var/cache/pacman/pkg/", "-iname", "\"*.part\"", "-delete"])
                .output();
            command.unwrap();
        }

        // Case: cannot lock database
        if stdout.contains("Failed to init transaction (unable to lock database)") {
            println!("Failed to init transaction (unable to lock database)");
            println!("REMOVING: /var/lib/pacman/db.lck");

            let command = Command::new("rm")
                .args(&["/var/lib/pacman/db.lck"])
                .output();
            command.unwrap();
        }

        // Case: PGP error
        if stdout.contains("error: GPGME") {
            println!("error: GPGME");
            println!("REMOVING: /var/lib/pacman/sync/");

            let command = Command::new("rm")
                .args(&["-rf", "/var/lib/pacman/sync/"])
                .output();
            command.unwrap();
        }

        smart_update(overwrite).unwrap();
    }
    Ok(())
}
