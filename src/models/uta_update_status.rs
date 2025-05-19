use crate::prelude::*;

/// Details the result of a UTA status update.
///
/// Part of the `UTAResponse`, this struct provides the updated unified margin status and any associated messages. Bots use this to confirm the account’s margin configuration.
#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UTAUpdateStatus {
    /// The unified margin update status.
    ///
    /// Indicates the new status of the unified margin setting (e.g., "Enabled"). Bots should verify this matches the requested configuration.
    pub unified_update_status: String,
    /// Additional messages related to the update.
    ///
    /// Contains any warnings or informational messages about the status change. Bots should log these for auditing and debugging.
    pub unified_update_msg: UnifiedUpdateMsg,
}
