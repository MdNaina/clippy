use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, AppHandle, SystemTrayEvent, Manager};

pub fn init_system_tray() -> SystemTray {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Show/Hide");
    let tray_menu = SystemTrayMenu::new()
        .add_item(quit)
        .add_item(hide);

    SystemTray::new().with_menu(tray_menu)
}

pub fn handle_system_tray(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        // SystemTrayEvent::DoubleClick { position: _, size: _, .. } => {
        //     println!("system tray received a left click");
        //     let window = app.get_window("main").unwrap();
        //     window.show().unwrap()
        // }
        SystemTrayEvent::MenuItemClick { id, .. } => {
            match id.as_str() {
                "hide" => {
                    let window = app.get_window("main").unwrap();
                    if window.is_visible().unwrap() {
                        window.hide().unwrap()
                    }else{
                        window.show().unwrap()
                    }
                }
                "quit" => {
                    std::process::exit(0);
                }
                _ => ()
            }
        }
        _ => ()
    }
}