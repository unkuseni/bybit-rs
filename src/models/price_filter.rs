use crate::prelude::*;

/// Price constraints for an instrument.
///
/// Defines the allowable price ranges and increments for orders. Bots must use these to place valid orders in perpetual futures.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase", default)]
pub struct PriceFilter {
    /// The minimum price allowed (optional).
    ///
    /// The lowest price at which orders can be placed. If `None`, there’s no lower bound. Bots should check this to avoid rejected orders due to prices being too low.
    #[serde(
        with = "string_to_float_optional",
        skip_serializing_if = "Option::is_none"
    )]
    pub min_price: Option<f64>,
    /// The maximum price allowed (optional).
    ///
    /// The highest price at which orders can be placed. If `None`, there’s no upper bound. Bots should ensure order prices are below this to avoid rejections.
    #[serde(
        with = "string_to_float_optional",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_price: Option<f64>,
    /// The price tick size.
    ///
    /// The increment for price changes (e.g., `0.01` for two decimal places). Bots must round order prices to this increment to comply with Bybit’s precision rules, avoiding rejection.
    #[serde(with = "string_to_float")]
    pub tick_size: f64,
}
