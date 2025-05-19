use crate::prelude::*;

/// Summarizes repayment quantities for borrowed liabilities.
///
/// Part of the `RepayLiabilityResponse`, this struct contains a list of repayment details for each currency. Bots use this to confirm the amounts repaid and update account status.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LiabilityQty {
    /// A list of repayment records.
    ///
    /// Contains details of the repaid amounts for each currency. Bots use this to verify repayment completion and update liability tracking.
    pub list: Vec<LiabilityQtyData>,
}
