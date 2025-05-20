use crate::prelude::*;

/// Represents a position event in Bybit's perpetual futures market.
///
/// This struct is used in WebSocket streams to provide updates about a user's open positions, including size, leverage, and profit/loss.
///
/// # Bybit API Reference
/// The Bybit API (https://bybit-exchange.github.io/docs/v5/websocket/private/position) provides position data via WebSocket.
///
/// # Perpetual Futures Context
/// Positions in perpetual futures represent a trader's exposure to a market. Bots monitor position updates to manage risk, adjust leverage, or trigger stop-loss/take-profit orders.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PositionEvent {
    /// The unique identifier for the position event.
    ///
    /// Used to track specific position updates. Bots can use this to correlate events with internal state.
    pub id: String,

    /// The WebSocket topic (e.g., "position").
    ///
    /// Identifies the position data stream. Bots use this to filter relevant messages.
    pub topic: String,

    /// The timestamp when the event was created (in milliseconds).
    ///
    /// Indicates when the position update occurred. Bots use this for time-based analysis.
    pub creation_time: u64,

    /// The position data for the event.
    ///
    /// Contains details about the user's positions. Bots process this to manage open trades.
    pub data: Vec<PositionData>,
}
