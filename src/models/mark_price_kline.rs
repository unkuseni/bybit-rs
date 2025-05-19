use crate::prelude::*;

/// Represents a single Mark Price Kline record.
///
/// Each record provides mark price data for a time interval. In perpetual futures, the mark price is a smoothed price used to calculate funding rates and trigger liquidations, making this data critical for risk management in trading bots.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MarkPriceKline {
    /// The start time of the candlestick (Unix timestamp in milliseconds).
    ///
    /// Marks the beginning of the time interval. Bots use this to align mark price data with other time-series data (e.g., regular Klines).
    #[serde(with = "string_to_u64")]
    pub start_time: u64,
    /// The opening mark price of the candlestick.
    ///
    /// The mark price at the start of the interval. Used to track funding rate trends and liquidation thresholds. Parse to `f64` for calculations.
    pub open_price: String,
    /// The highest mark price during the candlestick interval.
    ///
    /// Indicates the peak mark price. Useful for assessing potential liquidation risks during volatile periods. Parse to `f64`.
    pub high_price: String,
    /// The lowest mark price during the candlestick interval.
    ///
    /// Indicates the trough mark price. Helps bots identify safe price levels relative to liquidation thresholds. Parse to `f64`.
    pub low_price: String,
    /// The closing mark price of the candlestick.
    ///
    /// The mark price at the end of the interval. Critical for real-time funding rate calculations and liquidation monitoring. Parse to `f64`.
    pub close_price: String,
}
