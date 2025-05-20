use crate::prelude::*;

/// Summarizes Mark Price Kline data for a trading pair.
///
/// Part of the `MarkPriceKlineResponse`, this struct organizes the Mark Price Kline data by symbol, category, and a list of candlesticks. It’s used to analyze the mark price, which drives funding rates and liquidations in perpetual futures.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MarkPriceKlineSummary {
    /// The trading pair symbol (e.g., "BTCUSDT").
    ///
    /// Confirms the trading pair for the Mark Price Kline data. Bots should verify this matches the requested symbol.
    pub symbol: String,

    /// The product category (e.g., "linear").
    ///
    /// Indicates the instrument type (e.g., `linear` for USDT-margined perpetuals). Bots should ensure this aligns with their trading strategy.
    pub category: String,

    /// A list of Mark Price Kline records.
    ///
    /// Contains the mark price candlestick data, each representing a time interval with open, high, low, and close mark prices. Bots use this for funding rate analysis and liquidation risk assessment.
    pub list: Vec<MarkPriceKline>,
}
