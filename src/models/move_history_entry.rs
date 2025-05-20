use crate::prelude::*;

/// Represents a single position move history record.
///
/// Details a specific position transfer between accounts, including trade parameters and status. Bots use this to audit individual transfers and diagnose issues.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MoveHistoryEntry {
    /// The block trade ID for the transfer.
    ///
    /// A unique identifier for the position move, treated as a block trade. Bots use this to track specific transfer events.
    pub block_trade_id: String,

    /// The product category (e.g., "linear").
    ///
    /// Indicates the instrument type of the transferred position. Bots should verify this matches the requested category.
    pub category: String,

    /// The unique order ID.
    ///
    /// Identifies the order associated with the transfer. Bots use this to correlate transfers with specific orders.
    pub order_id: String,

    /// The user ID of the account involved.
    ///
    /// Indicates the account (source or destination) associated with the transfer. Bots use this to verify the correct accounts were involved.
    #[serde(rename = "userId")]
    pub user_id: u64,

    /// The trading pair symbol (e.g., "BTCUSDT").
    ///
    /// Identifies the perpetual futures contract for the transferred position. Bots should confirm this matches the requested symbol.
    pub symbol: String,

    /// The trade side ("Buy" or "Sell").
    ///
    /// Indicates whether the transferred position was long (Buy) or short (Sell). Bots use this to verify position direction.
    pub side: String,

    /// The transfer price.
    ///
    /// The price at which the position was transferred, used for valuation. Bots use this to verify the fairness of the transfer price.
    #[serde(with = "string_to_float")]
    pub price: f64,

    /// The quantity of the transferred position.
    ///
    /// The amount of the base asset transferred. Bots use this to verify the correct position size was moved.
    #[serde(with = "string_to_float")]
    pub qty: f64,

    /// The execution fee for the transfer.
    ///
    /// The fee charged for the transfer, in the settlement currency. Bots use this to calculate the net cost of the transfer.
    #[serde(with = "string_to_float")]
    pub exec_fee: f64,

    /// The status of the transfer (e.g., "Filled", "Rejected").
    ///
    /// Indicates whether the transfer was successful or encountered issues. Bots should check this to confirm transfer completion.
    pub status: String,

    /// The unique execution ID.
    ///
    /// A unique identifier for the transfer execution on Bybit’s exchange. Bots use this to track specific transfer events and avoid duplicates.
    pub exec_id: String,

    /// The result code for the transfer.
    ///
    /// A code indicating the outcome of the transfer (e.g., `0` for success). Bots should check this to identify issues with specific transfers.
    pub result_code: i16,

    /// A message describing the transfer result.
    ///
    /// Provides details about the transfer’s status, such as success or error reasons. Bots should log this for debugging and error handling.
    pub result_message: String,

    /// The timestamp when the transfer was created.
    ///
    /// Indicates when the transfer request was initiated. Bots use this to align transfer data with other time-series data.
    pub created_at: u64,

    /// The timestamp when the transfer was last updated.
    ///
    /// Indicates when the transfer status was last modified (e.g., completed or rejected). Bots use this to track transfer progress.
    pub updated_at: u64,

    /// The party that rejected the transfer, if applicable.
    ///
    /// Identifies the account (source or destination) that caused a rejection. Bots use this to diagnose transfer failures.
    pub reject_party: String,
}
