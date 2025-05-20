use crate::prelude::*;

/// The status of a position on Bybit exchange.
/// Either normal, liquidation in progress, or auto-deleverage in progress.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PositionStatus {
    /// Position is normal
    Normal,

    /// Liquidation is in progress
    #[serde(rename = "Liq")]
    LiquidationInProgress,

    /// Auto-deleverage is in progress
    #[serde(rename = "Adl")]
    AutoDeleverageInProgress,
}

impl Default for PositionStatus {
    fn default() -> Self {
        Self::Normal
    }
}
