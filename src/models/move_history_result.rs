use crate::prelude::*;

/// Summarizes position move history data.
///
/// Part of the `MoveHistoryResponse`, this struct organizes transfer history by pagination cursor and a list of transfer records. Bots use this to process and audit position transfers.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MoveHistoryResult {
    /// A list of position move records.
    ///
    /// Contains detailed transfer data, such as symbol, price, and status. Bots use this to audit transfers and verify portfolio changes.
    pub list: Vec<MoveHistoryEntry>,

    /// The cursor for pagination.
    ///
    /// Indicates the next page of results for large datasets. Bots should use this for paginated requests to fetch additional transfer history.
    #[serde(rename = "nextPageCursor")]
    pub next_page_cursor: String,
}
