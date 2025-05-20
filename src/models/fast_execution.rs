use crate::prelude::*;

/// Represents a fast execution event in Bybit's perpetual futures market.
///
/// This struct is a lightweight version of the execution event, providing essential trade details for high-frequency updates.
///
/// # Bybit API Reference
/// Part of the execution WebSocket stream (https://bybit-exchange.github.io/docs/v5/websocket/private/execution).
///
/// # Perpetual Futures Context
/// Fast executions are designed for low-latency trading bots that need minimal data to process trades quickly.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FastExecution {
    /// The WebSocket topic (e.g., "execution").
    ///
    /// Identifies the fast execution data stream. Bots use this to filter relevant messages.
    pub topic: String,
    /// The timestamp when the event was created (in milliseconds).
    ///
    /// Indicates when the execution occurred. Bots use this for time-based analysis.
    pub creation_time: u64,
    /// The fast execution data for the event.
    ///
    /// Contains essential trade details. Bots process this for rapid trade updates.
    pub data: Vec<FastExecData>,
}
