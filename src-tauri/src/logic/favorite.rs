use rusqlite::{Result};
use crate::model::favorire::Favorite;
use crate::AppState;
use tauri::{self, State};
// invokers

#[tauri::command]
pub fn add_to_favorite(state: State<'_, AppState>, value: String) -> Result<(), String> {
    let mut state = state.store.as_ref().lock().unwrap();
    let conn = state.conn.as_ref().unwrap();
    Favorite::create(conn, value.clone()).unwrap();
    state.remove_item_in_list(value);
    
    Ok(())
}

#[tauri::command]
pub fn get_fav_list(state: State<'_, AppState>,) -> Result<Vec<Favorite>, String> {
    let state = state.store.as_ref().lock().unwrap();
    let conn = state.conn.as_ref().unwrap();
    let rows = Favorite::find_all(conn).unwrap();
    Ok(rows)
}

#[tauri::command]
pub fn remove_from_favorite(state: State<'_, AppState>, id: i32) -> Result<() , String>{
    let state = state.store.as_ref().lock().unwrap();
    let conn = state.conn.as_ref().unwrap();
    conn.execute("DELETE FROM favorite WHERE id=:id", &[(":id", &id)]).unwrap();
    Ok(())
}

