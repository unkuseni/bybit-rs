use crate::prelude::*;

/// Enum representing ticker data for different instrument types.
///
/// This untagged enum allows the API to return ticker data for either spot or futures (including perpetuals). For perpetual futures, the `Futures` variant is most relevant, containing funding rate and open interest data.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum TickerData {
    /// Ticker data for spot markets.
    ///
    /// Contains market data for spot trading pairs. Not relevant for perpetual futures.
    Spot(SpotTicker),
    /// Ticker data for futures (including perpetuals).
    ///
    /// Contains market data for perpetual futures, including funding rates and open interest. Critical for bots monitoring market conditions and funding costs.
    Futures(FuturesTicker),
}
