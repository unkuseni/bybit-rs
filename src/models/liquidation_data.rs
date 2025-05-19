use crate::prelude::*;

/// Contains the details of a liquidation event.
///
/// This struct provides specific information about a liquidated position, including the time, symbol, side, size, and price.
///
/// # Bybit API Reference
/// Liquidation data is part of the WebSocket liquidation stream (https://bybit-exchange.github.io/docs/v5/websocket/public/liquidation).
///
/// # Perpetual Futures Context
/// Liquidation data helps bots understand the size and direction of forced closures, which can indicate market sentiment or trigger stop-loss cascades.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LiquidationData {
    /// The timestamp when the liquidation was updated (in milliseconds).
    ///
    /// Indicates the exact time of the liquidation event. Bots can use this to align liquidation data with price or volume spikes.
    pub updated_time: u64,
    /// The trading pair symbol (e.g., "BTCUSDT").
    ///
    /// Specifies the market where the liquidation occurred. Bots filter by symbol to focus on relevant markets.
    pub symbol: String,
    /// The side of the liquidated position ("Buy" or "Sell").
    ///
    /// Indicates whether the liquidated position was long (Buy) or short (Sell). A high volume of liquidations on one side can signal a potential price reversal, which bots can exploit.
    pub side: String,
    /// The size of the liquidated position (in contracts or base currency).
    ///
    /// Represents the volume of the position closed. Large liquidations can cause significant price movements, and bots should monitor this to anticipate volatility.
    #[serde(with = "string_to_float")]
    pub size: f64,
    /// The price at which the position was liquidated.
    ///
    /// This is the market price at which the position was forcibly closed. Bots can use this to identify liquidation price levels, which often act as support or resistance zones.
    #[serde(with = "string_to_float")]
    pub price: f64,
}
