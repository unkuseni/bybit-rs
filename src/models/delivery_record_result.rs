use crate::prelude::*;
/// Represents the result of a delivery record query.
///
/// This struct contains the category of the records, a list of delivery records,
/// and a cursor for pagination to fetch the next page of results.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryRecordResult {
    /// The category of the delivery records (e.g., futures, options).
    pub category: Category,

    /// A list of delivery records.
    pub list: Vec<DeliveryRecord>,

    /// A cursor for pagination to fetch the next page of results.
    pub next_page_cursor: String,
}
