use crate::prelude::*;

/// Summarizes risk limit data for a trading category.
/// Defines constraints on position size and leverage in perpetual futures trading.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RiskLimitSummary {
    /// The trading category (e.g., "linear" for USDT-margined perpetuals).
    /// Matches the request category, ensuring data relevance. Bots use this to filter results.
    pub category: String,

    /// List of risk limit configurations.
    /// Each entry specifies leverage and margin requirements for a symbol, used by bots to
    /// manage trading constraints.
    pub list: Vec<RiskLimit>,
}
