use crate::prelude::*;

/// Details the result of a margin mode change.
///
/// Part of the `SetMarginModeResponse`, this struct provides reasons for the success or failure of the margin mode change. Bots use this to diagnose configuration issues.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MarginModeResult {
    /// A list of reasons for the margin mode change outcome.
    ///
    /// Contains detailed explanations for success or failure, such as restrictions due to open positions. Bots use this to handle errors and adjust settings.
    pub reason: Vec<ReasonObject>,
}
