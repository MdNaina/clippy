#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tauri::{Manager, State, ClipboardManager, WindowEvent, GlobalShortcutManager};
use tokio::time::sleep;

use rusqlite::Connection;

mod model;
mod logic;

use logic::favorite::{get_fav_list, add_to_favorite, remove_from_favorite};

struct MyState {
    clip_list: Vec<String>,
    system_clip: Option<String>,
    conn: Option<Connection>
}

impl Default for MyState {
    fn default() -> Self {
        Self { clip_list: Vec::new(), system_clip: None, conn: None }
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
fn copy_text(state: State<'_, AppState>, app_handler: tauri::AppHandle, value: String) {
    state.store.lock().unwrap().system_clip = Some(value.clone());
    let _ = app_handler.clipboard_manager().write_text(value);
    app_handler.get_window("main").unwrap().hide().unwrap();
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
            copy_text,
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
            // register hotkey
            let mut global_shortcut_manager = app_handler.global_shortcut_manager();
            let app_handler_clone = app_handler.clone();
            // use AltOrOption+Shift+V to show the window
            global_shortcut_manager.register("Alt+Shift+V", move || {
                app_handler_clone.get_window("main").unwrap().set_focus().unwrap();
            }).expect("Failed to register hotkey");
            let state: State<AppState> =  app_handler.state();
            let mut global_shortcut_manager = app_handler.global_shortcut_manager();
            let app_handler_clone = app_handler.clone();
            // use AltorOption+Shift+V to show the window
            global_shortcut_manager.register("Alt+Shift+V", move || {
                app_handler_clone.get_window("main").unwrap().set_focus().unwrap();
            }).expect("Failed to register hotkey");
            state.store.lock().unwrap().initialize_db(path);
            tauri::async_runtime::spawn(async move {
                sleep(Duration::from_secs(1)).await;
                let app_handler = app_handler.clone();
                loop {
                    let state: State<AppState> = app_handler.state();
                    let empty_string = String::from("");
                    let empty_list = Vec::from([empty_string.clone()]);
                    let clipboard = app_handler.clipboard_manager();
                    let clip_text = clipboard.to_owned().read_text().unwrap_or_else(|_| Some("".to_string())).unwrap_or_default();
                    let store =state.store.clone();
                    let list = store.lock().unwrap().clip_list.clone();
                    let system_clip = store.lock().unwrap().system_clip.clone();
                    let list = if list.len() == 0 { empty_list } else { list };
                    let last_clip = list.last().unwrap().to_owned();
                    match system_clip {
                        Some(val) => if val != clip_text { push_to_clipboard(last_clip, clip_text, store, &app_handler)},
                        None => push_to_clipboard(last_clip, clip_text, store, &app_handler),
                    };
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

fn push_to_clipboard(last_clip: String, clip_text: String, store: Arc<Mutex<MyState>>, app_handler: &tauri::AppHandle) {
    if last_clip != clip_text && clip_text.trim().len() != 0 {
        store.lock().unwrap().update("push", clip_text.to_owned());
        app_handler.emit_all("clipboard-update", "ping").unwrap();
    };
}
