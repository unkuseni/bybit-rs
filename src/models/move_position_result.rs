use crate::prelude::*;

/// Details the result of a position move between accounts.
///
/// Part of the `MovePositionResponse`, this struct provides the outcome of the position transfer, including identifiers and status. Bots use this to confirm the transfer and track its execution.
#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MovePositionResult {
    /// The block trade ID for the position move.
    ///
    /// A unique identifier for the transfer, treated as a block trade by Bybit. Bots use this to track the specific transfer event.
    pub block_trade_id: String,
    /// The status of the position move.
    ///
    /// Indicates whether the transfer was successful (e.g., "Filled") or encountered issues (e.g., "Rejected"). Bots should check this to confirm transfer completion.
    pub status: String,
    /// The party that rejected the move, if applicable.
    ///
    /// Identifies the account (source or destination) that caused a rejection, if any. Bots use this for error handling and to diagnose transfer failures.
    pub reject_party: String,
}
