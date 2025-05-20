use crate::prelude::*;

/// Represents a bid (buy) order in the order book.
///
/// Each bid contains a price and quantity at which buyers are willing to buy. Bots use this to assess buying support and optimize order placement.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Bid {
    /// The bid price.
    ///
    /// The price at which the buyer is offering to buy. Bots use this to determine support levels and calculate slippage for sell orders in perpetual futures.
    #[serde(with = "string_to_float")]
    pub price: f64,

    /// The bid quantity.
    ///
    /// The quantity available at the bid price. Bots use this to assess liquidity and estimate the impact of large sell orders on the market.
    #[serde(with = "string_to_float")]
    pub qty: f64,
}

impl Bid {
    /// Constructs a new Bid with specified price and quantity.
    ///
    /// Useful for testing or simulating order book data. Bots typically receive this from the API rather than constructing it manually.
    pub fn new(price: f64, qty: f64) -> Bid {
        Bid { price, qty }
    }
}
