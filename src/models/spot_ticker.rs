use crate::prelude::*;

/// Represents ticker data for a spot trading pair.
///
/// Contains market data for spot markets. Not relevant for perpetual futures but included for completeness.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SpotTicker {
    /// The trading pair symbol (e.g., "BTCUSDT").
    ///
    /// Identifies the spot trading pair. Not relevant for perpetuals.
    pub symbol: String,
    /// The best bid price.
    ///
    /// The highest price buyers are willing to pay in the spot market. Not relevant for perpetuals.
    #[serde(rename = "bid1Price", with = "string_to_float")]
    pub bid_price: f64,
    /// The size of the best bid order.
    ///
    /// The quantity at the best bid price. Not relevant for perpetuals.
    #[serde(rename = "bid1Size", with = "string_to_float")]
    pub bid_size: f64,
    /// The best ask price.
    ///
    /// The lowest price sellers are willing to accept in the spot market. Not relevant for perpetuals.
    #[serde(rename = "ask1Price", with = "string_to_float")]
    pub ask_price: f64,
    /// The size of the best ask order.
    ///
    /// The quantity at the best ask price. Not relevant for perpetuals.
    #[serde(rename = "ask1Size", with = "string_to_float")]
    pub ask_size: f64,
    /// The last traded price.
    ///
    /// The most recent price in the spot market. Not relevant for perpetuals.
    #[serde(with = "string_to_float")]
    pub last_price: f64,
    /// The price 24 hours ago.
    ///
    /// The price in the spot market 24 hours prior. Not relevant for perpetuals.
    #[serde(rename = "prevPrice24h", with = "string_to_float")]
    pub prev_price_24h: f64,
    /// The percentage price change over the last 24 hours.
    ///
    /// Calculated for the spot market. Not relevant for perpetuals.
    #[serde(rename = "price24hPcnt", with = "string_to_float")]
    pub daily_change_percentage: f64,
    /// The highest price in the last 24 hours.
    ///
    /// The peak price in the spot market. Not relevant for perpetuals.
    #[serde(rename = "highPrice24h", with = "string_to_float")]
    pub high_24h: f64,
    /// The lowest price in the last 24 hours.
    ///
    /// The trough price in the spot market. Not relevant for perpetuals.
    #[serde(rename = "lowPrice24h", with = "string_to_float")]
    pub low_24h: f64,
    /// The trading turnover in the last 24 hours (in quote asset).
    ///
    /// The total value traded in the spot market. Not relevant for perpetuals.
    #[serde(with = "string_to_float")]
    pub turnover_24h: f64,
    /// The trading volume in the last 24 hours (in base asset).
    ///
    /// The total quantity traded in the spot market. Not relevant for perpetuals.
    #[serde(with = "string_to_float")]
    pub volume_24h: f64,
    /// The USD index price.
    ///
    /// The spot price of the asset in USD. Not relevant for perpetuals.
    #[serde(with = "string_to_float")]
    pub usd_index_price: f64,
}
