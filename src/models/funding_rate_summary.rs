use crate::prelude::*;

/// Summarizes funding rate history for a trading pair.
///
/// Part of the `FundingRateResponse`, this struct organizes funding rate data by category and a list of funding rate records. Bots use this to analyze funding cost trends.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FundingRateSummary {
    /// The product category (e.g., "linear").
    ///
    /// Indicates the instrument type. Bots should verify this matches their strategy.
    pub category: String,
    /// A list of funding rate records.
    ///
    /// Contains the actual funding rate data. Bots iterate over this list to analyze historical funding costs.
    pub list: Vec<FundingRate>,
}
