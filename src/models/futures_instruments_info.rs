use crate::prelude::*;

/// Contains instrument information for futures contracts.
///
/// Part of the `InstrumentInfo` enum, this struct provides details for futures, including perpetual futures. It’s critical for bots trading perpetuals to understand leverage, price, and lot size constraints.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FuturesInstrumentsInfo {
    /// The product category (e.g., "linear", "inverse").
    ///
    /// Confirms the type of futures contract. For perpetuals, this is `linear` (USDT-margined) or `inverse` (coin-margined). Bots should verify this matches their trading strategy.
    pub category: Category,

    /// A list of futures instrument details.
    ///
    /// Contains the actual instrument data for each futures contract. Bots iterate over this list to extract parameters like leverage and funding intervals for perpetual futures.
    pub list: Vec<FuturesInstrument>,

    /// The cursor for pagination.
    ///
    /// Indicates the next page of results for large datasets. Bots should use this for paginated requests to fetch all instruments when `limit` is reached.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub next_page_cursor: String,
}
