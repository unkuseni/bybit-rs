use crate::prelude::*;

/// Summarizes open interest data for a trading pair.
///
/// Part of the `OpenInterestResponse`, this struct organizes open interest data by symbol and a list of records. It’s the primary container for open interest data used by bots to assess market dynamics in perpetual futures.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OpenInterestSummary {
    /// The trading pair symbol (e.g., "BTCUSDT").
    ///
    /// Confirms the perpetual futures contract for the open interest data. Bots should verify this matches the requested symbol.
    pub symbol: String,

    /// A list of open interest records.
    ///
    /// Contains the open interest data for each time interval, including quantity and timestamp. Bots use this to analyze trends in market participation and predict potential price volatility.
    pub list: Vec<OpenInterest>,
}
