use crate::prelude::*;

/// Represents essential data about a single trade execution for high-frequency updates.
///
/// This struct provides a minimal set of fields for low-latency trade processing.
///
/// # Bybit API Reference
/// Part of the execution WebSocket stream (https://bybit-exchange.github.io/docs/v5/websocket/private/execution).
///
/// # Perpetual Futures Context
/// Fast execution data is optimized for bots requiring minimal latency, such as high-frequency trading strategies.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FastExecData {
    /// The category of the execution (e.g., "linear").
    ///
    /// Specifies the contract type. Bots must handle different categories due to varying margin rules.
    pub category: String,

    /// The trading pair symbol (e.g., "BTCUSDT").
    ///
    /// Identifies the market for the execution. Bots filter by symbol to process relevant trades.
    pub symbol: String,

    /// The unique identifier for the execution.
    ///
    /// Used to track individual trades. Bots use this to match executions with orders.
    pub exec_id: String,

    /// The price at which the trade was executed.
    ///
    /// The actual price of the filled order. Bots use this to update position entry prices.
    #[serde(with = "string_to_float")]
    pub exec_price: f64,

    /// The quantity executed in the trade.
    ///
    /// The volume of the trade. Bots use this to update position sizes and track fills.
    #[serde(with = "string_to_float")]
    pub exec_qty: f64,

    /// The ID of the order associated with the execution.
    ///
    /// Links the execution to the original order. Bots use this to track order status.
    pub order_id: String,

    /// The user-defined ID for the order.
    ///
    /// Allows bots to assign custom identifiers for internal tracking.
    pub order_link_id: String,

    /// The side of the order ("Buy" or "Sell").
    ///
    /// Indicates the direction of the trade. Bots use this to update position direction.
    pub side: String,

    /// The timestamp when the execution occurred (in milliseconds).
    ///
    /// Indicates the exact time of the trade. Bots use this for precise timing.
    #[serde(with = "string_to_u64")]
    pub exec_time: u64,

    /// The sequence number for the execution.
    ///
    /// Used to ensure executions are processed in order. Bots validate sequence numbers to avoid missing updates.
    pub seq: u64,
}
