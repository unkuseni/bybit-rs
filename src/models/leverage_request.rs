use crate::prelude::*;

/// Parameters for setting leverage for a trading pair.
///
/// Used to construct a request to the `/v5/position/set-leverage` endpoint to adjust leverage for a specific symbol. Bots use this to manage risk exposure in perpetual futures, as higher leverage increases both potential returns and liquidation risk.
#[derive(Clone, Default)]
pub struct LeverageRequest<'a> {
    /// The product category (e.g., Linear, Inverse).
    ///
    /// Specifies the instrument type. Bots must set this to target the correct contract type.
    pub category: Category,

    /// The trading pair symbol (e.g., "BTCUSDT").
    ///
    /// Identifies the perpetual futures contract for which leverage is being set. Bots must specify a valid symbol.
    pub symbol: Cow<'a, str>,

    /// The leverage value (e.g., 10 for 10x).
    ///
    /// The desired leverage multiplier. Bots should ensure this complies with Bybit’s maximum leverage limits for the symbol to avoid request failures.
    pub leverage: i8,
}

impl<'a> LeverageRequest<'a> {
    /// Constructs a new Leverage request with specified parameters.
    ///
    /// Allows customization of the leverage request. Bots should use this to specify the exact symbol, category, and leverage value.
    pub fn new(category: Category, symbol: &'a str, leverage: i8) -> Self {
        Self {
            category,
            symbol: Cow::Borrowed(symbol),
            leverage,
        }
    }

    /// Creates a default Leverage request.
    ///
    /// Returns a request with `category` set to `Linear`, `symbol` set to `"BTCUSDT"`, and `leverage` set to `10`. Suitable for testing but should be customized for production.
    pub fn default() -> LeverageRequest<'a> {
        LeverageRequest::new(Category::Linear, "BTCUSDT", 10)
    }
}
