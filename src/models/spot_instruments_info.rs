use crate::prelude::*;

/// Contains instrument information for spot markets.
///
/// Part of the `InstrumentInfo` enum, this struct provides details for spot trading pairs. Less relevant for perpetual futures but included for completeness.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SpotInstrumentsInfo {
    /// The product category (e.g., "spot").
    ///
    /// Confirms the instrument type as spot. Bots trading perpetuals can ignore this.
    pub category: Category,

    /// A list of spot instrument details.
    ///
    /// Contains data for each spot trading pair. Not relevant for perpetual futures.
    pub list: Vec<SpotInstrument>,

    /// The cursor for pagination.
    ///
    /// Used for paginated requests. Bots can ignore this unless querying spot instruments.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub next_page_cursor: String,
}
