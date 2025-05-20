use crate::prelude::*;

/// Represents an execution event in Bybit's perpetual futures market.
///
/// This struct is used in WebSocket streams to provide details about trade executions, such as price, quantity, and fees.
///
/// # Bybit API Reference
/// The Bybit API (https://bybit-exchange.github.io/docs/v5/websocket/private/execution) provides execution data via WebSocket.
///
/// # Perpetual Futures Context
/// Executions represent filled orders or partial fills. Bots use this data to track trade performance, calculate costs, and update position states.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Execution {
    /// The unique identifier for the execution event.
    ///
    /// Used to track specific executions. Bots can use this to correlate events with internal trade records.
    pub id: String,
    /// The WebSocket topic (e.g., "execution").
    ///
    /// Identifies the execution data stream. Bots use this to filter relevant messages.
    pub topic: String,
    /// The timestamp when the event was created (in milliseconds).
    ///
    /// Indicates when the execution occurred. Bots use this for time-based analysis.
    pub creation_time: u64,
    /// The execution data for the event.
    ///
    /// Contains details about the executed trades. Bots process this to update trade logs and positions.
    pub data: Vec<ExecutionData>,
}
