use serde::{Deserialize, Serialize};
use std::sync::RwLock;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TradeType {
    BUY,
    SELL,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    id: Option<usize>,
    token: String,
    qty: i32,
    rate: f32,
    trade_type: TradeType,
}

pub struct Db {
    store: RwLock<Vec<Trade>>,
}

impl Db {
    pub fn new() -> Self {
        Self {
            store: RwLock::new(Vec::new()),
        }
    }

    pub fn trade_token(&self, trade: Trade) {
        let mut store = self.store.write().unwrap();
        let id = store.len() + 1;
        match trade.trade_type {
            TradeType::BUY => {
                let t: Trade = Trade {
                    id: Some(id),
                    token: trade.token,
                    qty: trade.qty,
                    rate: trade.rate,
                    trade_type: TradeType::BUY,
                };
                store.push(t);
            }
            TradeType::SELL => {
                let t: Trade = Trade {
                    id: Some(id),
                    token: trade.token,
                    qty: trade.qty,
                    rate: trade.rate,
                    trade_type: TradeType::SELL,
                };
                store.push(t);
            }
        }
    }

    pub fn get_trade_list(&self) -> Result<Vec<Trade>, Vec<Trade>> {
        let store = self.store.read().unwrap();
        Ok(store.clone())
    }

    pub fn delete_trade(&self, id: usize) {
        let mut store = self.store.write().unwrap();
        let index = store.iter().position(|t| t.id == Some(id)).unwrap();
        store.remove(index);
    }
}
