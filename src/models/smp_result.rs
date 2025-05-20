use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SmpResult {
    /// The SMP group ID.
    ///
    /// Identifies the Self-Managed Portfolio group for the account, used for copy trading or portfolio management. Bots use this to apply group-specific rules.
    pub smp_group: u8,
}
