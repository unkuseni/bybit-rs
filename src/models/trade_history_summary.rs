use crate::prelude::*;

/// Summarizes trade history data for a trading pair.
///
/// Part of the `TradeHistoryResponse`, this struct organizes trade history by category, pagination cursor, and a list of trade records. Bots use this to process executed trades and calculate metrics like average execution price or total fees.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TradeHistorySummary {
    /// The cursor for pagination.
    ///
    /// Indicates the next page of results for large datasets. Bots should use this for paginated requests to fetch additional trade history when `limit` is reached.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub next_page_cursor: String,

    /// The product category (e.g., "linear").
    ///
    /// Indicates the instrument type (e.g., `linear` for USDT-margined perpetuals). Bots should verify this matches the requested `category` to ensure data relevance.
    pub category: Category,

    /// A list of trade history records.
    ///
    /// Contains detailed execution data for each trade, such as price, quantity, and fees. Bots use this to analyze trade performance and refine strategies.
    pub list: Vec<TradeHistory>,
}
