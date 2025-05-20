use crate::prelude::*;

/// Parameters for setting risk limits for a position.
///
/// Used to construct a request to the `/v5/position/set-risk-limit` endpoint to adjust risk limits for a specific symbol. Bots use this to control maximum exposure and manage liquidation risk in perpetual futures.
#[derive(Clone, Default)]
pub struct SetRiskLimit<'a> {
    /// The product category (e.g., Linear, Inverse).
    ///
    /// Specifies the instrument type. Bots must set this to target the correct contract type.
    pub category: Category,

    /// The trading pair symbol (e.g., "BTCUSDT").
    ///
    /// Identifies the perpetual futures contract for which risk limits are being set. Bots must specify a valid symbol.
    pub symbol: Cow<'a, str>,

    /// The risk ID to apply.
    ///
    /// Identifies the risk limit tier to use, corresponding to specific margin and exposure limits. Bots should select a risk ID that aligns with their risk management strategy.
    pub risk_id: i8,

    /// The position index (optional).
    ///
    /// Specifies the position type (e.g., 0 for one-way mode, 1 or 2 for hedge mode). Bots should set this for hedge mode positions to target the correct side.
    pub position_idx: Option<i32>,
}
