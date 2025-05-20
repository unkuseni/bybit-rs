use crate::prelude::*;

/// Summarizes long/short ratio data for a market.
/// Provides insights into trader positioning in perpetual futures.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LongShortRatioSummary {
    /// List of long/short ratio records.
    /// Each entry details the balance of long vs. short positions, used by bots for
    /// contrarian or momentum strategies.
    pub list: Vec<LongShortRatio>,
}
