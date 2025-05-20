use crate::prelude::*;

/// Summarizes the list of batched orders.
///
/// Part of the `BatchPlaceResponse`, this struct contains the details of all orders placed in the batch request. Bots use this to confirm order creation and update their state.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BatchedOrderList {
    /// A list of placed orders.
    ///
    /// Contains details like order IDs and symbols for each successfully placed order. Bots should iterate through this to update their internal order book.
    pub list: Vec<BatchedOrder>,
}
