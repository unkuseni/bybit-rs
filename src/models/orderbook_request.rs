use crate::prelude::*;

/// Parameters for requesting order book data.
///
/// This struct defines the parameters for querying the order book via the `/v5/market/orderbook` endpoint. The order book shows current bid and ask prices and quantities, critical for liquidity analysis and order placement in perpetual futures.
#[derive(Clone, Default)]
pub struct OrderbookRequest<'a> {
    /// The trading pair symbol (e.g., "BTCUSDT").
    ///
    /// Identifies the perpetual futures contract. Bots must specify a valid symbol to fetch the correct order book.
    pub symbol: Cow<'a, str>,

    /// The product category (e.g., Linear, Inverse).
    ///
    /// Specifies the instrument type. For perpetual futures, use `Linear` or `Inverse`. Bots must set this correctly to avoid errors.
    pub category: Category,

    /// The maximum number of order book levels to return (1-500, default: 50).
    ///
    /// Controls the depth of the order book (number of bid/ask levels). Bots should balance depth with performance: deeper books provide more liquidity data but increase latency and memory usage.
    pub limit: Option<u64>,
}

impl<'a> OrderbookRequest<'a> {
    /// Creates a default Orderbook request.
    ///
    /// Returns a request with `symbol` set to `"BTCUSDT"` and `category` set to `Linear`. Suitable for testing but should be customized for production.
    pub fn default() -> OrderbookRequest<'a> {
        OrderbookRequest::new("BTCUSDT", Category::Linear, None)
    }
    /// Constructs a new Orderbook request with specified parameters.
    ///
    /// Allows customization. Bots should use this to specify the exact symbol and category for their perpetual futures strategy.
    pub fn new(symbol: &'a str, category: Category, limit: Option<u64>) -> OrderbookRequest<'a> {
        OrderbookRequest {
            symbol: Cow::Borrowed(symbol),
            category,
            limit,
        }
    }
}
