use crate::prelude::*;

/// Represents a single Index Price Kline record.
///
/// Each record provides index price data for a time interval. In perpetual futures, the index price reflects the spot market, helping bots predict mark price movements and funding rate changes.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IndexPriceKline {
    /// The start time of the candlestick (Unix timestamp in milliseconds).
    ///
    /// Marks the beginning of the time interval. Bots use this to align index price data with other data sources.
    #[serde(with = "string_to_u64")]
    pub start_time: u64,

    /// The opening index price of the candlestick.
    ///
    /// The index price at the start of the interval. Used to track spot market trends. Parse to `f64`.
    pub open_price: String,

    /// The highest index price during the candlestick interval.
    ///
    /// Indicates the peak index price. Useful for understanding spot market volatility. Parse to `f64`.
    pub high_price: String,

    /// The lowest index price during the candlestick interval.
    ///
    /// Indicates the trough index price. Helps bots assess spot market support levels. Parse to `f64`.
    pub low_price: String,

    /// The closing index price of the candlestick.
    ///
    /// The index price at the end of the interval. Critical for predicting mark price and funding rate trends. Parse to `f64`.
    pub close_price: String,
}
