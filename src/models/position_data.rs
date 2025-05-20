use crate::prelude::*;

/// Represents detailed data about a single position in Bybit's perpetual futures market.
///
/// This struct contains comprehensive information about a position, including size, leverage, profit/loss, and risk parameters.
///
/// # Bybit API Reference
/// Part of the position WebSocket stream (https://bybit-exchange.github.io/docs/v5/websocket/private/position).
///
/// # Perpetual Futures Context
/// Position data is critical for risk management in futures trading. Bots use this to monitor margin requirements, unrealized PnL, and liquidation risks.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PositionData {
    /// The position index (0 for one-way mode, 1 for buy-side hedge, 2 for sell-side hedge).
    ///
    /// Indicates the position mode. Bots must handle different indices for hedged vs. one-way positions to manage risk accurately.
    pub position_idx: u8,

    /// The trading mode (0 for regular, 1 for isolated margin).
    ///
    /// Specifies whether the position uses cross or isolated margin. Bots need to adjust margin calculations based on this mode.
    pub trade_mode: u8,

    /// The risk ID for the position's risk limit.
    ///
    /// Corresponds to Bybit's risk limit tiers, which define margin requirements. Bots should monitor this to ensure compliance with risk limits.
    pub risk_id: u8,

    /// The risk limit value (as a string, e.g., "1000000").
    ///
    /// Represents the maximum exposure allowed for the risk tier. Bots use this to cap position sizes and avoid margin violations.
    pub risk_limit_value: String,

    /// The trading pair symbol (e.g., "BTCUSDT").
    ///
    /// Identifies the market for the position. Bots filter by symbol to manage specific markets.
    pub symbol: String,

    /// The side of the position ("Buy" for long, "Sell" for short).
    ///
    /// Indicates the direction of the position. Bots use this to calculate exposure and hedge positions.
    pub side: Side,

    /// The size of the position (in contracts or base currency).
    ///
    /// Represents the volume of the position. Bots monitor this to manage exposure and ensure it aligns with strategy limits.
    #[serde(with = "string_to_float")]
    pub size: f64,

    /// The average entry price of the position.
    ///
    /// The price at which the position was opened. Bots use this to calculate unrealized PnL and set stop-loss/take-profit levels.
    #[serde(with = "string_to_float")]
    pub entry_price: f64,

    /// The leverage used for the position (as a string, e.g., "10").
    ///
    /// Indicates the leverage multiplier (e.g., 10x). Higher leverage increases potential returns but also liquidation risk. Bots must adjust position sizes based on leverage to manage risk.
    pub leverage: String,

    /// The total value of the position (in quote currency).
    ///
    /// Calculated as size * entry price. Bots use this to assess exposure and margin requirements.
    #[serde(with = "string_to_float")]
    pub position_value: f64,

    /// The margin allocated to the position.
    ///
    /// Represents the funds reserved for the position. Bots monitor this to ensure sufficient margin and avoid liquidation.
    #[serde(with = "string_to_float")]
    pub position_balance: f64,

    /// The current mark price of the position.
    ///
    /// The fair value of the asset, used for PnL and margin calculations. Bots use this to track unrealized PnL and liquidation risks.
    #[serde(with = "string_to_float")]
    pub mark_price: f64,

    /// The initial margin requirement (as a string, e.g., "1000").
    ///
    /// The minimum margin needed to open the position. Bots use this to calculate capital requirements and optimize margin usage.
    #[serde(rename = "positionIM")]
    pub position_im: String,

    /// The maintenance margin requirement (as a string, e.g., "500").
    ///
    /// The minimum margin needed to maintain the position. Falling below this triggers liquidation. Bots must monitor this closely to avoid forced closures.
    #[serde(rename = "positionMM")]
    pub position_mm: String,

    /// The take-profit price for the position.
    ///
    /// The price at which the position will automatically close for a profit. Bots can set this to lock in gains and automate exits.
    #[serde(with = "string_to_float")]
    pub take_profit: f64,

    /// The stop-loss price for the position.
    ///
    /// The price at which the position will automatically close to limit losses. Bots use this to manage downside risk.
    #[serde(with = "string_to_float")]
    pub stop_loss: f64,

    /// The trailing stop price (optional).
    ///
    /// A dynamic stop-loss that follows the market price. Bots can use this for trend-following strategies to lock in profits while allowing upside potential.
    #[serde(with = "string_to_float_optional")]
    pub trailing_stop: Option<f64>,

    /// The unrealized profit and loss (as a string, e.g., "100.50").
    ///
    /// The current profit or loss based on the mark price. Bots use this to monitor performance and make real-time adjustments.
    #[serde(rename = "unrealisedPnl")]
    pub unrealised_pnl: String,

    /// The cumulative realized profit and loss (as a string, e.g., "500.75").
    ///
    /// The total profit or loss from closed portions of the position. Bots use this to track overall performance.
    #[serde(rename = "cumRealisedPnl")]
    pub cum_realised_pnl: String,

    /// The timestamp when the position was created (in milliseconds).
    ///
    /// Indicates when the position was opened. Bots use this for position aging and strategy timing.
    #[serde(with = "string_to_u64")]
    pub created_time: u64,

    /// The timestamp when the position was last updated (in milliseconds).
    ///
    /// Indicates the most recent change to the position. Bots use this to track updates and ensure data freshness.
    #[serde(with = "string_to_u64")]
    pub updated_time: u64,

    /// The take-profit/stop-loss mode (e.g., "Full" or "Partial").
    ///
    /// Specifies how TP/SL orders are applied. Bots must handle different modes to execute correct exit strategies.
    #[serde(rename = "tpslMode")]
    pub tpsl_mode: String,

    /// The liquidation price for the position.
    ///
    /// The price at which the position will be forcibly closed due to insufficient margin. Bots must monitor this to avoid unexpected liquidations, especially in volatile markets.
    #[serde(with = "string_to_float")]
    pub liq_price: f64,

    /// The bankruptcy price for the position (optional).
    ///
    /// The price at which the position's margin is completely depleted, leading to a total loss. This is critical for bots to monitor, as it represents the worst-case scenario. Bots should use this to set conservative stop-losses and avoid catastrophic losses.
    #[serde(with = "string_to_float_optional")]
    pub bust_price: Option<f64>,

    /// The category of the position (e.g., "linear" for USDT-margined futures).
    ///
    /// Specifies the contract type. Bots must handle different categories (e.g., linear vs. inverse) due to differences in margin and settlement.
    pub category: Category,

    /// The status of the position (e.g., "Normal").
    ///
    /// Indicates whether the position is active or in a special state (e.g., liquidation). Bots use this to filter active positions for management.
    pub position_status: PositionStatus,

    /// The auto-deleveraging (ADL) rank indicator (0-4).
    ///
    /// Indicates the likelihood of the position being auto-deleveraged in extreme market conditions. Higher ranks mean higher risk. Bots should monitor this to reduce exposure in volatile markets.
    pub adl_rank_indicator: u8,

    /// Whether auto-margin addition is enabled (0 for disabled, 1 for enabled).
    ///
    /// If enabled, Bybit automatically adds margin to prevent liquidation. Bots should account for this in margin management to avoid unexpected changes.
    pub auto_add_margin: u8,

    /// The timestamp when the maintenance margin rate was last updated (optional, in milliseconds).
    ///
    /// Indicates when margin requirements changed. Bots can use this to track margin updates and adjust strategies.
    #[serde(with = "string_to_u64_optional")]
    pub mmr_sys_updated_time: Option<u64>,

    /// The timestamp when the leverage was last updated (optional, in milliseconds).
    ///
    /// Indicates when leverage settings changed. Bots should monitor this to ensure leverage aligns with risk parameters.
    #[serde(with = "string_to_u64_optional")]
    pub leverage_sys_updated_time: Option<u64>,

    /// The sequence number for the position update.
    ///
    /// Used to ensure updates are processed in order. Bots should validate sequence numbers to avoid missing or duplicate updates.
    pub seq: u64,

    /// Whether the position is reduce-only.
    ///
    /// If `true`, the position can only reduce size (e.g., close trades). Bots must respect this to avoid placing invalid orders.
    pub is_reduce_only: bool,
}
