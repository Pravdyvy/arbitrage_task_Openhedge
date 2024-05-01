use std::sync::Arc;
use tokio::sync::Mutex;



use crate::{
    aevo::aevo_structs::OrderbookAEVO, dxdy::dxdy_structs::OrderbookDXDY, structs_general::TradeDirection,
};

pub fn estimate_swap(direction: TradeDirection, mut balance: u64) -> u64 {
    todo!()
}

pub fn check_orderbooks(
    orderbokk_aevo: Arc<Mutex<OrderbookAEVO>>,
    orderbokk_dxdy: Arc<Mutex<OrderbookDXDY>>,
    balance: u64,
) -> TradeDirection {
    todo!()
}
