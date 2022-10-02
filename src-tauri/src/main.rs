#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tauri::{Manager, State, ClipboardManager, WindowEvent};
use tokio::time::sleep;

use rusqlite::Connection;

mod model;
mod logic;

use logic::favorite::{get_fav_list, add_to_favorite, remove_from_favorite};

struct MyState {
    clip_list: Vec<String>,
    conn: Option<Connection>
}

impl Default for MyState {
    fn default() -> Self {
        Self { clip_list: Vec::new(), conn: None }
    }
}

impl MyState {
    fn update(&mut self, action: &str, payload: String) {
        if action == "push" {
            if self.clip_list.contains(&payload) {
                let old_value = payload.clone();
                self.remove_item_in_list(old_value);
            }
            self.clip_list.push(payload)
        }
        if self.clip_list.len() > 20 {
            let leng = self.clip_list.len();
            self.clip_list = self.clip_list[leng - 20 .. leng].to_vec()
        }
       
    }
    fn remove_item_in_list(&mut self, value: String){
        self.clip_list.remove(self.clip_list.iter().position(|x| *x == value).expect("not found"));
    }

    fn initialize_db(&mut self, path: PathBuf) {
        self.conn = Some(Connection::open(path.to_str().unwrap().to_owned()+ "/store.db").unwrap());
        self.conn.as_ref().unwrap().execute(r#"
        CREATE TABLE IF NOT EXISTS favorite (
            id INTEGER PRIMARY KEY,
            value TEXT NOT NULL
        );
        "#, []).unwrap();
    }
}

pub struct AppState {
    store: Arc<Mutex<MyState>>,
}

// invoke commands

#[tauri::command]
fn get_clip_list(state: State<'_, AppState>) -> Vec<String> {
  state.store.lock().unwrap().clip_list.clone()
}

#[tauri::command]
fn remove_from_clip(state: State<'_, AppState>, value: String) {
  state.store.lock().unwrap().remove_item_in_list(value)
}

mod system_tray;

fn main() {
    let state = AppState {
        store: Arc::new(Mutex::new(MyState::default())),
    };
    tauri::Builder::default()
        .manage(state)
        .system_tray(system_tray::init_system_tray())
        .on_system_tray_event(system_tray::handle_system_tray)
        .invoke_handler(tauri::generate_handler![
            // clipboard
            get_clip_list, 
            remove_from_clip,
            // favorite
            get_fav_list, 
            add_to_favorite,
            remove_from_favorite
        ])
        .setup(|app| {
            let app_handler = app.app_handle();
            let path = app_handler.path_resolver().app_dir().to_owned();
            let path = path.unwrap();
            if !path.is_dir() {
                std::fs::create_dir_all(path.clone()).unwrap();
            }
            let state: State<AppState> =  app_handler.state();
            state.store.lock().unwrap().initialize_db(path);
            tauri::async_runtime::spawn(async move {
                sleep(Duration::from_secs(1)).await;
                let app_handler = app_handler.clone();
                loop {
                    let state: State<AppState> = app_handler.state();
                    let empty_string = String::from("");
                    let empty_list = Vec::from([empty_string.clone()]);
                    let clipboard = app_handler.clipboard_manager();
                    let clip_text = clipboard.to_owned().read_text().unwrap_or_else(|_| Some("".to_string())).unwrap();
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
        .on_window_event(|event| {
            let window_event = event.window();
            match event.event() {
                WindowEvent::CloseRequested { api , .. } => {
                    api.prevent_close();
                    window_event.hide().unwrap();
                },
                _ => (),
            }})
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
