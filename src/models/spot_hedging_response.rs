use crate::prelude::*;

/// Response structure for spot hedging status requests.
///
/// Returned by the `/v5/account/set-spot-hedging` endpoint, this struct confirms the result of enabling or disabling spot hedging. Bots use this to verify successful configuration and handle errors for hedging strategies.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SpotHedgingResponse {
    /// The return code indicating the success or failure of the request.
    ///
    /// A `0` indicates success. Non-zero codes require error handling for issues like invalid settings. Bots should implement retry logic or parameter adjustments.
    pub ret_code: i32,

    /// A human-readable message describing the result or error.
    ///
    /// Provides context for the `ret_code`, such as `"OK"` or an error description. Bots should log this for debugging.
    pub ret_msg: String,
}
