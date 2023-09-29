use tauri::{AppHandle, Manager, SystemTrayEvent, Wry};
use tauri_plugin_positioner::{Position, WindowExt};

#[allow(clippy::unwrap_used)]
/// # Panics
///
/// Will panic if any window operation fails.
pub fn system_tray_event_handler(app: &AppHandle<Wry>, event: SystemTrayEvent) {
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
        SystemTrayEvent::MenuItemClick { id, .. } => {
            if id.as_str() == "quit" {
                std::process::exit(0);
            }
        }
        _ => {}
    }
}
