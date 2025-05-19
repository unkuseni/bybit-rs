use crate::prelude::*;

/// Represents a liquidation event in Bybit's perpetual futures market.
///
/// Liquidation occurs when a trader's position cannot meet margin requirements, leading to forced closure. This struct is used in WebSocket streams to notify bots of liquidation events, which can signal market stress or volatility.
///
/// # Bybit API Reference
/// The Bybit API (https://bybit-exchange.github.io/docs/v5/websocket/public/liquidation) provides liquidation data via WebSocket, including symbol, side, size, and price.
///
/// # Perpetual Futures Context
/// In perpetual futures, liquidations happen when a trader's margin balance falls below the maintenance margin due to adverse price movements. Liquidation events can cause price cascades, especially in highly leveraged markets, making them critical for bots to monitor.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Liquidation {
    /// The WebSocket topic (e.g., "liquidation.BTCUSDT").
    ///
    /// Identifies the data stream and symbol for the liquidation event. Bots use this to filter relevant liquidation messages.
    #[serde(rename = "topic")]
    pub topic: String,
    /// The type of WebSocket event (e.g., "snapshot" or "delta").
    ///
    /// Indicates whether the data is a full snapshot or incremental update. Bots must handle both types to maintain an accurate state.
    #[serde(rename = "type")]
    pub event_type: String,
    /// The timestamp of the event (in milliseconds).
    ///
    /// Represents when the liquidation occurred. Bots use this for time-based analysis, such as correlating liquidations with price movements.
    #[serde(rename = "ts")]
    pub ts: u64,
    /// The liquidation details.
    ///
    /// Contains the core data about the liquidation, such as symbol, side, size, and price. Bots process this to assess market impact.
    #[serde(rename = "data")]
    pub data: LiquidationData,
}
