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
    #[serde(default, with = "string_to_float_optional")]
    pub base_precision: Option<f64>,

    /// The quote asset precision (optional).
    ///
    /// The precision for the quote asset amount. Useful for calculating order values in USDT for linear perpetuals.
    #[serde(default, with = "string_to_float_optional")]
    pub quote_precision: Option<f64>,

    /// The maximum market order quantity (optional).
    ///
    /// The largest quantity allowed for market orders. Bots should check this to avoid oversized market orders, which could be rejected or cause slippage.
    #[serde(default, with = "string_to_float_optional")]
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
    #[serde(default, with = "string_to_float_optional")]
    pub min_order_amt: Option<f64>,

    /// The maximum order amount (optional).
    ///
    /// The largest order value in the quote asset. Bots should ensure order values are below this to avoid rejections.
    #[serde(default, with = "string_to_float_optional")]
    pub max_order_amt: Option<f64>,

    /// The quantity step size (optional).
    ///
    /// The increment for order quantities. Bots must round order sizes to this step to comply with Bybit’s rules.
    #[serde(default, with = "string_to_float_optional")]
    pub qty_step: Option<f64>,
    /// The maximum post-only order quantity (optional).
    ///
    /// The largest quantity for post-only orders. Bots using post-only orders (maker-only) should respect this limit.
    #[serde(default, with = "string_to_float_optional")]
    pub post_only_max_order_qty: Option<f64>,
    /// The minimum notional value (optional).
    ///
    /// The smallest notional value (price × quantity) for orders. Bots should verify orders meet this to avoid rejections.
    #[serde(default, with = "string_to_float_optional")]
    pub min_notional_value: Option<f64>,
}

// Spot Lot Size Filter Example
// "lotSizeFilter": {
//     "basePrecision": "0.000001",
//     "quotePrecision": "0.00000001",
//     "minOrderQty": "0.000048",
//     "maxOrderQty": "71.73956243",
//     "minOrderAmt": "1",
//     "maxOrderAmt": "2000000"
// },

// Linear Lot Size Filter Example
// "lotSizeFilter": {
//     "maxOrderQty": "1190.000",
//     "minOrderQty": "0.001",
//     "qtyStep": "0.001",
//     "postOnlyMaxOrderQty": "1190.000",
//     "maxMktOrderQty": "500.000",
//     "minNotionalValue": "5"
// },
