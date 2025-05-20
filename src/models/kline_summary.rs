use crate::prelude::*;

/// Summarizes Kline data for a trading pair.
///
/// Part of the `KlineResponse`, this struct organizes the Kline data by symbol, category, and a list of candlesticks. It’s the primary container for the data used in technical analysis for perpetual futures.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct KlineSummary {
    /// The trading pair symbol (e.g., "BTCUSDT").
    ///
    /// Confirms the trading pair for which the Kline data was requested. Bots should verify this matches the requested `symbol` to ensure data integrity.
    pub symbol: String,
    /// The product category (e.g., "linear", "spot").
    ///
    /// Indicates the type of instrument (e.g., `linear` for USDT-margined perpetuals). Bots should check this to ensure the data aligns with the intended trading strategy.
    pub category: String,
    /// A list of Kline (candlestick) records.
    ///
    /// Contains the actual candlestick data, each representing a time interval with open, high, low, close prices, and volume. This is the core data used by trading bots for technical analysis and strategy execution.
    pub list: Vec<Kline>,
}
