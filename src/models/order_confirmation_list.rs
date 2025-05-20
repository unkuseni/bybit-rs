use crate::prelude::*;

/// Summarizes the confirmation status of batched orders.
///
/// Part of the `BatchPlaceResponse`, this struct contains confirmation details for each order in the batch, including success or failure statuses. Bots use this to handle errors for individual orders.
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct OrderConfirmationList {
    /// A list of order confirmations.
    ///
    /// Contains the status (success or failure) for each order in the batch. Bots should inspect this to identify and handle failed orders.
    pub list: Vec<OrderConfirmation>,
}
