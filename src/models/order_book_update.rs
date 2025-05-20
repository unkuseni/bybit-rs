use crate::prelude::*;

/// Structure for WebSocket order book update events.
///
/// Contains real-time updates to the order book for a trading pair, including bids, asks, and sequence numbers. Bots use this for market depth analysis and liquidity monitoring in perpetual futures trading.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderBookUpdate {
    /// The WebSocket topic for the event (e.g., "orderbook.50.BTCUSDT").
    ///
    /// Specifies the data stream for the order book update, including depth and symbol. Bots use this to verify the correct market and depth level.
    #[serde(rename = "topic")]
    pub topic: String,
    /// The event type (e.g., "snapshot", "delta").
    ///
    /// Indicates whether the update is a full snapshot or incremental delta. Bots use this to initialize or update their order book state.
    #[serde(rename = "type")]
    pub event_type: String,
    /// The timestamp of the event in milliseconds.
    ///
    /// Indicates when the order book update was generated. Bots use this to ensure data freshness and align with other market data.
    #[serde(rename = "ts")]
    pub timestamp: u64,
    /// The order book data.
    ///
    /// Contains the bids, asks, and sequence numbers for the order book. Bots use this to update their internal order book representation.
    pub data: WsOrderBook,
    /// The creation timestamp in milliseconds.
    ///
    /// Indicates when the order book update was created by Bybit’s server. Bots use this to measure latency and ensure data consistency.
    pub cts: u64,
}
