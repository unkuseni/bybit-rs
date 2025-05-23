use crate::prelude::*;

/// Represents a single trade record.
///
/// Each trade record details an executed trade, including price, quantity, side, and timestamp. For perpetual futures, this data is critical for understanding short-term market dynamics and confirming trading signals.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RecentTrade {
    /// The timestamp of the trade execution (Unix timestamp in milliseconds).
    ///
    /// Indicates when the trade occurred. Bots use this to align trade data with other time-series data (e.g., Klines) and to assess the recency of market activity for real-time strategies.
    #[serde(with = "string_to_u64")]
    pub time: u64,

    /// The trading pair symbol (e.g., "BTCUSDT").
    ///
    /// Identifies the perpetual futures contract for the trade. Bots should verify this matches the requested symbol to ensure data accuracy.
    pub symbol: String,

    /// The trade execution price.
    ///
    /// The price at which the trade was executed. Bots use this to track price movements and calculate metrics like average trade price or slippage in perpetual futures. Stored as a string to preserve precision, so bots must parse it to `f64` for calculations.
    #[serde(with = "string_to_float")]
    pub price: f64,

    /// The trade quantity (in base asset).
    ///
    /// The amount of the base asset traded (e.g., BTC in `BTCUSDT`). Bots use this to assess trade size and market liquidity, as large trades may indicate institutional activity or strong market sentiment.
    #[serde(with = "string_to_float")]
    pub size: f64,

    /// The side of the trade ("Buy" or "Sell").
    ///
    /// Indicates whether the trade was a buy (taker buying from maker) or sell (taker selling to maker). Bots use this to analyze buying vs. selling pressure, which can inform momentum-based strategies in perpetual futures.
    pub side: Side,

    /// The unique trade ID.
    ///
    /// A unique identifier for the trade on Bybit’s exchange. Bots can use this to track specific trades, avoid duplicates in data processing, or correlate trades with other exchange data.
    pub exec_id: String,

    /// Indicates if the trade is a block trade.
    ///
    /// A boolean (`true` or `false`) indicating whether the trade was a large block trade, typically executed off the public order book. Bots can use this to identify significant market moves driven by institutional or large traders in perpetual futures.
    pub is_block_trade: bool,

    #[serde(rename = "isRPITrade")]
    pub is_rpi_trade: bool,
}
