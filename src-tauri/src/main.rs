#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

#[allow(clippy::wildcard_imports)]
use repos_menubar::{commands::*, db, events::*};
use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu};

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit").accelerator("Cmd+Q");
    let system_tray_menu = SystemTrayMenu::new().add_item(quit);

    #[allow(clippy::expect_used)]
    tauri::Builder::default()
        .setup(|app| {
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);
            Ok(())
        })
        .plugin(tauri_plugin_positioner::init())
        .plugin(tauri_plugin_sql::Builder::default().build())
        .setup(|_app| {
            tauri::async_runtime::spawn(async move {
                // Initialize the database.
                db::init().await;
            });

            Ok(())
        })
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            add_new_repo,
            remove_repo,
            list_repos
        ])
        .system_tray(SystemTray::new().with_menu(system_tray_menu))
        .on_system_tray_event(system_tray_event_handler)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
