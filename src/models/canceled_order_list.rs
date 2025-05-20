use crate::prelude::*;

/// Summarizes the list of canceled orders in a batch.
///
/// Part of the `BatchCancelResponse`, this struct contains the details of all orders canceled in the batch request. Bots use this to confirm cancellation success and update their state.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CanceledOrderList {
    /// A list of canceled orders.
    ///
    /// Contains details like order IDs and symbols for each successfully canceled order. Bots should iterate through this to update their internal order book.
    pub list: Vec<CanceledOrder>,
}
