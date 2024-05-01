use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderbookPayloadDXDY {
    r#type: String,
    channel: String,
    id: String,
}

impl OrderbookPayloadDXDY {
    pub fn new() -> Self {
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
pub struct OrderbookDXDY {
    pub bids: Vec<PriceDataDXDY>,
    pub asks: Vec<PriceDataDXDY>,
}

impl OrderbookDXDY {
    pub fn apply_changes(&mut self, resp: OrderbookDXDYResponse) {
        if resp.r#type == "subscribed" {
            self.asks = resp.contents.asks;
            self.bids = resp.contents.bids;
        } else {
            for item in resp.contents.asks {
                self.asks.push(item);
            }
            for item in resp.contents.bids {
                self.bids.push(item);
            }
        }
    }
}
