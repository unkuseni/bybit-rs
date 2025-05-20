use crate::prelude::*;

/// Represents a delivery price record for a futures contract.
/// Details the settlement price and time, primarily for non-perpetual futures.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryPrice {
    /// The trading symbol (e.g., "BTCUSDT").
    /// Specifies the market for the delivery price. Bots use this to align with trading pairs.
    pub symbol: String,
    /// The settlement price at delivery (e.g., "50000").
    /// The price at which the contract settles. For perpetual futures, this is less relevant
    /// but may be used for benchmarking or strategy backtesting.
    pub delivery_price: String,
    /// Timestamp of the delivery (Unix epoch in milliseconds).
    /// Marks when the contract was settled. Bots use this for time-series analysis.
    #[serde(with = "string_to_u64")]
    pub delivery_time: u64,
}
