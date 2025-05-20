use crate::prelude::*;

/// Summarizes position information data.
///
/// Part of the `InfoResponse`, this struct organizes position data by category, pagination cursor, and a list of position records. Bots use this to process and monitor open positions.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InfoResult {
    /// A list of position records.
    ///
    /// Contains detailed position data, such as size, entry price, and P&L. Bots use this to calculate risk metrics and update position tracking systems.
    pub list: Vec<PositionInfo>,

    /// The cursor for pagination (optional).
    ///
    /// Indicates the next page of results for large datasets. Bots should use this for paginated requests to fetch additional position data when `limit` is reached.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,

    /// The product category (e.g., "linear").
    ///
    /// Indicates the instrument type. Bots should verify this matches the requested `category` to ensure data relevance.
    pub category: String,
}
