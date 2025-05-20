use crate::prelude::*;

/// Response structure for batch order cancellation.
///
/// Returned by the `/v5/order/cancel-batch` endpoint, this struct provides the result of canceling multiple orders. Bots use this to confirm successful cancellations and handle any errors for individual orders.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BatchCancelResponse {
    /// The return code indicating the overall success or failure of the request.
    ///
    /// A `0` indicates success for the batch request, but individual cancellations may have errors. Bots should check `ret_ext_info` for per-order results.
    pub ret_code: i32,

    /// A human-readable message describing the overall result or error.
    ///
    /// Typically `"OK"` for success or an error description. Bots should log this for debugging.
    pub ret_msg: String,

    /// The list of canceled orders.
    ///
    /// Contains details of successfully canceled orders, including order IDs and symbols. Bots use this to update their order tracking systems.
    pub result: CanceledOrderList,

    /// Additional information about individual order confirmations.
    ///
    /// Contains per-order success or failure details, including error codes and messages. Bots should inspect this to handle failed cancellations and retry or adjust as needed.
    pub ret_ext_info: OrderConfirmationList,

    /// The timestamp of the response in milliseconds.
    ///
    /// Indicates when the response was generated. Bots can use this to ensure data freshness and measure latency.
    pub time: u64,
}
