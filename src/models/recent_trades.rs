use crate::prelude::*;

/// Summarizes recent trading records for a trading pair.
///
/// Part of the `RecentTradesResponse`, this struct organizes recent trade data by category and a list of trade records. It’s the primary container for trade data used by bots to analyze market activity in perpetual futures.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RecentTrades {
    /// The product category (e.g., "linear").
    ///
    /// Indicates the instrument type (e.g., `linear` for USDT-margined perpetuals). Bots should verify this matches the requested `category` to ensure data relevance.
    pub category: String,
    /// A list of recent trade records.
    ///
    /// Contains the executed trade details, such as price, quantity, and side (buy/sell). Bots use this to analyze trade volume, price momentum, and market liquidity for perpetual futures.
    pub list: Vec<RecentTrade>,
}
