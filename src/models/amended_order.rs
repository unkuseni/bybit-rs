use crate::prelude::*;

/// Represents a single amended order result.
///
/// Details the outcome of an individual order amendment in a batch request. Bots use this to track specific order modifications and correlate with their trading strategy.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AmendedOrder {
    /// The product category (e.g., "linear").
    ///
    /// Indicates the instrument type of the order. Bots should verify this matches the requested category.
    pub category: Category,

    /// The trading pair symbol (e.g., "BTCUSDT").
    ///
    /// Identifies the perpetual futures contract for the order. Bots should confirm this matches the requested symbol.
    pub symbol: String,

    /// The unique order ID.
    ///
    /// Identifies the amended order on Bybit’s exchange. Bots use this to track the order’s status and executions.
    pub order_id: String,

    /// The user-defined order link ID.
    ///
    /// A custom identifier for the order. Bots can use this to correlate the order with specific strategies or client requests.
    pub order_link_id: String,
}
