#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::{Arc, Mutex};
use std::time::Duration;
use tauri::{Manager, State, ClipboardManager};
use tokio::time::sleep;


struct MyState {
    clip_list: Vec<String>,
}

impl Default for MyState {
    fn default() -> Self {
        Self { clip_list: Vec::new() }
    }
}

impl MyState {
    fn update(&mut self, action: &str, payload: String) {
        if action == "push" {
            let mut new_list  = self.clip_list.clone();
            new_list.extend_from_slice(&[payload]);
            // new_list.reverse();
            self.clip_list = new_list;
        }
    }
}

struct AppState {
    store: Arc<Mutex<MyState>>
}

// invoke commands

#[tauri::command]
fn get_clip_list(state: State<'_, AppState>) -> Vec<String> {
  state.store.lock().unwrap().clip_list.clone()
}

#[tauri::command]
fn set_into_clipboard(value: String, app: tauri::AppHandle){
    let app_handler = app.app_handle();
    app_handler.clipboard_manager().write_text(value).unwrap();
}


fn main() {
    let state = AppState {
        store: Arc::new(Mutex::new(MyState::default()))
    };
    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![get_clip_list, set_into_clipboard])
        .setup(|app| {
            let app_handler = app.app_handle();
            tauri::async_runtime::spawn(async move {
                sleep(Duration::from_secs(1)).await;
                let app_handler = app_handler.clone();
                loop {
                    let state: State<AppState> = app_handler.state();
                    let empty_string = String::from("");
                    let empty_list = Vec::from([empty_string.clone()]);
                    let clipboard = app_handler.clipboard_manager();
                    let clip_text = clipboard.to_owned().read_text().unwrap().unwrap();
                    let store =state.store.clone();
                    let list = store.lock().unwrap().clip_list.clone();
                    let list = if list.len() == 0 { empty_list } else { list };
                    if list.last().unwrap() != &clip_text {
                        store.lock().unwrap().update("push", clip_text);
                        app_handler.emit_all("clipboard-update", "ping").unwrap()
                    }
                };
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
