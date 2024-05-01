use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
///Enum describes on which DEX we have seen arbitrage oportunity
pub enum TradeDirection {
    Left {
        left_price_delta: i64,
        right_price_delta: u64,
    },
    Right {
        left_price_delta: u64,
        right_price_delta: i64,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthData {
    key: String,
    secret: String,
}

impl AuthData {
    pub fn new(key: String, secret: String) -> Self {
        Self { key, secret }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthPayload {
    op: String,
    data: AuthData,
}

impl AuthPayload {
    pub fn new(key: String, secret: String) -> Self {
        Self {
            op: "auth".to_string(),
            data: AuthData::new(key, secret),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelsPayload {
    op: String,
}

impl ChannelsPayload {
    pub fn new() -> Self {
        Self {
            op: "channels".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderbookPayload {
    op: String,
    data: Vec<String>,
}

impl OrderbookPayload {
    pub fn new(channels: Vec<String>) -> Self {
        Self {
            op: "orderbook".to_string(),
            data: channels,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelsResponse {
    pub data: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderbookAEVOData {
    r#type: String,
    instrument_type: String,
    bids: Vec<(String, String, String)>,
    asks: Vec<(String, String, String)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderbookAEVOResponse {
    channel: String,
    data: OrderbookAEVOData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderbookAEVO {
    pub bids: Vec<(String, String, String)>,
    pub asks: Vec<(String, String, String)>,
}

impl OrderbookAEVO {
    pub fn apply_changes(&mut self, resp: OrderbookAEVOResponse) {
        if resp.data.r#type == "snapshot" {
            self.asks = resp.data.asks;
            self.bids = resp.data.bids;
        } else {
            for item in resp.data.asks {
                self.asks.push(item);
            }
            for item in resp.data.bids {
                self.bids.push(item);
            }
        }
    }
}
