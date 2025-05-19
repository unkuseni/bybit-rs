use crate::prelude::*;

/// Represents a single Premium Index Price Kline record.
///
/// Each record provides premium index price data for a time interval. This data helps bots understand the funding rate’s directional pressure in perpetual futures.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PremiumIndexPriceKline {
    /// The start time of the candlestick (Unix timestamp in milliseconds).
    ///
    /// Marks the beginning of the time interval. Bots use this for data alignment.
    #[serde(with = "string_to_u64")]
    pub start_time: u64,
    /// The opening premium index price of the candlestick.
    ///
    /// The premium/discount at the start of the interval. A positive value indicates a premium, driving positive funding rates. Parse to `f64`.
    pub open_price: String,
    /// The highest premium index price during the candlestick interval.
    ///
    /// Indicates the peak premium/discount. Useful for assessing funding rate volatility. Parse to `f64`.
    pub high_price: String,
    /// The lowest premium index price during the candlestick interval.
    ///
    /// Indicates the trough premium/discount. Helps bots evaluate funding rate stability. Parse to `f64`.
    pub low_price: String,
    /// The closing premium index price of the candlestick.
    ///
    /// The premium/discount at the end of the interval. Critical for real-time funding rate predictions. Parse to `f64`.
    pub close_price: String,
}
