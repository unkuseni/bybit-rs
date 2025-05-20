use crate::prelude::*;

/// Summarizes Index Price Kline data for a trading pair.
///
/// Part of the `IndexPriceKlineResponse`, this struct organizes the Index Price Kline data by symbol, category, and a list of candlesticks. It’s used to analyze the spot market trends affecting perpetual futures.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IndexPriceKlineSummary {
    /// The trading pair symbol (e.g., "BTCUSDT").
    ///
    /// Confirms the trading pair for the Index Price Kline data. Bots should verify this matches the requested symbol.
    pub symbol: String,

    /// The product category (e.g., "linear").
    ///
    /// Indicates the instrument type. Bots should ensure this aligns with their strategy.
    pub category: Category,

    /// A list of Index Price Kline records.
    ///
    /// Contains the index price candlestick data. Bots use this to monitor spot market trends that influence perpetual futures pricing and funding rates.
    pub list: Vec<IndexPriceKline>,
}
