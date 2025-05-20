use crate::prelude::*;

/// Summarizes collateral information for an account.
///
/// Part of the `CollateralInfoResponse`, this struct contains a list of collateral details for each coin. Bots use this to assess collateral eligibility and borrowing capabilities.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CollateralInfoList {
    /// A list of collateral information records.
    ///
    /// Contains detailed data for each coin, such as borrowing limits and collateral status. Bots use this to optimize collateral allocation and manage margin.
    pub list: Vec<CollateralInfo>,
}
