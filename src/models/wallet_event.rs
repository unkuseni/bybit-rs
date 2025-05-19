use crate::prelude::*;

/// Represents an event containing wallet updates from Bybit’s WebSocket API.
///
/// This struct delivers real-time wallet balance and margin updates via Bybit’s private WebSocket stream (https://bybit-exchange.github.io/docs/v5/websocket/private/wallet).
/// In perpetual futures, wallet events are essential for monitoring account health, margin levels, and funding requirements, especially in leveraged trading.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WalletEvent {
    /// Unique identifier for the WebSocket event.
    ///
    /// Similar to `OrderEvent.id`, this tracks the event for debugging and message integrity.
    /// **Bot Implication**: Bots use `id` to ensure all wallet updates are processed and to troubleshoot WebSocket issues.
    pub id: String,
    /// The WebSocket topic, e.g., "wallet".
    ///
    /// This identifies the event as a wallet update, allowing bots to route it to the appropriate handler (https://bybit-exchange.github.io/docs/v5/websocket/private/wallet).
    /// **Bot Implication**: Bots filter on `topic` to prioritize wallet updates, critical for margin and risk management.
    pub topic: String,
    /// Timestamp of the event, in milliseconds since Unix epoch.
    ///
    /// This records when the wallet update was generated, aiding in latency and sequencing analysis (https://bybit-exchange.github.io/docs/v5/websocket/private/wallet).
    /// **Bot Implication**: Bots use `creation_time` to ensure timely margin adjustments and to monitor system performance.
    pub creation_time: u64,
    /// List of wallet data updates included in the event.
    ///
    /// This contains one or more `WalletData` structs, each representing updated wallet metrics (https://bybit-exchange.github.io/docs/v5/websocket/private/wallet).
    /// **Bot Implication**: Bots process `data` to update account balances and margin ratios, ensuring compliance with Bybit’s risk controls.
    pub data: Vec<WalletData>,
}
