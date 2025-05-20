use crate::prelude::*;

/// Represents a WebSocket k-line (candlestick) update for a trading pair.
///
/// K-lines provide historical price and volume data over a specific interval (e.g., 1 minute, 1 hour). This struct is used in Bybit's WebSocket streams to deliver real-time candlestick updates for perpetual futures.
///
/// # Bybit API Reference
/// The Bybit API (https://bybit-exchange.github.io/docs/v5/websocket/public/kline) provides k-line data via WebSocket, including open, close, high, low, volume, and turnover.
///
/// # Perpetual Futures Context
/// K-lines are fundamental for technical analysis in futures trading. Bots use candlestick patterns to generate trading signals, such as breakouts, reversals, or trend continuations.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WsKline {
    /// The WebSocket topic (e.g., "kline.1m.BTCUSDT").
    ///
    /// Identifies the k-line stream and interval. Bots use this to subscribe to specific timeframes and symbols.
    pub topic: String,

    /// The k-line data for the update.
    ///
    /// Contains a vector of candlestick data points. Bots process this to update technical indicators or chart patterns.
    pub data: Vec<KlineData>,

    /// The timestamp of the update (in milliseconds).
    ///
    /// Indicates when the k-line data was sent. Bots use this to ensure data is processed in chronological order.
    #[serde(rename = "ts")]
    pub timestamp: u64,

    /// The type of WebSocket event (e.g., "snapshot" or "delta").
    ///
    /// Specifies whether the data is a full snapshot or incremental update. Bots must handle both to maintain accurate k-line history.
    #[serde(rename = "type")]
    pub event_type: String,
}
