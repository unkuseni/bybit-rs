use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SwitchList {
    /// A list of collateral coin status updates.
    ///
    /// Contains details of each coin’s collateral setting (enabled or disabled). Bots use this to verify the new collateral configuration.
    pub list: Vec<SwitchListData>,
}
