use crate::prelude::*;

/// Represents a single Kline (candlestick) record.
///
/// Each Kline record corresponds to a time interval and includes price and volume data. For perpetual futures, this data is used to analyze price trends, volatility, and trading volume to inform trading decisions.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Kline {
    /// The start time of the candlestick (Unix timestamp in milliseconds).
    ///
    /// Marks the beginning of the time interval for this candlestick. Bots use this to align data with their internal timelines and ensure chronological order when processing multiple Klines.
    #[serde(with = "string_to_u64")]
    pub start_time: u64,
    /// The opening price of the candlestick.
    ///
    /// The price at the start of the interval. For perpetual futures, this is critical for calculating price changes and indicators like moving averages. Stored as a string to preserve precision, so bots must parse it to `f64` for calculations.
    pub open_price: String,
    /// The highest price during the candlestick interval.
    ///
    /// Indicates the peak price within the interval. Used in volatility analysis and for indicators like Bollinger Bands. Bots should parse this string to `f64` for numerical computations.
    pub high_price: String,
    /// The lowest price during the candlestick interval.
    ///
    /// Indicates the trough price within the interval. Essential for identifying support levels and calculating indicators like the Average True Range (ATR). Requires parsing to `f64`.
    pub low_price: String,
    /// The closing price of the candlestick.
    ///
    /// The price at the end of the interval. Critical for trend analysis and indicators like MACD or candlestick pattern recognition. Bots must parse this string to `f64`.
    pub close_price: String,
    /// The trading volume during the candlestick interval.
    ///
    /// Represents the total quantity of the base asset traded (e.g., BTC in `BTCUSDT`). For perpetual futures, high volume often indicates strong market interest or liquidity, which bots can use to confirm trade signals. Parse to `f64` for analysis.
    pub volume: String,
    /// The quote asset volume during the candlestick interval.
    ///
    /// Represents the total value traded in the quote asset (e.g., USDT in `BTCUSDT`). Useful for assessing market activity and liquidity in perpetual futures. Bots should parse this to `f64` for financial calculations.
    pub quote_asset_volume: String,
}
