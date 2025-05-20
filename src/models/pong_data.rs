use crate::prelude::*;

/// Data structure for WebSocket pong responses.
///
/// Contains details of a pong response, including connection status and request metadata. Bots use this to confirm WebSocket connection health and manage reconnection logic.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PongData {
    /// The return code for the pong response (optional).
    ///
    /// Indicates the status of the pong response (e.g., `0` for success). Bots should check this to verify connection health.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    /// Whether the pong response was successful (optional).
    ///
    /// If `true`, the WebSocket connection is healthy. Bots use this to confirm successful ping-pong cycles.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// A message describing the pong response.
    ///
    /// Typically `"pong"` or an error message. Bots should log this for debugging connection issues.
    pub ret_msg: String,
    /// The WebSocket connection ID.
    ///
    /// A unique identifier for the WebSocket connection. Bots use this to track specific connections in multi-connection setups.
    pub conn_id: String,
    /// The request ID for the ping (optional).
    ///
    /// The ID of the ping request that triggered this pong. Bots use this to correlate ping-pong pairs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub req_id: Option<String>,
    /// Additional arguments for the pong response (optional).
    ///
    /// Contains any additional data included in the pong, typically empty. Bots can ignore this unless handling custom WebSocket protocols.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
    /// Additional data for the pong response (optional).
    ///
    /// Contains any extra data included in the pong, typically empty. Bots can ignore this unless handling custom WebSocket protocols.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<String>>,
    /// The operation type (e.g., "pong").
    ///
    /// Specifies the WebSocket operation, typically `"pong"`. Bots use this to confirm the response type.
    pub op: String,
}
