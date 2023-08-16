use repos_menubar::{commands::*, events::*};
use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu};

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit").accelerator("Cmd+Q");
    let system_tray_menu = SystemTrayMenu::new().add_item(quit);
    tauri::Builder::default()
        .setup(|app| {
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);
            Ok(())
        })
        .plugin(tauri_plugin_positioner::init())
        .invoke_handler(tauri::generate_handler![list_repos])
        .system_tray(SystemTray::new().with_menu(system_tray_menu))
        .on_system_tray_event(system_tray_event_handler)
        .on_window_event(window_event_handler)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
