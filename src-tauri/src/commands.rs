use tauri::State;
use crate::state::{Trade, Db};

#[tauri::command]
pub fn create_trade(st:State<Db>, trade:Trade) -> Trade {
 st.trade_token(trade).unwrap()
}

#[tauri::command]
pub fn get_trade_list(st:State<Db>) -> Vec<Trade> {
  st.get_trade_list().unwrap()
}

#[tauri::command]
pub fn delete_trade(st:State<Db>, id:usize) {
  st.delete_trade(id);
}