use crate::prelude::*;

/// Summarizes transaction log data for an account.
///
/// Part of the `TransactionLogResponse`, this struct organizes transaction data by pagination cursor and a list of transaction records. Bots use this to process and audit trading activity.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TransactionLogResult {
    /// The cursor for pagination.
    ///
    /// Indicates the next page of results for large datasets. Bots should use this for paginated requests to fetch additional transaction logs.
    pub next_page_cursor: String,
    /// A list of transaction log entries.
    ///
    /// Contains detailed transaction data, such as trades, fees, and funding. Bots use this to analyze trading activity and calculate performance metrics.
    pub list: Vec<TransactionLogEntry>,
}
