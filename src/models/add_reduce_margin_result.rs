use crate::prelude::*;

/// Details the result of a margin adjustment for a position.
///
/// Part of the `AddReduceMarginResponse`, this struct provides updated position metrics after adding or reducing margin. Bots use this to confirm the new position state and update risk management calculations.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AddReduceMarginResult {
    /// The product category (e.g., Linear).
    ///
    /// Indicates the instrument type of the position. Bots should verify this matches the requested `category`.
    pub category: Category,

    /// The trading pair symbol (e.g., "BTCUSDT").
    ///
    /// Identifies the perpetual futures contract. Bots should confirm this matches the requested symbol.
    pub symbol: String,

    /// The position index (e.g., 0 for one-way mode, 1 or 2 for hedge mode).
    ///
    /// Indicates the position type. Bots use this to distinguish between long and short positions in hedge mode.
    pub position_idx: i32,

    /// The risk ID associated with the position.
    ///
    /// Identifies the risk limit tier applied to the position. Bots use this to verify compliance with risk management settings.
    pub risk_id: i32,

    /// The risk limit value for the position.
    ///
    /// The maximum exposure allowed for the position, in the settlement currency. Bots use this to ensure positions stay within risk limits.
    #[serde(with = "string_to_float")]
    pub risk_limit_value: f64,

    /// The position size (in base asset).
    ///
    /// The quantity of the base asset held in the position. Bots use this to calculate position value and risk exposure.
    #[serde(with = "string_to_float")]
    pub size: f64,

    /// The position value.
    ///
    /// The monetary value of the position (`size` * `avg_price`). Bots use this to calculate margin requirements and exposure.
    #[serde(with = "string_to_float")]
    pub position_value: f64,

    /// The average entry price of the position.
    ///
    /// The average price at which the position was opened. Bots use this to calculate unrealized P&L and assess profitability.
    #[serde(with = "string_to_float")]
    pub avg_price: f64,

    /// The liquidation price.
    ///
    /// The price at which the position will be liquidated. Bots use this to set stop-loss orders or trigger risk management actions.
    #[serde(with = "string_to_float")]
    pub liq_price: f64,

    /// The bankruptcy price.
    ///
    /// The price at which the position would result in account bankruptcy. Bots use this as a critical risk threshold.
    #[serde(with = "string_to_float")]
    pub bust_price: f64,

    /// The mark price of the position.
    ///
    /// The current mark price used for P&L calculations in perpetual futures. Bots use this to calculate unrealized P&L and assess position health.
    #[serde(with = "string_to_float")]
    pub mark_price: f64,

    /// The leverage applied to the position.
    ///
    /// The leverage multiplier (e.g., "10" for 10x). Bots use this to calculate margin requirements and assess risk exposure.
    pub leverage: String,

    /// Auto-margin addition status (0 or 1).
    ///
    /// Indicates whether auto-margin addition is enabled (`1`) or disabled (`0`). Bots use this to monitor margin settings.
    pub auto_add_margin: i32,

    /// The position status (e.g., "Normal", "Liq").
    ///
    /// Indicates the current state of the position, such as active or in liquidation. Bots use this to trigger risk management actions if needed.
    pub position_status: PositionStatus,

    /// The initial margin for the position.
    ///
    /// The initial margin required to maintain the position. Bots use this to calculate leverage and margin utilization.
    #[serde(rename = "positionIM")]
    pub position_im: String,

    /// The maintenance margin for the position.
    ///
    /// The minimum margin required to avoid liquidation. Bots use this to calculate margin ratios and manage risk.
    #[serde(rename = "positionMM")]
    pub position_mm: String,

    /// The unrealized profit and loss.
    ///
    /// The current unrealized P&L for the position, based on the mark price. Bots use this to monitor position profitability in real time.
    pub unrealised_pnl: String,

    /// The cumulative realized profit and loss.
    ///
    /// The total realized P&L for the position from all executions. Bots use this to track historical performance.
    pub cum_realised_pnl: String,

    /// The stop-loss price (optional).
    ///
    /// The price at which the position will automatically close to limit losses. Bots use this to verify stop-loss settings.
    #[serde(with = "string_to_float_optional")]
    pub stop_loss: Option<f64>,

    /// The take-profit price (optional).
    ///
    /// The price at which the position will automatically close for a profit. Bots use this to verify take-profit settings.
    #[serde(with = "string_to_float_optional")]
    pub take_profit: Option<f64>,

    /// The trailing stop value.
    ///
    /// The trailing stop offset, if enabled. Bots use this to verify dynamic stop-loss settings that follow market movements.
    pub trailing_stop: String,

    /// The timestamp when the position was created.
    ///
    /// Indicates when the position was opened. Bots use this to calculate position duration and align with other time-series data.
    #[serde(with = "string_to_u64")]
    pub created_time: u64,

    /// The timestamp of the last position update.
    ///
    /// Indicates when the position was last modified (e.g., margin or size changes). Bots use this to track position changes in real time.
    #[serde(with = "string_to_u64")]
    pub updated_time: u64,
}
