use crate::prelude::*;

/// Lot size constraints for an instrument.
///
/// Defines the allowable order quantity ranges and increments. Bots must use these to place valid orders in perpetual futures.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LotSizeFilter {
    /// The base asset precision (optional).
    ///
    /// The precision for the base asset quantity (e.g., `"0.001"` for BTC). Bots should use this to format order quantities correctly.
    #[serde(skip_serializing_if = "is_empty_or_none")]
    pub base_precision: Option<String>,

    /// The quote asset precision (optional).
    ///
    /// The precision for the quote asset amount. Useful for calculating order values in USDT for linear perpetuals.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_precision: Option<String>,

    /// The maximum market order quantity (optional).
    ///
    /// The largest quantity allowed for market orders. Bots should check this to avoid oversized market orders, which could be rejected or cause slippage.
    #[serde(
        with = "string_to_float_optional",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_mkt_order_qty: Option<f64>,
    /// The minimum order quantity.
    ///
    /// The smallest quantity allowed for orders. Bots must ensure order sizes are at least this value to avoid rejections.
    #[serde(with = "string_to_float")]
    pub min_order_qty: f64,

    /// The maximum order quantity.
    ///
    /// The largest quantity allowed for orders. Bots should cap order sizes to this value to comply with Bybit’s rules.
    #[serde(with = "string_to_float")]
    pub max_order_qty: f64,

    /// The minimum order amount (optional).
    ///
    /// The smallest order value in the quote asset (e.g., USDT). Bots should verify order values meet this threshold.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_order_amt: Option<String>,

    /// The maximum order amount (optional).
    ///
    /// The largest order value in the quote asset. Bots should ensure order values are below this to avoid rejections.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_order_amt: Option<String>,

    /// The quantity step size (optional).
    ///
    /// The increment for order quantities. Bots must round order sizes to this step to comply with Bybit’s rules.
    #[serde(
        with = "string_to_float_optional",
        skip_serializing_if = "Option::is_none"
    )]
    pub qty_step: Option<f64>,
    /// The maximum post-only order quantity (optional).
    ///
    /// The largest quantity for post-only orders. Bots using post-only orders (maker-only) should respect this limit.
    #[serde(
        with = "string_to_float_optional",
        skip_serializing_if = "Option::is_none"
    )]
    pub post_only_max_order_qty: Option<f64>,
    /// The minimum notional value (optional).
    ///
    /// The smallest notional value (price × quantity) for orders. Bots should verify orders meet this to avoid rejections.
    #[serde(
        with = "string_to_float_optional",
        skip_serializing_if = "Option::is_none"
    )]
    pub min_notional_value: Option<f64>,
}
