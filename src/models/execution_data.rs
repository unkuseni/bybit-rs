use crate::prelude::*;

/// Represents detailed data about a single trade execution.
///
/// This struct contains comprehensive information about a trade, including price, quantity, fees, and order details.
///
/// # Bybit API Reference
/// Part of the execution WebSocket stream (https://bybit-exchange.github.io/docs/v5/websocket/private/execution).
///
/// # Perpetual Futures Context
/// Execution data is critical for tracking trade performance and costs. Bots use this to update position sizes, calculate realized PnL, and monitor fees.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionData {
    /// The category of the execution (e.g., "linear").
    ///
    /// Specifies the contract type (e.g., USDT-margined futures). Bots must handle different categories due to varying margin and settlement rules.
    pub category: String,
    /// The trading pair symbol (e.g., "BTCUSDT").
    ///
    /// Identifies the market for the execution. Bots filter by symbol to process relevant trades.
    pub symbol: String,
    /// The fee charged for the execution.
    ///
    /// Represents the trading cost for the executed trade. Bots must include this in PnL calculations to ensure accurate profitability tracking.
    #[serde(with = "string_to_float")]
    pub exec_fee: f64,
    /// The unique identifier for the execution.
    ///
    /// Used to track individual trades. Bots can use this to match executions with orders.
    pub exec_id: String,
    /// The price at which the trade was executed.
    ///
    /// The actual price of the filled order. Bots use this to update position entry prices and calculate PnL.
    #[serde(with = "string_to_float")]
    pub exec_price: f64,
    /// The quantity executed in the trade.
    ///
    /// The volume of the trade (in contracts or base currency). Bots use this to update position sizes and track order fills.
    #[serde(with = "string_to_float")]
    pub exec_qty: f64,
    /// The type of execution (e.g., "Trade", "Bust").
    ///
    /// Indicates the nature of the execution (e.g., normal trade or liquidation). Bots must handle different types appropriately.
    pub exec_type: String,
    /// The value of the executed trade (in quote currency).
    ///
    /// Calculated as `exec_price * exec_qty`. Bots use this to assess trade size and impact on margin.
    #[serde(with = "string_to_float")]
    pub exec_value: f64,
    /// Whether the execution was a maker order.
    ///
    /// If `true`, the trade added liquidity and likely incurred a lower fee. Bots can use this to optimize order placement for cost efficiency.
    pub is_maker: bool,
    /// The fee rate applied to the execution.
    ///
    /// The percentage fee charged (e.g., 0.0006 for 0.06%). Bots use this to verify fee calculations and optimize trading costs.
    #[serde(rename = "feeRate", with = "string_to_float")]
    pub fee_rate: f64,
    /// The implied volatility for the trade (as a string).
    ///
    /// Relevant for options trading. For futures bots, this may be less critical but can indicate market expectations of volatility.
    #[serde(rename = "tradeIv")]
    pub trade_iv: String,
    /// The mark implied volatility (as a string).
    ///
    /// The implied volatility based on the mark price. Bots can use this for volatility-based strategies in options or futures.
    #[serde(rename = "markIv")]
    pub mark_iv: String,
    /// The ID of the block trade (if applicable).
    ///
    /// Identifies large trades executed off the order book. Bots can monitor this for institutional activity that may impact prices.
    #[serde(rename = "blockTradeId")]
    pub block_trade_id: String,
    /// The current mark price at the time of execution.
    ///
    /// Used for PnL and margin calculations. Bots use this to track unrealized PnL and liquidation risks.
    #[serde(with = "string_to_float")]
    pub mark_price: f64,
    /// The index price at the time of execution.
    ///
    /// The reference price for the asset. Bots use this to calculate funding rates and fair value.
    #[serde(with = "string_to_float")]
    pub index_price: f64,
    /// The underlying price at the time of execution.
    ///
    /// The price of the underlying asset (e.g., spot price). Bots use this for arbitrage or hedging strategies.
    #[serde(with = "string_to_float")]
    pub underlying_price: f64,
    /// The remaining quantity to be filled for the order.
    ///
    /// Indicates how much of the order is still open. Bots use this to track partial fills and manage open orders.
    #[serde(with = "string_to_float")]
    pub leaves_qty: f64,
    /// The ID of the order associated with the execution.
    ///
    /// Links the execution to the original order. Bots use this to track order status and fills.
    pub order_id: String,
    /// The user-defined ID for the order.
    ///
    /// Allows bots to assign custom identifiers to orders for internal tracking.
    pub order_link_id: String,
    /// The price specified in the order.
    ///
    /// The target price for the order (e.g., limit order price). Bots use this to verify execution prices against order prices.
    #[serde(with = "string_to_float")]
    pub order_price: f64,
    /// The total quantity specified in the order.
    ///
    /// The full size of the order. Bots use this to calculate fill percentages and manage order execution.
    #[serde(with = "string_to_float")]
    pub order_qty: f64,
    /// The type of order (e.g., "Limit", "Market").
    ///
    /// Specifies the order placement method. Bots use this to determine execution behavior and strategy alignment.
    pub order_type: String,
    /// The type of stop order (if applicable, e.g., "StopLoss").
    ///
    /// Indicates if the order was a conditional stop order. Bots use this to manage risk and automate exits.
    #[serde(rename = "stopOrderType")]
    pub stop_order_type: String,
    /// The side of the order ("Buy" or "Sell").
    ///
    /// Indicates the direction of the trade. Bots use this to update position direction and exposure.
    pub side: String,
    /// The timestamp when the execution occurred (in milliseconds).
    ///
    /// Indicates the exact time of the trade. Bots use this for precise timing and correlation with market data.
    #[serde(with = "string_to_u64")]
    pub exec_time: u64,
    /// Whether leverage was used (as a string, e.g., "1").
    ///
    /// Indicates if the trade used borrowed funds. Bots use this to adjust margin and risk calculations.
    pub is_leverage: String,
    /// The size of the position closed by the execution (as a string).
    ///
    /// Relevant for closing trades. Bots use this to update position sizes and track closures.
    pub closed_size: String,
    /// The sequence number for the execution.
    ///
    /// Used to ensure executions are processed in order. Bots validate sequence numbers to avoid missing updates.
    pub seq: u64,
}
