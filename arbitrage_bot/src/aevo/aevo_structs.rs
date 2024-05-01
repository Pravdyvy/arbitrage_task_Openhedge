use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthDataAEVO {
    key: String,
    secret: String,
}

impl AuthDataAEVO {
    pub fn new(key: String, secret: String) -> Self {
        Self { key, secret }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthPayloadAEVO {
    op: String,
    data: AuthDataAEVO,
}

impl AuthPayloadAEVO {
    pub fn new(key: String, secret: String) -> Self {
        Self {
            op: "auth".to_string(),
            data: AuthDataAEVO::new(key, secret),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelsPayloadAEVO {
    op: String,
}

impl ChannelsPayloadAEVO {
    pub fn new() -> Self {
        Self {
            op: "channels".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderbookPayloadAEVO {
    op: String,
    data: Vec<String>,
}

impl OrderbookPayloadAEVO {
    pub fn new(channels: Vec<String>) -> Self {
        Self {
            op: "orderbook".to_string(),
            data: channels,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelsResponseAEVO {
    pub data: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderbookAEVOData {
    r#type: String,
    instrument_type: String,
    bids: Vec<(String, String, String)>,
    asks: Vec<(String, String, String)>,
    last_updated: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderbookAEVOResponse {
    channel: String,
    data: OrderbookAEVOData,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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
