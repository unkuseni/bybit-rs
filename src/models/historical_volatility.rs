use crate::prelude::*;

/// Represents a single historical volatility data point.
/// Provides volatility metrics for a specific time and period, used to gauge price fluctuation
/// risks in perpetual futures trading.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalVolatility {
    /// The time period for which volatility is calculated (e.g., 7 for 7 days).
    /// In perpetual futures, this indicates the lookback window for the volatility metric.
    /// Bots use this to align with trading strategy timeframes.
    pub period: u64,

    /// The calculated volatility value (e.g., 0.25 for 25% annualized volatility).
    /// Volatility is often expressed as a percentage, representing the standard deviation of
    /// returns. High volatility implies greater risk, impacting leverage decisions in bots.
    #[serde(with = "string_to_float")]
    pub value: f64,

    /// The timestamp of the volatility data point (Unix epoch in milliseconds).
    /// Marks when the volatility was recorded. Bots use this to sequence data and align with
    /// market events or other time-series data.
    #[serde(rename = "time", with = "string_to_u64")]
    pub timestamp: u64,
}
