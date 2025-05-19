use crate::prelude::*;

/// Details the result of setting a risk limit.
///
/// Part of the `SetRiskLimitResponse`, this struct provides the applied risk limit details. Bots use this to confirm the new risk settings for a position.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SetRiskLimitResult {
    /// The applied risk ID.
    ///
    /// Confirms the risk limit tier applied to the position. Bots should verify this matches the requested `risk_id`.
    pub risk_id: i32,
    /// The risk limit value.
    ///
    /// The maximum exposure allowed for the position, in the settlement currency. Bots use this to ensure compliance with risk management settings.
    #[serde(with = "string_to_u64")]
    pub risk_limit_value: u64,
    /// The product category (e.g., "linear").
    ///
    /// Indicates the instrument type. Bots should verify this matches the requested `category`.
    pub category: String,
}
