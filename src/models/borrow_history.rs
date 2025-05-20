use crate::prelude::*;

/// Summarizes borrow history data for an account.
///
/// Part of the `BorrowHistoryResponse`, this struct organizes borrowing data by pagination cursor and a list of borrowing records. Bots use this to process borrowing activity and calculate interest costs.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BorrowHistory {
    /// The cursor for pagination.
    ///
    /// Indicates the next page of results for large datasets. Bots should use this for paginated requests to fetch additional borrowing history.
    pub next_page_cursor: String,

    /// A list of borrowing records.
    ///
    /// Contains detailed borrowing data, such as amounts, interest rates, and timestamps. Bots use this to audit borrowing activity and manage leverage.
    pub rows: Vec<BorrowHistoryEntry>,
}
