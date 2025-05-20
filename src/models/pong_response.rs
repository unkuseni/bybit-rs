use crate::prelude::*;

/// Enum representing WebSocket pong responses.
///
/// Encapsulates pong responses for public and private WebSocket connections, used to confirm connection health. Bots use this to verify WebSocket connectivity and handle reconnection logic.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum PongResponse {
    /// A pong response for public WebSocket connections.
    ///
    /// Confirms the health of a public WebSocket connection. Bots use this to ensure market data feeds are active.
    PublicPong(PongData),

    /// A pong response for private WebSocket connections.
    ///
    /// Confirms the health of a private WebSocket connection for account data. Bots use this to ensure account feeds are active.
    PrivatePong(PongData),
}
