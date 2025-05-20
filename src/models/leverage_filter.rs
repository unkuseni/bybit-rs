use crate::prelude::*;

/// Leverage constraints for a futures instrument.
///
/// Specifies the allowable leverage settings for a perpetual futures contract. Bots must adhere to these to set valid leverage levels and manage risk.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LeverageFilter {
    /// The minimum leverage allowed.
    ///
    /// The lowest leverage setting for the contract (e.g., `"1"` for 1x). Bots must ensure leverage settings are at least this value to avoid API errors. Low leverage reduces risk but also potential returns.
    pub min_leverage: String,

    /// The maximum leverage allowed.
    ///
    /// The highest leverage setting (e.g., `"100"` for 100x). High leverage amplifies gains and losses in perpetual futures, increasing liquidation risk. Bots should use this to cap leverage based on risk tolerance.
    pub max_leverage: String,

    /// The leverage step size.
    ///
    /// The increment for leverage adjustments (e.g., `"0.1"`). Bots must set leverage in multiples of this step to comply with Bybit’s rules.
    pub leverage_step: String,
}
