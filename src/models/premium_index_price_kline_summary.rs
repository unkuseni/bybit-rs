use crate::prelude::*;

/// Summarizes Premium Index Price Kline data for a trading pair.
///
/// Part of the `PremiumIndexPriceKlineResponse`, this struct organizes the Premium Index Price Kline data. It’s used to analyze the premium/discount in perpetual futures pricing.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PremiumIndexPriceKlineSummary {
    /// The trading pair symbol (e.g., "BTCUSDT").
    ///
    /// Confirms the trading pair. Bots should verify this matches the requested symbol.
    pub symbol: String,

    /// The product category (e.g., "linear").
    ///
    /// Indicates the instrument type. Bots should ensure alignment with their strategy.
    pub category: String,

    /// A list of Premium Index Price Kline records.
    ///
    /// Contains the premium index price candlestick data. Bots use this to predict funding rate changes and optimize trading strategies.
    pub list: Vec<PremiumIndexPriceKline>,
}
