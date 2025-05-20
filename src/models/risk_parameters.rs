use crate::prelude::*;

/// Risk parameters for a spot instrument.
///
/// Specifies risk-related constraints for spot trading. Not relevant for perpetual futures.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RiskParameters {
    /// The limit order risk parameter.
    ///
    /// Defines risk constraints for limit orders in spot trading. Not relevant for perpetuals.
    pub limit_parameter: String,
    /// The market order risk parameter.
    ///
    /// Defines risk constraints for market orders in spot trading. Not relevant for perpetuals.
    pub market_parameter: String,
}
