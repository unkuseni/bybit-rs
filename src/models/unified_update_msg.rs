use crate::prelude::*;

/// Contains messages related to a UTA status update.
///
/// Part of the `UTAUpdateStatus`, this struct holds a list of messages providing context for the unified margin update. Bots use this to diagnose issues or confirm successful configuration.
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct UnifiedUpdateMsg {
    /// A list of update messages.
    ///
    /// Contains human-readable messages describing the update outcome or any warnings. Bots should log these for auditing and user feedback.
    pub msg: Vec<String>,
}
