use std::sync::Arc;

use anyhow::Result;
use log::info;
use tokio::sync::Mutex;

use crate::{
    aevo::{
        aevo_orderbook_feed::{AEVOWSAuthenticator, AEVOWSOrderbookFeed},
        aevo_structs::OrderbookAEVO,
    },
    calculations::{check_orderbooks, estimate_swap},
    dxdy::{
        dxdy_orderbook_feed::{DXDYWSAuthenticator, DXDYWSOrderbookFeed},
        dxdy_structs::OrderbookDXDY,
    },
};

pub mod aevo;
pub mod calculations;
pub mod dxdy;
pub mod structs_general;

pub async fn main_loop() -> Result<()> {
    env_logger::init();

    info!("Starting main loop");

    let mut cumulative_p_l = 0;
    let mut balance = 1000;

    let orderbook_aevo = OrderbookAEVO::default();
    let orderbook_dxdy = OrderbookDXDY::default();

    let orderbook_aevo_ref = Arc::new(Mutex::new(orderbook_aevo));
    let orderbook_dxdy_ref = Arc::new(Mutex::new(orderbook_dxdy));

    let aevo_auth = AEVOWSAuthenticator::new("wss://ws.aevo.xyz");
    let dxdy_auth = DXDYWSAuthenticator::new("wss://indexer.dydx.trade/v4/ws");

    let aevo_channel = aevo_auth.authenticate().await?;
    let dxdy_channel = dxdy_auth.authenticate().await?;

    let aevo_feeder = AEVOWSOrderbookFeed::new(aevo_channel);
    let dxdy_feeder = DXDYWSOrderbookFeed::new(dxdy_channel);

    aevo_feeder.spawn_feed(orderbook_aevo_ref.clone()).await?;
    dxdy_feeder.spawn_feed(orderbook_dxdy_ref.clone()).await?;

    loop {
        //Search for arbitrage posibilities
        let direction = check_orderbooks(orderbook_aevo_ref.clone(), orderbook_dxdy_ref.clone(), balance);

        //If p&l is positive, initiate trading
        let p_l = estimate_swap(direction, balance);
        cumulative_p_l += p_l;

        //update results in log format
        info!("Profit and loss after last trade : {p_l}");
        info!("Cumulative profit and loss : {cumulative_p_l}");
    }
}
