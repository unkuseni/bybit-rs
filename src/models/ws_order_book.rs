use crate::prelude::*;

/// Structure for WebSocket order book data.
///
/// Contains the bids, asks, and sequence numbers for a trading pair’s order book. Bots use this to maintain an up-to-date view of market depth and liquidity.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WsOrderBook {
    /// The trading pair symbol (e.g., "BTCUSDT").
    ///
    /// Identifies the perpetual futures contract for the order book. Bots use this to verify the correct market.
    #[serde(rename = "s")]
    pub symbol: String,
    /// A list of ask prices and quantities.
    ///
    /// Contains the current ask levels in the order book, sorted by price. Bots use this to assess selling pressure and liquidity on the ask side.
    #[serde(rename = "a")]
    pub asks: Vec<Ask>,
    /// A list of bid prices and quantities.
    ///
    /// Contains the current bid levels in the order book, sorted by price. Bots use this to assess buying pressure and liquidity on the bid side.
    #[serde(rename = "b")]
    pub bids: Vec<Bid>,
    /// The update ID for the order book.
    ///
    /// A unique identifier for the order book update. Bots use this to ensure updates are processed in the correct order.
    #[serde(rename = "u")]
    pub update_id: u64,
    /// The sequence number for the update.
    ///
    /// A monotonically increasing number for ordering updates. Bots use this to detect missing or out-of-order updates and maintain order book consistency.
    pub seq: u64,
}
