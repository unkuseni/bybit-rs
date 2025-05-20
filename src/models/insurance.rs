use crate::prelude::*;

/// Represents an insurance fund entry for a specific cryptocurrency.
/// Details the fund’s balance and value, which backstops losses in perpetual futures trading.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Insurance {
    /// The cryptocurrency of the insurance fund (e.g., "BTC").
    /// Specifies which asset’s fund is being described. Bots must match this with trading pairs.
    pub coin: String,
    /// The balance of the insurance fund in the specified coin (e.g., 0.5 for 0.5 BTC).
    /// A larger balance indicates a stronger buffer against liquidations. Bots monitor this to
    /// gauge exchange stability, as a depleted fund increases counterparty risk.
    #[serde(with = "string_to_float")]
    pub balance: f64,
    /// The USD value of the fund balance (e.g., "10000").
    /// Provides a standardized valuation, useful for comparing fund sizes across coins. Bots use
    /// this to assess the fund’s adequacy relative to market volatility.
    pub value: String,
}
