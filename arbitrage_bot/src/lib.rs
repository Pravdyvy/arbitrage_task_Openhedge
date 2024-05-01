use anyhow::Result;
use log::info;

use crate::calculations::{check_orderbooks, estimate_swap, update_orderbooks};

pub mod aevo_client;
pub mod calculations;
pub mod dxdy_client;
pub mod structs;

pub async fn main_loop() -> Result<()> {
    env_logger::init();

    info!("Starting main loop");

    let mut cumulative_p_l = 0;

    loop {
        //Receive updates from exchanges
        update_orderbooks().await?;

        //Search for arbitrage posibilities
        let direction = check_orderbooks();

        //If p&l is positive, initiate trading
        let p_l = estimate_swap(direction);
        cumulative_p_l += p_l;

        //update results in log format
        info!("Profit and loss after last trade : {p_l}");
        info!("Cumilative profit and loss : {cumulative_p_l}");
    }
}
