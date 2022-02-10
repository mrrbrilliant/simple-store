use std::env;
use std::fs::create_dir_all;
use std::path::Path;
use walkdir::WalkDir;

pub fn list_installed() -> Option<Vec<String>> {
    // Get home directory
    let key = "HOME";
    let home = env::var(key);

    let mut packages: Vec<String> = Vec::new();

    if let Ok(home) = home {
        // App directory
        let app_dir = Path::new(&home).join("Applications");

        if !app_dir.exists() {
            create_dir_all(&app_dir).unwrap();
        }
        // List all AppImages
        for entry in WalkDir::new(app_dir).min_depth(1).max_depth(2) {
            let path = entry
                .as_ref()
                .unwrap()
                .path()
                .to_path_buf()
                .display()
                .to_string();
            if path.to_lowercase().ends_with(".appimage") {
                packages.push(path);
            }
        }
    }

    if !packages.is_empty() {
        return Some(packages);
    }
    None
}
