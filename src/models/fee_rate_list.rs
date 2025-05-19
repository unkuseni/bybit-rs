use crate::prelude::*;

/// Summarizes fee rate data for trading pairs.
///
/// Part of the `FeeRateResponse`, this struct contains a list of fee rate records for each trading pair. Bots use this to optimize trading strategies based on fee structures.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FeeRateList {
    /// A list of fee rate records.
    ///
    /// Contains maker and taker fee rates for each trading pair. Bots use this to calculate trading costs and optimize order placement.
    pub list: Vec<FeeRate>,
}
