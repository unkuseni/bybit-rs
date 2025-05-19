use crate::prelude::*;

/// Structure for trade stream events received via WebSocket.
///
/// Contains real-time trade stream data, including order status updates and API rate limit metadata. Bots use this for high-frequency trading and low-latency order monitoring in perpetual futures markets.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TradeStreamEvent {
    /// The request ID for the trade stream event (optional).
    ///
    /// A unique identifier for the request that triggered this event, if applicable. Bots use this to correlate events with specific requests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub req_id: Option<String>,
    /// The return code for the event.
    ///
    /// Indicates the status of the event (e.g., `0` for success). Bots should check this to handle errors or confirm successful updates.
    pub ret_code: i32,
    /// A message describing the event.
    ///
    /// Provides context for the `ret_code`, such as `"OK"` or an error description. Bots should log this for debugging.
    pub ret_msg: String,
    /// The operation type (e.g., "trade").
    ///
    /// Specifies the type of trade stream event, typically `"trade"`. Bots use this to confirm the event type.
    pub op: String,
    /// The order status data for the event.
    ///
    /// Contains detailed order status updates, such as filled or cancelled orders. Bots use this to track order progress in real time.
    pub data: OrderStatus,
    /// The API response header.
    ///
    /// Contains rate limit and timing metadata for the API response. Bots use this to manage API usage and ensure compliance with rate limits.
    pub header: Header,
    /// The WebSocket connection ID.
    ///
    /// A unique identifier for the WebSocket connection. Bots use this to track specific connections in multi-connection setups.
    pub conn_id: String,
}
