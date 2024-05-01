use std::{cmp, sync::Arc};
use tokio::sync::Mutex;

///Price delta after arbitrage operation
pub type PriceDelta = u64;

use crate::{aevo::aevo_structs::OrderbookAEVO, dxdy::dxdy_structs::OrderbookDXDY};

impl OrderbookAEVO {
    ///Matches with all bids to buy as much asset as possible with balance
    ///
    /// Assuming that our sum is relatively small, small enough, to be fully spent
    pub fn buy_as_much_as_possible(&self, balance: u64) -> u64 {
        let mut asset_balance = 0;
        let mut curr_balacne = balance;

        for (price, (_, size, _)) in self.bids.iter().rev() {
            if price*size <= curr_balacne {
                asset_balance += size;
                curr_balacne -= price*size;
            } else {
                asset_balance += curr_balacne/price;
                curr_balacne = 0;
            }

            if curr_balacne == 0 {
                break;
            }
        }

        asset_balance
    }

    ///Matches with all asks to sell as much asset as possible with asset_balance
    ///
    /// Assuming that our sum is relatively small, small enough, to be fully spent
    pub fn sell_as_much_as_possible(&self, asset_balance: u64) -> u64 {
        let mut balance = 0;
        let mut curr_asset_balacne = asset_balance;

        for (price, (_, size, _)) in self.asks.iter() {
            if *size <= curr_asset_balacne {
                balance += size*price;
                curr_asset_balacne -= size;
            } else {
                balance += curr_asset_balacne*price;
                curr_asset_balacne = 0;
            }

            if curr_asset_balacne == 0 {
                break;
            }
        }

        balance
    }
}

impl OrderbookDXDY {
    ///Matches with all bids to buy as much asset as possible with balance
    ///
    /// Assuming that our sum is relatively small, small enough, to be fully spent
    pub fn buy_as_much_as_possible(&self, balance: u64) -> u64 {
        let mut asset_balance = 0;
        let mut curr_balacne = balance;

        for (price, (_, size)) in self.bids.iter().rev() {
            if price*size <= curr_balacne {
                asset_balance += size;
                curr_balacne -= price*size;
            } else {
                asset_balance += curr_balacne/price;
                curr_balacne = 0;
            }

            if curr_balacne == 0 {
                break;
            }
        }

        asset_balance
    }

    ///Matches with all asks to sell as much asset as possible with asset_balance
    ///
    /// Assuming that our sum is relatively small, small enough, to be fully spent
    pub fn sell_as_much_as_possible(&self, asset_balance: u64) -> u64 {
        let mut balance = 0;
        let mut curr_asset_balacne = asset_balance;

        for (price, (_, size)) in self.asks.iter() {
            if *size <= curr_asset_balacne {
                balance += size*price;
                curr_asset_balacne -= size;
            } else {
                balance += curr_asset_balacne*price;
                curr_asset_balacne = 0;
            }

            if curr_asset_balacne == 0 {
                break;
            }
        }

        balance
    }
}

pub async fn check_orderbooks(
    orderbook_aevo: Arc<Mutex<OrderbookAEVO>>,
    orderbook_dxdy: Arc<Mutex<OrderbookDXDY>>,
    balance: u64,
) -> (PriceDelta, i8) {
    //For simplicity sake let`s assume, that we want to have only USDC after operation
    //There is 2 possible variants
    {
        let orderbook_aevo = orderbook_aevo.lock().await;
        let orderbook_dxdy = orderbook_dxdy.lock().await;
        //Buy asset on AEVO sell on dXdY
        let left_buy_right_sell = orderbook_dxdy.sell_as_much_as_possible(orderbook_aevo.buy_as_much_as_possible(balance));
        //Buy asset on dXdY sell on AEVO
        let right_buy_left_sell = orderbook_aevo.sell_as_much_as_possible(orderbook_dxdy.buy_as_much_as_possible(balance));

        let left_right_delta;
        let is_left_right_profitable;
        let right_left_delta;
        let is_right_left_profitable;

        if balance > left_buy_right_sell {
            left_right_delta = balance - left_buy_right_sell;
            is_left_right_profitable = false;
        } else {
            left_right_delta = left_buy_right_sell;
            is_left_right_profitable = true;
        }

        if balance > right_buy_left_sell {
            right_left_delta = balance - right_buy_left_sell;
            is_right_left_profitable = false;
        } else {
            right_left_delta = right_buy_left_sell;
            is_right_left_profitable = true;
        }

        match (is_left_right_profitable, is_right_left_profitable) {
            (false, false) => (cmp::min(left_right_delta, right_left_delta), -1),
            (true, true) => (cmp::max(left_right_delta, right_left_delta) , 1),
            (true, false) => (left_right_delta, 1),
            (false, true) => (right_left_delta, 1),
        }
    }
}
