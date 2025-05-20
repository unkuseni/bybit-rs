use crate::prelude::*;

/// Represents a collection of historical orders with pagination information.
///
/// This struct contains the list of orders and a cursor for fetching additional pages.
/// In perpetual futures, bots use this to analyze trading history, calculate performance
/// metrics, and adjust strategies based on past executions.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderHistory {
    /// The category of the orders (e.g., "spot", "linear").
    ///
    /// Indicates the market of the orders (e.g., Spot or Linear for perpetual futures).
    /// Bots should verify this matches the query to ensure correct data processing.
    pub category: Category,

    /// The list of historical orders.
    ///
    /// Contains detailed information about each order. Bots should iterate over this
    /// to extract metrics like execution price, fees, and status for perpetual futures
    /// analysis.
    pub list: Vec<Order>,

    /// The cursor for fetching the next page of results.
    ///
    /// Used for pagination when the query returns more orders than the `limit`. Bots
    /// should include this in subsequent requests to retrieve additional orders in
    /// perpetual futures trading.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub next_page_cursor: String,
}
