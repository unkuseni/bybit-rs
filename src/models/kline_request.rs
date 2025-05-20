use crate::prelude::*;

/// Parameters for requesting Kline (candlestick) data.
///
/// Kline data represents price movements over fixed time intervals (e.g., 1-minute, 1-hour) and is used for technical analysis in trading. This struct defines the parameters for querying Kline data via Bybit's `/v5/market/kline` endpoint. Perpetual futures on Bybit, unlike traditional futures, have no expiry date and are funded via a funding rate mechanism, making Kline data critical for analyzing price trends in these instruments.
#[derive(Clone, Default)]
pub struct KlineRequest<'a> {
    /// The product category (e.g., Spot, Linear, Inverse, Option).
    ///
    /// Specifies the type of instrument for the Kline data. For perpetual futures, `Linear` (USDT-margined) or `Inverse` (coin-margined) are most relevant. Bots should set this to match the trading pair (e.g., `Linear` for `BTCUSDT`). If unset, the API may return an error or default to an unexpected category.
    pub category: Option<Category>,

    /// The trading pair symbol (e.g., "BTCUSDT").
    ///
    /// Identifies the specific perpetual futures contract or other instrument. For perpetuals, this is typically a USDT pair (e.g., `BTCUSDT`) for linear contracts or a coin pair (e.g., `BTCUSD`) for inverse contracts. Bots must ensure the symbol is valid for the chosen `category` to avoid errors.
    pub symbol: Cow<'a, str>,

    /// The time interval for each candlestick (e.g., "1m", "1h", "1d").
    ///
    /// Specifies the granularity of the Kline data, such as `1m` (1 minute), `5m` (5 minutes), `1h` (1 hour), or `1d` (1 day). The choice depends on the trading strategy: short-term bots may use smaller intervals (e.g., `1m`), while long-term strategies may use `1d`. Invalid intervals will result in an API error.
    pub interval: Cow<'a, str>,

    /// The start time for the Kline data (Unix timestamp in milliseconds).
    ///
    /// Defines the beginning of the time range for the Kline data. For perpetual futures, this is useful for fetching historical data to backtest trading strategies. If unset, the API may return data from the most recent period, which may not suit historical analysis needs.
    pub start: Option<Cow<'a, str>>,

    /// The end time for the Kline data (Unix timestamp in milliseconds).
    ///
    /// Defines the end of the time range for the Kline data. Bots should set this to limit the data to a specific period, especially for performance optimization when processing large datasets. If unset, the API typically returns data up to the current time.
    pub end: Option<Cow<'a, str>>,

    /// The maximum number of Kline records to return (1-200, default: 200).
    ///
    /// Controls the number of candlesticks returned in the response. For trading bots, setting a lower limit can reduce latency and memory usage, especially for high-frequency strategies. However, for comprehensive analysis, bots may need to paginate through multiple requests to fetch all desired data.
    pub limit: Option<u64>,
}

impl<'a> KlineRequest<'a> {
    /// Creates a default Kline request with preset values.
    ///
    /// Returns a `KlineRequest` with `symbol` set to `"BTCUSDT"` and other fields as defaults. Useful for quick testing or prototyping trading bots but should be customized for production to match specific trading pairs and intervals.
    pub fn default() -> KlineRequest<'a> {
        KlineRequest::new(None, "BTCUSDT", "", None, None, None)
    }

    /// Constructs a new Kline request with specified parameters.
    ///
    /// Allows full customization of the Kline request. Trading bots should use this to specify exact parameters for their strategy, ensuring the `symbol`, `interval`, and `category` align with the perpetual futures contract being traded.
    pub fn new(
        category: Option<Category>,
        symbol: &'a str,
        interval: &'a str,
        start: Option<&'a str>,
        end: Option<&'a str>,
        limit: Option<u64>,
    ) -> KlineRequest<'a> {
        KlineRequest {
            category,
            symbol: Cow::Borrowed(symbol),
            interval: Cow::Borrowed(interval),
            start: start.map(Cow::Borrowed),
            end: end.map(Cow::Borrowed),
            limit,
        }
    }
}
