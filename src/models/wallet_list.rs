use crate::prelude::*;

/// Summarizes wallet balance data for an account.
///
/// Part of the `WalletResponse`, this struct contains a list of wallet data entries for different account types or currencies. Bots use this to monitor overall account health and margin availability.
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct WalletList {
    /// A list of wallet data entries.
    ///
    /// Contains detailed balance and margin information for each account type or currency. Bots use this to calculate available margin and manage risk across assets.
    pub list: Vec<WalletData>,
}
