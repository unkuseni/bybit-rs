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
    #[serde(with = "string_to_float")]
    pub price_limit_ratio_x: f64,
    /// The market order risk parameter.
    ///
    /// Defines risk constraints for market orders in spot trading. Not relevant for perpetuals.
    #[serde(with = "string_to_float")]
    pub price_limit_ratio_y: f64,
}
