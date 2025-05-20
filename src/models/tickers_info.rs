use crate::prelude::*;

/// Summarizes ticker information for a category.
///
/// Part of the `TickerResponse`, this struct organizes ticker data by category and a list of ticker records. Bots use this to monitor multiple trading pairs.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TickersInfo {
    /// The product category (e.g., "linear", "spot").
    ///
    /// Indicates the instrument type. For perpetual futures, this is `linear` or `inverse`. Bots should verify this matches their strategy.
    pub category: Category,

    /// A list of ticker data records.
    ///
    /// Contains the actual market data for each trading pair. Bots iterate over this list to extract prices, volumes, and funding rates for perpetual futures.
    pub list: Vec<TickerData>,
}
