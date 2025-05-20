use crate::prelude::*;

/// Represents a request to cancel an order on Bybit.
///
/// This struct defines parameters to cancel an existing order, identified by `order_id`
/// or `order_link_id`. In perpetual futures, canceling orders is common to adjust
/// strategies or avoid unwanted executions. Bots must ensure accurate identification
/// and handle API responses to confirm cancellation, as untracked orders can lead to
/// unintended positions.
#[derive(Clone, Serialize)]
pub struct CancelOrderRequest<'a> {
    /// The category of the trading product (e.g., Spot, Linear).
    ///
    /// Specifies the market of the order to cancel. Must match the original order’s
    /// category to avoid errors. In perpetual futures, `Linear` is typical.
    pub category: Category,
    /// The trading pair symbol, e.g., "BTCUSDT".
    ///
    /// Identifies the asset pair of the order. Must match the original order’s symbol.
    /// Bots should validate symbol consistency to ensure correct cancellation.
    pub symbol: Cow<'a, str>,
    /// The unique identifier of the order to cancel.
    ///
    /// Provided by Bybit when the order was created. Either `order_id` or
    /// `order_link_id` must be provided to identify the order for cancellation.
    pub order_id: Option<Cow<'a, str>>,
    /// The user-defined identifier of the order to cancel.
    ///
    /// Allows bots to reference orders using their custom ID. Useful for tracking
    /// cancellations in complex perpetual futures strategies.
    pub order_link_id: Option<Cow<'a, str>>,
    /// A filter to specify the order type (e.g., "tpslOrder").
    ///
    /// Used to target specific order types for cancellation, such as TP/SL orders.
    /// Bots should set this correctly to cancel the intended orders in perpetual futures.
    pub order_filter: Option<Cow<'a, str>>,
}
