use crate::prelude::*;

/// Structure for WebSocket trade update events.
///
/// Contains real-time updates on executed trades for a trading pair. Bots use this for price discovery, trade signal generation, and market momentum analysis in perpetual futures trading.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TradeUpdate {
    /// The WebSocket topic for the event (e.g., "trade.BTCUSDT").
    ///
    /// Specifies the data stream for the trade update. Bots use this to verify the correct market.
    pub topic: String,
    /// The event type (e.g., "snapshot").
    ///
    /// Indicates the type of trade update, typically a snapshot of recent trades. Bots use this to process trade data appropriately.
    #[serde(rename = "type")]
    pub event_type: String,
    /// The timestamp of the event in milliseconds.
    ///
    /// Indicates when the trade update was generated. Bots use this to ensure data freshness and align with other market data.
    #[serde(rename = "ts")]
    pub timestamp: u64,
    /// A list of trade details.
    ///
    /// Contains the executed trades, including price, volume, and side. Bots use this to analyze market activity and generate trading signals.
    pub data: Vec<WsTrade>,
}
