use crate::prelude::*;

/// Represents a single k-line (candlestick) data point.
///
/// This struct contains the price, volume, and time data for a single candlestick, used for technical analysis in trading bots.
///
/// # Bybit API Reference
/// Part of the k-line WebSocket stream (https://bybit-exchange.github.io/docs/v5/websocket/public/kline).
///
/// # Perpetual Futures Context
/// Candlestick data is critical for strategies like moving averages, RSI, or Bollinger Bands. Bots rely on this data to make real-time trading decisions.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KlineData {
    /// The start time of the candlestick (in milliseconds).
    ///
    /// Marks the beginning of the k-line interval. Bots use this to align candlesticks with other time-based data.
    pub start: u64,
    /// The end time of the candlestick (in milliseconds).
    ///
    /// Marks the end of the k-line interval. Bots verify this to ensure data continuity.
    pub end: u64,
    /// The time interval of the candlestick (e.g., "1m", "1h").
    ///
    /// Specifies the timeframe (e.g., 1 minute, 1 hour). Bots select intervals based on strategy requirements (e.g., short-term vs. long-term trading).
    pub interval: String,
    /// The opening price of the candlestick.
    ///
    /// The price at the start of the interval. Bots use this for calculating price changes and patterns.
    #[serde(with = "string_to_float")]
    pub open: f64,
    /// The closing price of the candlestick.
    ///
    /// The price at the end of the interval. Critical for determining candlestick direction (bullish or bearish).
    #[serde(with = "string_to_float")]
    pub close: f64,
    /// The highest price during the interval.
    ///
    /// Used to identify resistance levels or breakout points in technical analysis.
    #[serde(with = "string_to_float")]
    pub high: f64,
    /// The lowest price during the interval.
    ///
    /// Used to identify support levels or reversal points.
    #[serde(with = "string_to_float")]
    pub low: f64,
    /// The trading volume during the interval (in base currency).
    ///
    /// Indicates market activity. High volume on price movements can confirm trends for bots.
    #[serde(with = "string_to_float")]
    pub volume: f64,
    /// The turnover (value of trades) during the interval (in quote currency).
    ///
    /// Represents the monetary value of trades. Bots use this to assess liquidity and market interest.
    #[serde(with = "string_to_float")]
    pub turnover: f64,
    /// Whether the candlestick is confirmed (closed).
    ///
    /// A `true` value indicates the candlestick is finalized. Bots should wait for confirmation before acting on signals to avoid premature trades.
    pub confirm: bool,
    /// The timestamp of the candlestick (in milliseconds).
    ///
    /// Typically aligns with `end`. Bots use this for precise timing in strategies.
    pub timestamp: u64,
}
