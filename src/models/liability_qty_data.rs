use crate::prelude::*;

/// Represents a single repayment record for a currency.
///
/// Details the amount repaid for a specific currency. Bots use this to confirm repayment amounts and update account balance calculations.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LiabilityQtyData {
    /// The currency of the repayment (e.g., "USDT").
    ///
    /// Specifies the currency used for the repayment. Bots should verify this matches the expected currency.
    pub coin: String,

    /// The quantity repaid.
    ///
    /// The amount of the currency repaid to reduce the borrowed liability. Bots use this to update liability tracking and account balances.
    #[serde(with = "string_to_float")]
    pub repayment_qty: f64,
}
