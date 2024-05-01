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