use crate::prelude::*;

/// Represents a single reason for a margin mode change outcome.
///
/// Part of the `MarginModeResult`, this struct details a specific reason for the success or failure of the margin mode change. Bots use this for error handling and debugging.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReasonObject {
    /// The reason code for the outcome.
    ///
    /// A code indicating the specific reason for success or failure (e.g., "10001" for invalid settings). Bots use this to implement targeted error handling.
    pub reason_code: String,

    /// A human-readable message describing the reason.
    ///
    /// Provides context for the `reason_code`, such as `"Cannot change mode with open positions"`. Bots should log this for debugging and user feedback.
    pub reason_msg: String,
}
