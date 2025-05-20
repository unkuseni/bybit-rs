use crate::prelude::*;

/// Represents spot market ticker data for a trading pair on Bybit.
///
/// This struct provides real-time market data for spot trading pairs, including price, volume, and 24-hour statistics. While the file focuses on perpetual futures, this struct is included for spot market data, which can be relevant for bots hedging or arbitraging between spot and futures markets.
///
/// # Bybit API Reference
/// The Bybit API (https://bybit-exchange.github.io/docs/v5/market/ticker) provides ticker data for spot markets, including last price, 24-hour high/low, and volume. Fields are serialized as strings but deserialized to `f64` for numerical computations.
///
/// # Perpetual Futures Context
/// Spot ticker data can be used in conjunction with perpetual futures to identify arbitrage opportunities (e.g., cash-and-carry trades) or to monitor funding rates, as spot prices often serve as a reference for futures pricing.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SpotTickerData {
    /// The trading pair symbol (e.g., "BTCUSDT").
    ///
    /// Identifies the market for which the ticker data applies. Bots use this to filter and process data for specific trading pairs.
    pub symbol: String,
    /// The most recent trade price.
    ///
    /// This is the price of the last executed trade in the spot market. For bots, this is critical for tracking market trends and triggering trades based on price thresholds.
    #[serde(with = "string_to_float")]
    pub last_price: f64,
    /// The highest price in the last 24 hours.
    ///
    /// Useful for identifying price volatility and setting upper bounds for trading strategies. Bots can use this to detect breakout patterns or avoid trading during extreme volatility.
    #[serde(with = "string_to_float")]
    pub high_price_24h: f64,
    /// The lowest price in the last 24 hours.
    ///
    /// Helps bots assess support levels and market ranges. Combined with `high_price_24h`, it provides a 24-hour price range for volatility-based strategies.
    #[serde(with = "string_to_float")]
    pub low_price_24h: f64,
    /// The price 24 hours ago.
    ///
    /// Used to calculate price changes over the last 24 hours. Bots can use this to compute momentum or mean-reversion signals.
    #[serde(with = "string_to_float")]
    pub prev_price_24h: f64,
    /// The trading volume in the last 24 hours (in base currency).
    ///
    /// Indicates market activity and liquidity. High volume suggests strong participation, which bots can use to prioritize liquid markets for lower slippage.
    #[serde(with = "string_to_float")]
    pub volume_24h: f64,
    /// The turnover (value of trades) in the last 24 hours (in quote currency).
    ///
    /// Represents the total monetary value of trades. Bots can use this to gauge market interest and correlate with volume for liquidity analysis.
    #[serde(with = "string_to_float")]
    pub turnover_24h: f64,
    /// The percentage price change over the last 24 hours.
    ///
    /// A key metric for momentum trading strategies. Bots can use this to identify trending markets or trigger mean-reversion trades based on overbought/oversold conditions.
    #[serde(with = "string_to_float")]
    pub price_24h_pcnt: f64,
    /// The USD index price for the asset.
    ///
    /// Represents the fair value of the asset based on Bybit's index calculation. For bots, this is critical for pricing perpetual futures, as it influences funding rates and mark prices.
    #[serde(with = "string_to_float")]
    pub usd_index_price: f64,
}
