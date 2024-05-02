use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderbookPayloadDXDY {
    r#type: String,
    channel: String,
    id: String,
}

impl Default for OrderbookPayloadDXDY {
    fn default() -> Self {
        Self {
            r#type: "subscribe".to_string(),
            channel: "v4_orderbook".to_string(),
            id: "ETH-USDC".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceDataDXDY {
    price: String,
    size: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderbookDXDYData {
    bids: Vec<PriceDataDXDY>,
    asks: Vec<PriceDataDXDY>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderbookDXDYResponse {
    r#type: String,
    contents: OrderbookDXDYData,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// dXdY Orderbook struct
/// 
/// Storing BTreeMaps to make insert operations fast 
pub struct OrderbookDXDY {
    pub bids: BTreeMap<u64, (u64, u64)>,
    pub asks: BTreeMap<u64, (u64, u64)>,
}

impl OrderbookDXDY {
    pub fn apply_changes(&mut self, resp: OrderbookDXDYResponse) {
        if resp.r#type == "subscribed" {
            self.asks = resp
                .contents
                .asks
                .into_iter()
                .map(|item| {
                    (
                        item.price.clone().parse().unwrap(),
                        (item.price.parse().unwrap(), item.size.parse().unwrap()),
                    )
                })
                .collect();
            self.bids = resp
                .contents
                .bids
                .into_iter()
                .map(|item| {
                    (
                        item.price.clone().parse().unwrap(),
                        (item.price.parse().unwrap(), item.size.parse().unwrap()),
                    )
                })
                .collect();
        } else {
            for item in resp.contents.asks {
                self.asks.insert(
                    item.price.clone().parse().unwrap(),
                    (item.price.parse().unwrap(), item.size.parse().unwrap()),
                );
            }
            for item in resp.contents.bids {
                self.bids.insert(
                    item.price.clone().parse().unwrap(),
                    (item.price.parse().unwrap(), item.size.parse().unwrap()),
                );
            }
        }
    }
}
