use crate::prelude::*;

/// Represents a long/short ratio record for a trading symbol.
/// Shows the proportion of long vs. short positions, indicating market sentiment.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LongShortRatio {
    /// The trading symbol (e.g., "BTCUSDT").
    /// Specifies the market for the ratio. Bots use this to align with trading pairs.
    pub symbol: String,
    /// The ratio of long positions (e.g., 0.6 for 60%).
    /// A higher ratio indicates bullish sentiment. Bots use this for sentiment-based strategies,
    /// such as fading extreme ratios.
    #[serde(with = "string_to_float")]
    pub buy_ratio: f64,
    /// The ratio of short positions (e.g., 0.4 for 40%).
    /// A higher ratio indicates bearish sentiment. Bots combine this with `buy_ratio` to assess
    /// market bias and potential reversals.
    #[serde(with = "string_to_float")]
    pub sell_ratio: f64,
    /// Timestamp of the ratio data (Unix epoch in milliseconds).
    /// Marks when the ratio was recorded. Bots use this to align with market events.
    #[serde(with = "string_to_u64")]
    pub timestamp: u64,
}
