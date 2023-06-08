use dirs;
use git2::Repository;
use serde::Serialize;
use std::fs;
use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu};
use tauri_plugin_positioner::{Position, WindowExt};

#[derive(Debug, Serialize)]
struct RepoInfo {
    full_path: String,
    path: String,
    branch_name: String,
}

fn expand_home_dir(path: &str) -> String {
    if let Some(home_dir) = dirs::home_dir() {
        if path.starts_with("~/") {
            let suffix = &path[2..];
            let home_dir_path = home_dir.to_string_lossy();
            return format!("{}/{}", home_dir_path, suffix);
        }
    }
    path.to_string()
}

#[tauri::command]
fn list_repos(repo_path: &str) -> Vec<RepoInfo> {
    println!("Listing repos in {}", repo_path);

    let ignored_files = vec![".DS_Store"]; // Add other ignored files if needed
    let mut result = Vec::new();

    let expanded_path = expand_home_dir(&repo_path);

    // Get a list of directories in the repository path
    let dirs = match fs::read_dir(&expanded_path) {
        Ok(dirs) => dirs,
        Err(e) => {
            println!("Failed to read directory: {}", e);
            panic!("Failed to read directory.")
        }
    };

    for dir in dirs {
        if let Ok(entry) = dir {
            let path = entry.path();
            // Skip ignored files
            if let Some(entry_name) = path.file_name() {
                let name_str = entry_name.to_string_lossy();
                if ignored_files.contains(&name_str.as_ref()) {
                    continue;
                }
            }

            let path_str = path
                .strip_prefix(&expanded_path)
                .unwrap_or(&path)
                .to_string_lossy();
            let full_path_str = path.to_string_lossy();

            if let Ok(repo) = Repository::open(entry.path()) {
                if let Ok(head) = repo.head() {
                    let branch_name = match head.shorthand() {
                        Some(name) => name,
                        None => "Detached HEAD",
                    };

                    let repo_info = RepoInfo {
                        full_path: String::from(full_path_str),
                        path: String::from(path_str),
                        branch_name: String::from(branch_name),
                    };

                    result.push(repo_info);
                } else {
                    let repo_info = RepoInfo {
                        full_path: String::from(full_path_str),
                        path: String::from(path_str),
                        branch_name: String::from("No branch found"),
                    };
                    result.push(repo_info);
                }
            }
        }
    }

    result
}

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit").accelerator("Cmd+Q");
    let system_tray_menu = SystemTrayMenu::new().add_item(quit);
    tauri::Builder::default()
        .setup(|app| Ok(app.set_activation_policy(tauri::ActivationPolicy::Accessory)))
        .plugin(tauri_plugin_positioner::init())
        // This is where you pass in your commands
        .invoke_handler(tauri::generate_handler![list_repos])
        .system_tray(SystemTray::new().with_menu(system_tray_menu))
        .on_system_tray_event(|app, event| {
            tauri_plugin_positioner::on_tray_event(app, &event);
            match event {
                SystemTrayEvent::LeftClick {
                    position: _,
                    size: _,
                    ..
                } => {
                    let window = app.get_window("main").unwrap();
                    // use TrayCenter as initial window position
                    let _ = window.move_window(Position::TrayCenter);
                    if window.is_visible().unwrap() {
                        window.hide().unwrap();
                    } else {
                        window.show().unwrap();
                        window.set_focus().unwrap();
                    }
                }
                SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                    "quit" => {
                        std::process::exit(0);
                    }
                    _ => {}
                },
                _ => {}
            }
        })
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::Focused(is_focused) => {
                // detect click outside of the focused window and hide the app
                if !is_focused {
                    event.window().hide().unwrap();
                }
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
