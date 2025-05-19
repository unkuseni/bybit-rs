use crate::prelude::*;

/// Summarizes the list of canceled orders.
///
/// Part of the `CancelAllResponse`, this struct contains the details of all orders canceled in the request. Bots use this to confirm cancellation success and update their order tracking systems.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CancelledList {
    /// A list of canceled order statuses.
    ///
    /// Contains details like order IDs and symbols for each canceled order. Bots should iterate through this to update their internal order book and ensure no orphaned orders remain.
    pub list: Vec<OrderStatus>,
}
