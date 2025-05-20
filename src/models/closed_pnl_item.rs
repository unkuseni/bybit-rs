use crate::prelude::*;

/// Represents a single closed P&L record.
///
/// Details the realized P&L for a closed position, including trade details and profit/loss metrics. Bots use this to analyze individual trade outcomes and assess strategy effectiveness in perpetual futures.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClosedPnlItem {
    /// The trading pair symbol (e.g., "BTCUSDT").
    ///
    /// Identifies the perpetual futures contract for the closed position. Bots should verify this matches the requested symbol.
    pub symbol: String,

    /// The order type (e.g., "Limit", "Market").
    ///
    /// Indicates whether the closing trade was executed via a limit or market order. Bots use this to analyze execution strategy effectiveness.
    pub order_type: String,

    /// The leverage used for the position.
    ///
    /// The leverage multiplier (e.g., "10" for 10x). Bots use this to assess the impact of leverage on realized P&L.
    pub leverage: String,

    /// The timestamp of the last update to the P&L record.
    ///
    /// Indicates when the position was closed or P&L was finalized. Bots use this to align P&L data with other time-series data.
    #[serde(with = "string_to_u64")]
    pub updated_time: u64,

    /// The trade side ("Buy" or "Sell").
    ///
    /// Indicates whether the position was long (Buy) or short (Sell). Bots use this to track position direction and calculate net exposure.
    pub side: Side,

    /// The unique order ID.
    ///
    /// Identifies the order that closed the position. Bots use this to correlate P&L with specific orders in their tracking systems.
    pub order_id: String,

    /// The realized profit and loss for the position.
    ///
    /// The net profit or loss from the closed position, in the settlement currency. Bots use this to calculate overall performance and profitability.
    #[serde(with = "string_to_float")]
    pub closed_pnl: f64,

    /// The average entry price of the position.
    ///
    /// The average price at which the position was opened. Bots use this to calculate the profit/loss relative to the exit price.
    #[serde(with = "string_to_float")]
    pub avg_entry_price: f64,

    /// The total quantity of the position.
    ///
    /// The amount of the base asset in the closed position. Bots use this to calculate position size and P&L magnitude.
    #[serde(with = "string_to_float")]
    pub qty: f64,

    /// The cumulative entry value of the position.
    ///
    /// The total monetary value of the position at entry (`qty` * `avg_entry_price`). Bots use this to assess position size in the settlement currency.
    #[serde(with = "string_to_float")]
    pub cum_entry_value: f64,

    /// The timestamp when the position was created.
    ///
    /// Indicates when the position was opened. Bots use this to calculate position duration and align with other data.
    #[serde(with = "string_to_float")]
    pub created_time: f64,

    /// The order price.
    ///
    /// The price specified in the closing order (for limit orders). Bots use this to compare with `avg_exit_price` to assess slippage.
    #[serde(with = "string_to_float")]
    pub order_price: f64,

    /// The size of the closed position.
    ///
    /// The portion of the position closed, in base asset units. Bots use this to track partial or full position closures.
    #[serde(with = "string_to_float")]
    pub closed_size: f64,

    /// The average exit price of the position.
    ///
    /// The average price at which the position was closed. Bots use this to calculate realized P&L (`closed_pnl` = `closed_size` * (`avg_exit_price` - `avg_entry_price`)).
    #[serde(with = "string_to_float")]
    pub avg_exit_price: f64,

    /// The execution type (e.g., "Trade", "Funding").
    ///
    /// Indicates the nature of the closing execution (e.g., regular trade or funding fee). Bots use this to filter P&L calculations.
    pub exec_type: String,

    /// The number of fills for the closing trade.
    ///
    /// Indicates how many partial executions occurred to close the position. Bots use this to analyze execution efficiency.
    pub fill_count: String,

    /// The cumulative exit value of the position.
    ///
    /// The total monetary value of the position at exit (`closed_size` * `avg_exit_price`). Bots use this to verify P&L calculations.
    #[serde(with = "string_to_float")]
    pub cum_exit_value: f64,
}
