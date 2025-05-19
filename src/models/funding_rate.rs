use crate::prelude::*;

/// Represents a single funding rate record.
///
/// Each record provides the funding rate and timestamp for a specific funding interval in a perpetual futures contract. Funding rates balance long and short positions by charging a fee, paid every funding interval (typically 8 hours). Bots use this data to analyze historical funding costs, optimize position timing, and estimate profitability, as positive rates mean longs pay shorts, while negative rates mean shorts pay longs.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FundingRate {
    /// The trading pair symbol (e.g., "BTCUSDT").
    ///
    /// Identifies the perpetual futures contract for this funding rate record. Bots should verify this matches the requested symbol to ensure data integrity.
    pub symbol: String,
    /// The funding rate for the interval.
    ///
    /// The rate applied during the funding interval, expressed as a decimal (e.g., `0.0001` for 0.01%). Positive rates indicate longs pay shorts; negative rates indicate shorts pay longs. Bots must parse this to `f64` for calculations and use it to estimate funding costs, which can significantly impact long-term position profitability in perpetual futures.
    #[serde(with = "string_to_float")]
    pub funding_rate: f64,
    /// The timestamp of the funding rate application (Unix timestamp in milliseconds).
    ///
    /// Indicates when the funding rate was applied. Bots use this to align funding rate data with other time-series data (e.g., price or position data) and to reconstruct the funding cost history for backtesting or strategy optimization.
    #[serde(with = "string_to_u64")]
    pub funding_rate_timestamp: u64,
}
