use crate::prelude::*;

/// Summarizes the list of amended orders.
///
/// Part of the `BatchAmendResponse`, this struct contains the details of all orders amended in the batch request. Bots use this to confirm amendment success and update their state.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AmendedOrderList {
    /// A list of amended orders.
    ///
    /// Contains details like order IDs and symbols for each successfully amended order. Bots should iterate through this to update their internal order book.
    pub list: Vec<AmendedOrder>,
}
