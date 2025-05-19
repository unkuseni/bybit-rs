use crate::prelude::*;

/// Represents a single collateral coin status update.
///
/// Details the collateral status for a specific coin in a batch setting request. Bots use this to confirm the outcome for each coin.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SwitchListData {
    /// The currency of the coin (e.g., "USDT").
    ///
    /// Specifies the coin whose collateral status was updated. Bots should verify this matches the requested coin.
    pub coin: String,
    /// The collateral switch status (e.g., "ON", "OFF").
    ///
    /// Indicates whether the coin is enabled (`ON`) or disabled (`OFF`) as collateral. Bots use this to confirm the new collateral setting.
    pub collateral_switch: String,
}
