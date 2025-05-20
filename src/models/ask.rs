use crate::prelude::*;

/// Represents an ask (sell) order in the order book.
///
/// Each ask contains a price and quantity at which sellers are willing to sell. Bots use this to assess selling pressure and optimize order placement.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Ask {
    /// The ask price.
    ///
    /// The price at which the seller is offering to sell. Bots use this to determine resistance levels and calculate slippage for buy orders in perpetual futures.
    #[serde(with = "string_to_float")]
    pub price: f64,

    /// The ask quantity.
    ///
    /// The quantity available at the ask price. Bots use this to assess liquidity and estimate the impact of large buy orders on the market.
    #[serde(with = "string_to_float")]
    pub qty: f64,
}

impl Ask {
    /// Constructs a new Ask with specified price and quantity.
    ///
    /// Useful for testing or simulating order book data. Bots typically receive this from the API rather than constructing it manually.
    pub fn new(price: f64, qty: f64) -> Ask {
        Ask { price, qty }
    }
}
