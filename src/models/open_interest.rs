use crate::prelude::*;

/// Represents a single open interest record.
///
/// Each record provides the open interest (total outstanding contracts) for a specific time interval in a perpetual futures contract. Bots use this to assess market sentiment, as rising open interest may indicate new positions and potential price momentum, while declining open interest may signal position unwinding.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OpenInterest {
    /// The open interest quantity (in base asset).
    ///
    /// The total number of outstanding contracts in the base asset (e.g., BTC in `BTCUSDT`). Bots use this to gauge market participation, as high open interest suggests strong liquidity and potential for larger price moves in perpetual futures. Parse to `f64` for calculations.
    #[serde(with = "string_to_float")]
    pub open_interest: f64,
    /// The timestamp of the open interest record (Unix timestamp in milliseconds).
    ///
    /// Indicates when the open interest was recorded. Bots use this to align open interest data with price or trade data for time-series analysis and to track changes over time.
    #[serde(with = "string_to_u64")]
    pub timestamp: u64,
}
