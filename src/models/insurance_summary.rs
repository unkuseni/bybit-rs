use crate::prelude::*;

/// Summarizes the insurance fund data for Bybit’s perpetual futures.
/// The insurance fund absorbs losses when a trader’s position is liquidated below the bankruptcy
/// price (bust price), preventing auto-deleveraging of other traders.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InsuranceSummary {
    /// Timestamp of the last update to the insurance fund (Unix epoch in milliseconds).
    /// Bots use this to verify the recency of the data, as outdated fund balances could signal risk.
    #[serde(with = "string_to_u64")]
    pub updated_time: u64,
    /// List of insurance fund entries for different coins.
    /// Each entry details the fund’s balance for a specific cryptocurrency, used to assess
    /// Bybit’s capacity to cover losses in perpetual futures.
    pub list: Vec<Insurance>,
}
