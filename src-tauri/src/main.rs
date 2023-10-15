// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod state;
use crate::state::{Db, Trade};
use tauri::State;

#[tauri::command]
fn create_trade(st: State<Db>, trade: Trade) {
    st.trade_token(trade);
}

#[tauri::command]
fn get_trade_list(st: State<Db>) -> Vec<Trade> {
    st.get_trade_list().unwrap()
}

#[tauri::command]
fn delete_trade(st: State<Db>, id: usize) {
    st.delete_trade(id);
}
fn main() {
    tauri::Builder::default()
        .manage(state::Db::new())
        .plugin(tauri_plugin_websocket::init())
        .invoke_handler(tauri::generate_handler![
            create_trade,
            get_trade_list,
            delete_trade
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
