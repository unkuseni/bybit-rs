use crate::prelude::*;

/// Summarizes closed P&L data for closed positions.
///
/// Part of the `ClosedPnlResponse`, this struct organizes P&L data by category, pagination cursor, and a list of P&L records. Bots use this to process realized P&L and calculate performance metrics.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClosedPnlResult {
    /// The cursor for pagination (optional).
    ///
    /// Indicates the next page of results for large datasets. Bots should use this for paginated requests to fetch additional P&L data when `limit` is reached.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,

    /// The product category (e.g., "linear").
    ///
    /// Indicates the instrument type. Bots should verify this matches the requested `category` to ensure data relevance.
    pub category: Category,

    /// A list of closed P&L records.
    ///
    /// Contains detailed P&L data for each closed position, such as realized profit and trade size. Bots use this to calculate performance metrics and refine strategies.
    pub list: Vec<ClosedPnlItem>,
}
