use crate::prelude::*;

/// Summarizes delivery price data for futures contracts.
/// Includes pagination details and a list of delivery prices.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryPriceSummary {
    /// The trading category (e.g., "linear" for USDT-margined contracts).
    /// Specifies the contract type. Bots use this to filter relevant data.
    pub category: String,

    /// Cursor for fetching the next page of results (if any).
    /// Used for pagination in large datasets. Bots must handle this to retrieve complete data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,

    /// List of delivery price records.
    /// Each entry provides settlement details for a futures contract, useful for historical
    /// analysis in trading bots.
    pub list: Vec<DeliveryPrice>,
}
