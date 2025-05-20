use crate::prelude::*;

/// Represents detailed information about a single position.
///
/// Returned as part of the `InfoResponse`, this struct provides comprehensive position data, including size, entry price, leverage, and P&L. Bots use this to monitor and manage open positions in perpetual futures.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PositionInfo {
    /// The position index (e.g., 0 for one-way mode, 1 or 2 for hedge mode).
    ///
    /// Indicates the position type in Bybit’s position management system. Bots use this to distinguish between long and short positions in hedge mode for perpetual futures.
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
    /// The trading pair symbol (e.g., "BTCUSDT").
    ///
    /// Identifies the perpetual futures contract for the position. Bots should verify this matches the expected symbol.
    pub symbol: String,
    /// The position side (Buy, Sell, or None).
    ///
    /// Indicates whether the position is long (Buy), short (Sell), or closed (None). Bots use this to track position direction and calculate net exposure.
    #[serde(deserialize_with = "empty_string_as_none")]
    pub side: Option<Side>,
    /// The position size (in base asset).
    ///
    /// The quantity of the base asset held in the position (e.g., BTC in `BTCUSDT`). Bots use this to calculate position value and risk exposure.
    #[serde(with = "string_to_float")]
    pub size: f64,
    /// The average entry price of the position (optional).
    ///
    /// The average price at which the position was opened. Bots use this to calculate unrealized P&L and assess position profitability.
    #[serde(with = "string_to_float_optional")]
    pub avg_price: Option<f64>,
    /// The position value (optional).
    ///
    /// The monetary value of the position (`size` * `avg_price`). Bots use this to calculate margin requirements and exposure in the settlement currency.
    #[serde(with = "string_to_float_optional")]
    pub position_value: Option<f64>,
    /// The trade mode (0 for cross margin, 1 for isolated margin).
    ///
    /// Indicates whether the position uses cross margin (shared across positions) or isolated margin (specific to this position). Bots use this to manage margin allocation strategies.
    pub trade_mode: i32,
    /// The position status (e.g., "Normal", "Liq").
    ///
    /// Indicates the current state of the position, such as active or in liquidation. Bots use this to trigger risk management actions if the position is at risk.
    pub position_status: String,
    /// Auto-margin addition status (0 or 1).
    ///
    /// Indicates whether auto-margin addition is enabled (`1`) or disabled (`0`). Bots use this to monitor margin settings and prevent unexpected margin calls.
    pub auto_add_margin: i32,
    /// The Auto-Deleveraging (ADL) rank indicator.
    ///
    /// Indicates the position’s priority for auto-deleveraging in case of market stress. Higher ranks (e.g., 4) are deleveraged first. Bots use this to assess liquidation risk.
    pub adl_rank_indicator: i32,
    /// The leverage applied to the position (optional).
    ///
    /// The leverage multiplier (e.g., `10` for 10x). Bots use this to calculate margin requirements and assess risk exposure.
    #[serde(with = "string_to_float_optional")]
    pub leverage: Option<f64>,
    /// The position’s margin balance.
    ///
    /// The amount of margin allocated to the position. Bots use this to monitor margin health and prevent liquidation.
    #[serde(with = "string_to_float")]
    pub position_balance: f64,
    /// The mark price of the position.
    ///
    /// The current mark price used for P&L calculations in perpetual futures. Bots use this to calculate unrealized P&L and assess position health.
    #[serde(with = "string_to_float")]
    pub mark_price: f64,
    /// The liquidation price (optional).
    ///
    /// The price at which the position will be liquidated. Bots use this to set stop-loss orders or trigger risk management actions.
    #[serde(with = "string_to_float_optional")]
    pub liq_price: Option<f64>,
    /// The bankruptcy price (optional).
    ///
    /// The price at which the position would result in account bankruptcy. Bots use this as a critical risk threshold for position management.
    #[serde(with = "string_to_float_optional")]
    pub bust_price: Option<f64>,
    /// The maintenance margin for the position (optional).
    ///
    /// The minimum margin required to maintain the position. Bots use this to calculate margin ratios and avoid liquidation.
    #[serde(rename = "positionMM", with = "string_to_float_optional")]
    pub position_mm: Option<f64>,
    /// The initial margin for the position (optional).
    ///
    /// The initial margin required to open the position. Bots use this to calculate leverage and margin utilization.
    #[serde(rename = "positionIM", with = "string_to_float_optional")]
    pub position_im: Option<f64>,
    /// The take-profit/stop-loss mode (e.g., "Full", "Partial").
    ///
    /// Indicates whether take-profit and stop-loss orders are applied to the entire position (`Full`) or partially. Bots use this to manage exit strategies.
    pub tpsl_mode: String,
    /// The take-profit price (optional).
    ///
    /// The price at which the position will automatically close for a profit. Bots use this to implement automated exit strategies.
    #[serde(with = "string_to_float_optional")]
    pub take_profit: Option<f64>,
    /// The stop-loss price (optional).
    ///
    /// The price at which the position will automatically close to limit losses. Bots use this to manage downside risk.
    #[serde(with = "string_to_float_optional")]
    pub stop_loss: Option<f64>,
    /// The trailing stop value.
    ///
    /// The trailing stop offset, if enabled. Bots use this to implement dynamic stop-loss strategies that follow market movements.
    pub trailing_stop: String,
    /// The unrealized profit and loss (optional).
    ///
    /// The current unrealized P&L for the position, based on the mark price. Bots use this to monitor position profitability in real time.
    #[serde(with = "string_to_float_optional")]
    pub unrealised_pnl: Option<f64>,
    /// The cumulative realized profit and loss (optional).
    ///
    /// The total realized P&L for the position from all executions. Bots use this to track historical performance.
    #[serde(with = "string_to_float_optional")]
    pub cum_realised_pnl: Option<f64>,
    /// The sequence number of the position update.
    ///
    /// A unique sequence ID for ordering position updates. Bots use this to ensure proper chronological processing of position data.
    pub seq: i64,
    /// Indicates if the position is reduce-only.
    ///
    /// `true` if the position can only be reduced (e.g., closing trades only). Bots use this to enforce position management rules.
    pub is_reduce_only: bool,
    /// The timestamp of the last margin maintenance update (optional).
    ///
    /// Indicates when the margin maintenance requirements were last updated. Bots use this to monitor margin health over time.
    #[serde(with = "string_to_u64_optional")]
    pub mmr_sys_updated_time: Option<u64>,
    /// The timestamp of the last leverage update (optional).
    ///
    /// Indicates when the leverage settings were last updated. Bots use this to track changes in risk exposure.
    #[serde(with = "string_to_u64_optional")]
    pub leverage_sys_updated_time: Option<u64>,
    /// The timestamp when the position was created.
    ///
    /// Indicates when the position was opened. Bots use this to calculate position duration and align with other time-series data.
    #[serde(with = "string_to_u64")]
    pub created_time: u64,
    /// The timestamp of the last position update.
    ///
    /// Indicates when the position was last modified (e.g., size or margin changes). Bots use this to track position changes in real time.
    #[serde(with = "string_to_u64")]
    pub updated_time: u64,
}
