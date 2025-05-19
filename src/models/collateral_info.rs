use crate::prelude::*;

/// Represents collateral information for a single coin.
///
/// Details the collateral eligibility, borrowing limits, and rates for a specific coin. Bots use this to determine which coins to use as collateral and manage borrowing costs.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CollateralInfo {
    /// The amount available to borrow for the coin.
    ///
    /// The total borrowing capacity for the coin, in the coin’s units. Bots use this to assess borrowing potential for margin trading.
    pub available_to_borrow: String,
    /// The free borrowing amount available.
    ///
    /// The portion of the borrowing capacity that is interest-free, if any. Bots use this to optimize borrowing by prioritizing free borrowing.
    #[serde(with = "string_to_float")]
    pub free_borrowing_amount: f64,
    /// The free borrow amount limit.
    ///
    /// The maximum interest-free borrowing limit for the coin. Bots use this to plan borrowing strategies within free limits.
    #[serde(with = "string_to_float")]
    pub free_borrow_amount: f64,
    /// The maximum borrowing amount for the coin.
    ///
    /// The total borrowing limit for the coin, including interest-bearing amounts. Bots use this to ensure borrowing stays within account limits.
    #[serde(with = "string_to_float")]
    pub max_borrowing_amount: f64,
    /// The hourly borrow rate for the coin.
    ///
    /// The interest rate applied to borrowed amounts per hour, as a decimal (e.g., 0.0001 for 0.01%). Bots use this to calculate borrowing costs.
    #[serde(with = "string_to_float")]
    pub hourly_borrow_rate: f64,
    /// The borrow usage rate.
    ///
    /// The percentage of the borrowing limit currently utilized. Bots use this to monitor borrowing capacity and avoid over-leveraging.
    #[serde(with = "string_to_float")]
    pub borrow_usage_rate: f64,
    /// Whether the coin is enabled as collateral.
    ///
    /// `true` if the coin can be used as collateral for margin trading, `false` otherwise. Bots use this to select appropriate collateral assets.
    pub collateral_switch: bool,
    /// The current borrowed amount for the coin.
    ///
    /// The total amount of the coin currently borrowed. Bots use this to track liabilities and calculate interest costs.
    #[serde(with = "string_to_float")]
    pub borrow_amount: f64,
    /// Whether the coin is borrowable.
    ///
    /// `true` if the coin can be borrowed for margin trading, `false` otherwise. Bots use this to determine borrowing eligibility.
    pub borrowable: bool,
    /// The currency of the coin (e.g., "USDT").
    ///
    /// Specifies the coin type. Bots should verify this matches the expected currency for their margin type.
    pub currency: String,
    /// Whether the coin is used as margin collateral.
    ///
    /// `true` if the coin is actively used as collateral, `false` otherwise. Bots use this to confirm collateral allocation.
    pub margin_collateral: bool,
    /// The free borrowing limit as a string.
    ///
    /// The maximum interest-free borrowing limit for the coin, represented as a string. Bots can use this for precise borrowing calculations.
    pub free_borrowing_limit: String,
    /// The collateral ratio for the coin.
    ///
    /// The ratio at which the coin contributes to margin requirements (e.g., "0.9" for 90%). Bots use this to calculate effective collateral value.
    pub collateral_ratio: String,
}
