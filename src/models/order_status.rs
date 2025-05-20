use crate::prelude::*;

/// Represents the status of an order, including its identifiers.
///
/// This struct provides the `order_id` and `order_link_id` for an order, used in
/// responses for order placement, amendment, or cancellation. In perpetual futures,
/// bots use this to track orders and correlate API responses with internal state.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderStatus {
    /// The unique identifier of the order provided by Bybit.
    ///
    /// Used to reference the order in API requests and responses. Bots must store
    /// this to manage orders effectively in perpetual futures trading.
    pub order_id: String,

    /// The user-defined identifier of the order.
    ///
    /// Allows bots to track orders using custom IDs. Essential for complex strategies
    /// in perpetual futures where multiple orders are active.
    pub order_link_id: String,
}
