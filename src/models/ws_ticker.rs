use crate::prelude::*;

/// Structure for WebSocket ticker update events.
///
/// Contains real-time ticker data for a trading pair, such as last price, volume, and funding rates. Bots use this for market monitoring and technical analysis in perpetual futures trading.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WsTicker {
    /// The WebSocket topic for the event (e.g., "tickers.BTCUSDT").
    ///
    /// Specifies the data stream for the ticker update. Bots use this to verify the correct market.
    pub topic: String,

    /// The event type (e.g., "snapshot").
    ///
    /// Indicates the type of ticker update, typically a snapshot of current market data. Bots use this to process ticker data appropriately.
    #[serde(rename = "type")]
    pub event_type: String,

    /// The ticker data.
    ///
    /// Contains market-specific ticker metrics, such as last price and open interest. Bots use this to monitor market conditions and generate trading signals.
    pub data: Ticker,

    /// The checksum for the ticker data.
    ///
    /// A sequence number or checksum to verify data integrity. Bots use this to ensure the ticker data is consistent and not corrupted.
    pub cs: u64,

    /// The timestamp of the event in milliseconds.
    ///
    /// Indicates when the ticker update was generated. Bots use this to ensure data freshness and align with other market data.
    pub ts: u64,
}
