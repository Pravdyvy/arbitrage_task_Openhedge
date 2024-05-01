use std::collections::BTreeMap;

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

#[derive(Debug, Clone, Default)]
pub struct OrderbookAEVO {
    pub bids: BTreeMap<u64, (u64, u64, f64)>,
    pub asks: BTreeMap<u64, (u64, u64, f64)>,
}

impl OrderbookAEVO {
    pub fn apply_changes(&mut self, resp: OrderbookAEVOResponse) {
        if resp.data.r#type == "snapshot" {
            self.asks = resp
                .data
                .asks
                .into_iter()
                .map(|(price, amount, iv)| {
                    (
                        price.clone().parse().unwrap(),
                        (
                            price.parse().unwrap(),
                            amount.parse().unwrap(),
                            iv.parse().unwrap(),
                        ),
                    )
                })
                .collect();
            self.bids = resp
                .data
                .bids
                .into_iter()
                .map(|(price, amount, iv)| {
                    (
                        price.clone().parse().unwrap(),
                        (
                            price.parse().unwrap(),
                            amount.parse().unwrap(),
                            iv.parse().unwrap(),
                        ),
                    )
                })
                .collect();
        } else {
            for (price, amount, iv) in resp.data.asks {
                self.asks.insert(
                    price.clone().parse().unwrap(),
                    (
                        price.parse().unwrap(),
                        amount.parse().unwrap(),
                        iv.parse().unwrap(),
                    ),
                );
            }
            for (price, amount, iv) in resp.data.bids {
                self.bids.insert(
                    price.clone().parse().unwrap(),
                    (
                        price.parse().unwrap(),
                        amount.parse().unwrap(),
                        iv.parse().unwrap(),
                    ),
                );
            }
        }
    }
}
