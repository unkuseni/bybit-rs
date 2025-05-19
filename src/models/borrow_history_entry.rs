use crate::prelude::*;

/// Represents a single borrowing record.
///
/// Details a specific borrowing event, including the amount borrowed and associated costs. Bots use this to calculate interest expenses and optimize leverage strategies.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BorrowHistoryEntry {
    /// The amount borrowed.
    ///
    /// The quantity of the currency borrowed (e.g., USDT). Bots use this to track borrowing activity and calculate interest costs.
    #[serde(with = "string_to_float")]
    pub borrow_amount: f64,
    /// The cost exemption status.
    ///
    /// Indicates any exemptions applied to borrowing costs (e.g., promotional rates). Bots can use this to adjust cost calculations if applicable.
    pub cost_exemption: String,
    /// The free borrowed amount.
    ///
    /// The portion of the borrowed amount that is interest-free, if any. Bots use this to optimize borrowing strategies by prioritizing free borrowing.
    #[serde(with = "string_to_float")]
    pub free_borrowed_amount: f64,
    /// The timestamp when the borrowing occurred.
    ///
    /// Indicates when the borrowing event took place. Bots use this to align borrowing data with other time-series data.
    pub created_time: u64,
    /// The interest-bearing borrow size.
    ///
    /// The portion of the borrowed amount that incurs interest. Bots use this to calculate interest expenses accurately.
    #[serde(with = "string_to_float")]
    pub interest_bearing_borrow_size: f64,
    /// The currency of the borrowed amount (e.g., "USDT").
    ///
    /// Specifies the currency used for borrowing. Bots should verify this matches the expected currency for their margin type.
    pub currency: String,
    /// The unrealized loss from borrowing.
    ///
    /// Any unrealized losses associated with the borrowed funds, typically due to market movements. Bots use this to assess the impact of borrowing on account health.
    #[serde(with = "string_to_float")]
    pub unrealised_loss: f64,
    /// The hourly borrow rate.
    ///
    /// The interest rate applied to the borrowed amount per hour, as a decimal (e.g., 0.0001 for 0.01%). Bots use this to calculate borrowing costs over time.
    #[serde(with = "string_to_float")]
    pub hourly_borrow_rate: f64,
    /// The total borrow cost.
    ///
    /// The cumulative interest cost for the borrowed amount. Bots use this to track expenses and optimize leverage usage.
    #[serde(with = "string_to_float")]
    pub borrow_cost: f64,
}
