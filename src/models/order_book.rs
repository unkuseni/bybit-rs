use crate::prelude::*;

/// Represents the order book for a trading pair.
///
/// Contains the current bid and ask levels, along with metadata like the update ID. Bots use this to analyze market depth and liquidity in perpetual futures.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OrderBook {
    /// The trading pair symbol (e.g., "BTCUSDT").
    ///
    /// Confirms the trading pair for the order book. Bots should verify this matches the requested symbol.
    #[serde(rename = "s")]
    pub symbol: String,
    /// A list of ask (sell) orders.
    ///
    /// Contains the current ask prices and quantities. Bots use this to assess selling pressure and determine resistance levels in perpetual futures.
    #[serde(rename = "a")]
    pub asks: Vec<Ask>,
    /// A list of bid (buy) orders.
    ///
    /// Contains the current bid prices and quantities. Bots use this to assess buying support and determine support levels in perpetual futures.
    #[serde(rename = "b")]
    pub bids: Vec<Bid>,
    /// The timestamp of the order book snapshot (Unix timestamp in milliseconds).
    ///
    /// Indicates when the order book data was captured. Bots should use this to ensure the data is recent, as stale order book data can lead to poor trading decisions.
    #[serde(rename = "ts")]
    pub timestamp: u64,
    /// The update ID of the order book.
    ///
    /// A unique identifier for the order book snapshot. Bots can use this to track updates and ensure they’re processing the latest data, especially in WebSocket streams.
    #[serde(rename = "u")]
    pub update_id: u64,
}
