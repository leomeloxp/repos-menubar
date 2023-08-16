use tauri::{AppHandle, GlobalWindowEvent, Manager, SystemTrayEvent, Wry};
use tauri_plugin_positioner::{Position, WindowExt};

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

pub fn window_event_handler(event: GlobalWindowEvent) {
    if let tauri::WindowEvent::Focused(is_focused) = event.event() {
        // detect click outside of the focused window and hide the app
        if !is_focused {
            let _ = event.window().hide();
        }
    }
}
