use crate::prelude::*;

/// Represents the confirmation status of a single order in a batch.
///
/// Details the success or failure of an individual order in a batch request. Bots use this to diagnose issues with specific orders and implement retry logic if needed.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderConfirmation {
    /// The confirmation code for the order.
    ///
    /// A `0` indicates success, while non-zero values indicate errors (e.g., `10001` for invalid parameters). Bots should check this to identify failed orders.
    pub code: i16,

    /// A human-readable message describing the confirmation result.
    ///
    /// Provides details about the order’s status, such as `"OK"` or an error description. Bots should log this for debugging and error handling.
    pub msg: String,
}
