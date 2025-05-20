use crate::prelude::*;

/// Parameters for requesting instrument information.
///
/// This struct defines the parameters for querying instrument details via the `/v5/market/instruments-info` endpoint. For perpetual futures, instrument info includes leverage, price filters, and lot size filters, which are critical for configuring trading bot parameters.
#[derive(Clone, Default)]
pub struct InstrumentRequest<'a> {
    /// The product category (e.g., Linear, Inverse, Spot, Option).
    ///
    /// Specifies the instrument type. For perpetual futures, use `Linear` or `Inverse`. Bots must set this to filter relevant instruments.
    pub category: Category,

    /// The trading pair symbol (e.g., "BTCUSDT").
    ///
    /// Optionally specifies a single trading pair. If unset, the API returns data for all instruments in the category, which may be voluminous. Bots should set this for specific pairs to reduce response size and latency.
    pub symbol: Option<Cow<'a, str>>,

    /// The trading status of the instrument (true for trading, false for not trading).
    ///
    /// Filters instruments by their trading status. Useful for bots to exclude delisted or inactive perpetual futures contracts. If unset, all statuses are returned.
    pub status: Option<bool>,

    /// The base coin of the instrument (e.g., "BTC").
    ///
    /// Filters instruments by their base asset. For example, setting `base_coin` to `"BTC"` returns all BTC-based perpetuals (e.g., `BTCUSDT`, `BTCUSD`). Useful for bots targeting specific assets.
    pub base_coin: Option<Cow<'a, str>>,

    /// The maximum number of instruments to return (1-1000, default: 500).
    ///
    /// Controls the response size. Bots should set a reasonable limit to balance data completeness and performance, especially when querying all instruments in a category.
    pub limit: Option<u64>,
}

impl<'a> InstrumentRequest<'a> {
    /// Creates a default Instrument request.
    ///
    /// Returns a request with `category` set to `Linear` and `symbol` set to `"BTCUSDT"`. Suitable for testing but should be customized for production to match specific trading needs.
    pub fn default() -> InstrumentRequest<'a> {
        InstrumentRequest::new(Category::Linear, Some("BTCUSDT"), None, None, None)
    }
    /// Constructs a new Instrument request with specified parameters.
    ///
    /// Allows full customization. Bots should use this to tailor requests to their strategy, ensuring `category` and `symbol` align with the perpetual futures being traded.
    pub fn new(
        category: Category,
        symbol: Option<&'a str>,
        status: Option<bool>,
        base_coin: Option<&'a str>,
        limit: Option<u64>,
    ) -> InstrumentRequest<'a> {
        InstrumentRequest {
            category,
            symbol: symbol.map(Cow::Borrowed),
            status,
            base_coin: base_coin.map(Cow::Borrowed),
            limit,
        }
    }
}
