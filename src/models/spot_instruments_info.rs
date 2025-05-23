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
    pub category: String,

    /// A list of spot instrument details.
    ///
    /// Contains data for each spot trading pair. Not relevant for perpetual futures.
    pub list: Vec<SpotInstrument>,
}
