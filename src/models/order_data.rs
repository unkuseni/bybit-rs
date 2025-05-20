use crate::prelude::*;

/// Represents order data for a trading order on Bybit's perpetual futures platform.
///
/// This struct encapsulates all details of an order, including its type, status, and associated financial metrics.
/// It is used in Bybit's WebSocket and REST APIs to convey order information (https://bybit-exchange.github.io/docs/v5/websocket/private/order).
/// In the context of perpetual futures, orders are instructions to buy or sell a contract that tracks the price of an underlying asset without an expiration date.
/// For trading bots, this struct is critical for tracking order execution, managing risk, and implementing strategies like market making or arbitrage.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderData {
    /// The trading pair or contract symbol, e.g., "BTCUSDT" for Bitcoin/Tether perpetual futures.
    ///
    /// In Bybit, the symbol uniquely identifies the perpetual futures contract. Perpetual futures are derivatives that allow traders to speculate on asset prices with leverage, without owning the underlying asset (https://bybit-exchange.github.io/docs/v5/market/instrument).
    /// For trading bots, this field is essential for routing orders to the correct market and ensuring strategy alignment with specific contracts.
    pub symbol: String,

    /// Unique identifier for the order, assigned by Bybit.
    ///
    /// This ID is used to track and manage orders via Bybit's API (https://bybit-exchange.github.io/docs/v5/order/create-order). It allows bots to query order status, cancel orders, or amend them.
    /// In perpetual futures trading, order IDs are critical for maintaining state in high-frequency trading systems, ensuring no duplicate or lost orders.
    pub order_id: String,

    /// Indicates whether the order is a buy or sell, e.g., "Buy" or "Sell".
    ///
    /// The side determines the direction of the trade in the perpetual futures market. A "Buy" order increases exposure to the asset (long position), while a "Sell" order decreases it (short position).
    /// Bots use this field to manage position direction and ensure alignment with market signals or hedging strategies (https://bybit-exchange.github.io/docs/v5/order/order-list).
    pub side: String,

    /// Specifies the type of order, e.g., "Limit", "Market".
    ///
    /// Bybit supports various order types for perpetual futures, such as limit orders (executed at a specified price) and market orders (executed at the best available price).
    /// Bots rely on this field to implement strategies like limit order books for liquidity provision or market orders for immediate execution (https://bybit-exchange.github.io/docs/v5/order/create-order).
    pub order_type: String,

    /// Indicates the reason or type of order cancellation, if applicable.
    ///
    /// This field is relevant when an order is canceled due to conditions like insufficient margin or timeout. In perpetual futures, cancellations can occur due to rapid market movements or risk limits.
    /// For bots, monitoring this field helps diagnose issues like order rejections and adjust strategies to avoid repeated cancellations.
    pub cancel_type: String,

    /// The price at which the order is placed (for limit orders) or executed (for market orders).
    ///
    /// In perpetual futures, the price is critical for calculating entry points and potential profits. Bybit represents prices in USDT or the quote currency (https://bybit-exchange.github.io/docs/v5/order/order-list).
    /// Bots use this field to set precise entry/exit points and manage slippage in volatile markets.
    #[serde(with = "string_to_float")]
    pub price: f64,

    /// The quantity of the contract to be traded, expressed in contracts or lots.
    ///
    /// For perpetual futures on Bybit, quantity represents the size of the position. Larger quantities increase exposure and risk, especially with leverage (https://bybit-exchange.github.io/docs/v5/order/create-order).
    /// Bots use this field to scale positions based on account size, risk tolerance, andავ
    /// The quantity of the contract to be traded, expressed in contracts or lots.
    ///
    /// For perpetual futures on Bybit, quantity represents the size of the position. Larger quantities increase exposure and risk, especially with leverage (https://bybit-exchange.github.io/docs/v5/order/create-order).
    /// Bots use this field to scale positions based on account size, risk tolerance, and market conditions.
    #[serde(with = "string_to_float")]
    pub qty: f64,

    /// Implied volatility associated with the order, if applicable.
    ///
    /// This field is typically used for options or volatility-based derivatives, not standard perpetual futures. It may be empty or unused in this context.
    /// For bots, this field is generally irrelevant unless dealing with Bybit's volatility products, which are less common.
    pub order_iv: String,

    /// Specifies how long the order remains active, e.g., "GTC" (Good Till Cancel), "IOC" (Immediate or Cancel).
    ///
    /// Time in force controls order execution behavior in Bybit's perpetual futures market (https://bybit-exchange.github.io/docs/v5/order/create-order).
    /// Bots use this field to manage order persistence, balancing execution speed with market exposure.
    pub time_in_force: String,

    /// Current status of the order, e.g., "New", "Filled", "Cancelled".
    ///
    /// This field tracks the lifecycle of an order on Bybit (https://bybit-exchange.github.io/docs/v5/order/order-list). In perpetual futures, understanding order status is crucial for managing open positions and avoiding unintended exposures.
    /// Bots monitor this field to confirm executions, handle partial fills, or trigger fallback strategies.
    /// **Order Status Monitoring**: Trading bots heavily rely on real-time order status updates to manage risk and ensure accurate position tracking. For instance, a "Filled" status confirms execution, while "Rejected" may require immediate re-evaluation of the strategy.
    pub order_status: String,

    /// A custom identifier for the order, set by the user or bot.
    ///
    /// This allows bots to attach metadata to orders for internal tracking, such as strategy IDs or batch identifiers. It’s particularly useful in high-frequency trading to correlate orders with specific algorithms or events (https://bybit-exchange.github.io/docs/v5/order/create-order).
    /// **Bot Implication**: Using meaningful `order_link_id` values can streamline reconciliation and debugging, especially when handling thousands of orders daily.
    pub order_link_id: String,

    /// The market price of the symbol when the order was created.
    ///
    /// This field captures the reference price at order placement, useful for auditing and performance analysis in perpetual futures trading. It helps bots assess slippage (the difference between `last_price_on_created` and `price`) and market conditions at the time of order submission.
    /// **Bot Implication**: Bots can use this to evaluate execution quality, especially for market orders where slippage can significantly impact profitability in volatile markets like crypto futures.
    #[serde(with = "string_to_float")]
    pub last_price_on_created: f64,

    /// Indicates if the order reduces an existing position (true) or opens a new one (false).
    ///
    /// In perpetual futures, `reduce_only` orders ensure that the trade only closes or reduces a position, preventing accidental increases in exposure (https://bybit-exchange.github.io/docs/v5/order/create-order).
    /// **Bot Implication**: Bots must set this correctly to avoid unintended leverage increases, which could lead to margin calls or liquidations in highly leveraged markets=1. **Risk Management**: Using `reduce_only` orders is critical for bots managing position sizing to prevent over-leveraging, especially in volatile markets where margin requirements can change rapidly.
    pub reduce_only: bool,

    /// The remaining quantity of the order that has not yet been filled.
    ///
    /// For partially filled orders, this field tracks how much of the original `qty` remains open. In perpetual futures, partial fills are common due to market liquidity constraints (https://bybit-exchange.github.io/docs/v5/order/order-list).
    /// **Bot Implication**: Bots need to monitor `leaves_qty` to decide whether to cancel the remaining order, adjust the price, or wait for further execution, balancing execution risk with market exposure.
    #[serde(with = "string_to_float")]
    pub leaves_qty: f64,

    /// The remaining value of the order, calculated as `leaves_qty` multiplied by the order price.
    ///
    /// This provides the monetary value of the unfilled portion of the order, useful for assessing exposure in USDT or the quote currency.
    /// **Bot Implication**: Bots use `leaves_value` to quantify the capital tied up in open orders, aiding in capital allocation and risk management.
    #[serde(with = "string_to_float")]
    pub leaves_value: f64,

    /// The total quantity executed so far for this order.
    ///
    /// This tracks the cumulative filled portion of the order, critical for monitoring progress toward full execution (https://bybit-exchange.github.io/docs/v5/order/order-list).
    /// **Bot Implication**: Bots use `cum_exec_qty` to calculate position size, update risk models, and determine whether additional orders are needed to achieve the desired exposure.
    #[serde(with = "string_to_float")]
    pub cum_exec_qty: f64,

    /// The total monetary value of the executed portion, calculated as `cum_exec_qty` multiplied by the average execution price.
    ///
    /// This quantifies the capital deployed in the executed portion of the order, useful for performance tracking and cost analysis.
    /// **Bot Implication**: Bots use `cum_exec_value` to assess trade costs, including fees, and to evaluate the profitability of partially filled orders.
    #[serde(with = "string_to_float")]
    pub cum_exec_value: f64,

    /// The average price at which the executed portion of the order was filled.
    ///
    /// This is calculated as the total value of executed trades divided by `cum_exec_qty`. It provides a weighted average execution price, reflecting market conditions during filling (https://bybit-exchange.github.io/docs/v5/order/order-list).
    /// **Bot Implication**: Bots use `avg_price` to evaluate trade execution quality, compare it to `last_price_on_created`, and adjust strategies to minimize slippage in future orders.
    #[serde(with = "string_to_float")]
    pub avg_price: f64,

    /// Identifier for a block trade, if applicable.
    ///
    /// Block trades are large, negotiated trades executed outside the public order book, often used by institutional traders. This field is typically empty for retail bots (https://bybit-exchange.github.io/docs/v5/trade/block-trade).
    /// **Bot Implication**: For most retail bots, this field is unused, but large-scale bots may use it to track high-value trades separately for reporting or compliance.
    pub block_trade_id: String,

    /// Index indicating the position direction, e.g., 0 for one-way mode, 1 for buy-side hedge, 2 for sell-side hedge.
    ///
    /// Bybit’s unified margin account supports one-way or hedge modes for perpetual futures positions (https://bybit-exchange.github.io/docs/v5/account/position).
    /// **Bot Implication**: Bots must align `position_idx` with the account’s position mode to avoid position mismatches, which could lead to unintended liquidations.
    #[serde(rename = "positionIdx")]
    pub position_idx: u8,

    /// The cumulative trading fees incurred for this order.
    ///
    /// Fees in perpetual futures include maker/taker fees, which impact profitability. Bybit’s fee structure varies by VIP level and order type (https://bybit-exchange.github.io/docs/v5/account/fee).
    /// **Bot Implication**: Bots must track `cum_exec_fee` to optimize order types (e.g., maker vs. taker) and account for fees in profitability calculations, as high-frequency trading can accumulate significant costs.
    #[serde(with = "string_to_float")]
    pub cum_exec_fee: f64,

    /// Timestamp when the order was created, in milliseconds since Unix epoch.
    ///
    /// This records the exact time of order submission, critical for auditing and performance analysis (https://bybit-exchange.github.io/docs/v5/order/order-list).
    /// **Bot Implication**: Bots use `created_time` to correlate orders with market events, calculate latency, and ensure compliance with exchange timestamps.
    #[serde(with = "string_to_u64")]
    pub created_time: u64,

    /// Timestamp of the last update to the order, in milliseconds since Unix epoch.
    ///
    /// This tracks the most recent change to the order’s status, such as partial fills or cancellations (https://bybit-exchange.github.io/docs/v5/order/order-list).
    /// **Bot Implication**: Bots use `updated_time` to detect delays in execution or status changes, enabling timely adjustments to strategies or risk controls.
    #[serde(with = "string_to_u64")]
    pub updated_time: u64,

    /// Reason for order rejection, if applicable, e.g., "Insufficient margin".
    ///
    /// This field explains why an order failed to execute, helping diagnose issues like funding shortages or invalid parameters (https://bybit-exchange.github.io/docs/v5/order/order-list).
    /// **Bot Implication**: Bots must monitor `reject_reason` to handle errors programmatically, such as topping up margin or adjusting order parameters to prevent repeated failures.
    pub reject_reason: String,

    /// Type of stop order, e.g., "Stop", "TrailingStop", if applicable.
    ///
    /// Stop orders in perpetual futures are conditional orders triggered by price movements, used for risk management (https://bybit-exchange.github.io/docs/v5/order/stop-order).
    /// **Bot Implication**: Bots use `stop_order_type` to implement automated stop-loss or trailing stops, critical for limiting losses in leveraged positions.
    #[serde(rename = "stopOrderType")]
    pub stop_order_type: String,

    /// Take-profit/stop-loss mode, e.g., "Full", "Partial".
    ///
    /// This defines how take-profit and stop-loss orders are applied in Bybit’s unified margin account, either to the full position or a portion (https://bybit-exchange.github.io/docs/v5/order/tpsl-mode).
    /// **Bot Implication**: Bots must configure `tpsl_mode` to align with risk management strategies, ensuring partial or full position exits as intended.
    #[serde(rename = "tpslMode")]
    pub tpsl_mode: String,

    /// Price level that triggers a stop order.
    ///
    /// In perpetual futures, the trigger price determines when a stop-loss or other conditional order activates (https://bybit-exchange.github.io/docs/v5/order/stop-order).
    /// **Bot Implication**: Bots set `trigger_price` to automate risk controls, ensuring timely exits from losing positions to prevent margin calls or liquidations.
    #[serde(with = "string_to_float")]
    pub trigger_price: f64,

    /// Price level at which a take-profit order is triggered.
    ///
    /// Take-profit orders lock in gains by closing a position when the market reaches a favorable price (https://bybit-exchange.github.io/docs/v5/order/tpsl-order).
    /// **Bot Implication**: Bots use `take_profit` to secure profits automatically, balancing the trade-off between capturing gains and allowing room for further upside.
    #[serde(with = "string_to_float")]
    pub take_profit: f64,

    /// Price level at which a stop-loss order is triggered.
    ///
    /// Stop-loss orders limit losses by closing a position when the market moves against it (https://bybit-exchange.github.io/docs/v5/order/tpsl-order).
    /// **Bot Implication**: Bots set `stop_loss` to cap downside risk, a critical feature for leveraged perpetual futures where losses can exceed initial margin.
    #[serde(with = "string_to_float")]
    pub stop_loss: f64,

    /// Specifies the price type that triggers the take-profit order, e.g., "LastPrice", "IndexPrice".
    ///
    /// Bybit allows different price feeds for triggering orders, affecting how take-profit is calculated (https://bybit-exchange.github.io/docs/v5/order/tpsl-order).
    /// **Bot Implication**: Bots must select the appropriate `tp_trigger_by` to align with strategy logic, as different price types can lead to varying trigger points.
    #[serde(rename = "tpTriggerBy")]
    pub tp_trigger_by: String,

    /// Specifies the price type that triggers the stop-loss order, e.g., "LastPrice", "IndexPrice".
    ///
    /// Similar to `tp_trigger_by`, this determines the reference price for stop-loss activation (https://bybit-exchange.github.io/docs/v5/order/tpsl-order).
    /// **Bot Implication**: Bots need to choose `sl_trigger_by` carefully, as price feed discrepancies can cause premature or delayed stop-loss triggers.
    #[serde(rename = "slTriggerBy")]
    pub sl_trigger_by: String,

    /// Limit price for the take-profit order, if applicable.
    ///
    /// This specifies the exact price at which the take-profit order executes once triggered, allowing precise profit capture (https://bybit-exchange.github.io/docs/v5/order/tpsl-order).
    /// **Bot Implication**: Bots use `tp_limit_price` to minimize slippage on take-profit execution, ensuring profits are locked in at the desired level.
    #[serde(rename = "tpLimitPrice", with = "string_to_float")]
    pub tp_limit_price: f64,

    /// Limit price for the stop-loss order, if applicable.
    ///
    /// This sets the execution price for the stop-loss order once triggered, controlling the exit price in adverse market moves (https://bybit-exchange.github.io/docs/v5/order/tpsl-order).
    /// **Bot Implication**: Bots set `sl_limit_price` to balance loss minimization with the risk of slippage, especially in fast-moving markets.
    #[serde(rename = "slLimitPrice", with = "string_to_float")]
    pub sl_limit_price: f64,

    /// Direction of the price movement that triggers the order, e.g., 1 for rising price, 2 for falling price.
    ///
    /// This defines whether the stop or take-profit order activates on an upward or downward price move (https://bybit-exchange.github.io/docs/v5/order/stop-order).
    /// **Bot Implication**: Bots must set `trigger_direction` correctly to align with the intended risk or profit-taking logic, as incorrect settings can lead to unintended executions.
    pub trigger_direction: u8,

    /// The price type that triggers the conditional order, e.g., "LastPrice", "MarkPrice".
    ///
    /// Similar to `tp_trigger_by` and `sl_trigger_by`, this specifies the reference price for conditional orders (https://bybit-exchange.github.io/docs/v5/order/stop-order).
    /// **Bot Implication**: Bots need to ensure `trigger_by` matches the strategy’s price feed to avoid discrepancies in trigger timing.
    pub trigger_by: String,

    /// Indicates if the order closes the position when triggered.
    ///
    /// When `true`, the order ensures the position is fully closed upon trigger, used in stop-loss or take-profit scenarios (https://bybit-exchange.github.io/docs/v5/order/tpsl-order).
    /// **Bot Implication**: Bots use `close_on_trigger` to enforce complete position exits, critical for risk management in leveraged perpetual futures.
    pub close_on_trigger: bool,

    /// The product category, e.g., "linear", "inverse".
    ///
    /// Bybit offers linear (USDT-margined) and inverse (coin-margined) perpetual futures, each with different margin and settlement mechanics (https://bybit-exchange.github.io/docs/v5/market/instrument).
    /// **Bot Implication**: Bots must align `category` with the account’s margin type to ensure correct position and margin calculations.
    pub category: String,

    /// Indicates how the order was placed, e.g., "Normal", "Adl" (Auto-Deleveraging).
    ///
    /// This field distinguishes standard orders from those triggered by Bybit’s risk management processes, like ADL (https://bybit-exchange.github.io/docs/v5/account/position#auto-deleveraging-adl).
    /// **Bot Implication**: Bots should monitor `place_type` to identify non-standard orders, which may indicate risk events like forced liquidations.
    pub place_type: String,

    /// Self-Match Prevention (SMP) type, if applicable.
    ///
    /// SMP prevents orders from the same account matching each other, reducing wash trading risks (https://bybit-exchange.github.io/docs/v5/order/smp).
    /// **Bot Implication**: Bots in multi-strategy setups should track `smp_type` to ensure compliance with Bybit’s trading rules and avoid penalties.
    pub smp_type: String,

    /// Group ID for SMP, if applicable.
    ///
    /// This identifies the SMP group to which the order belongs, used to enforce self-match prevention (https://bybit-exchange.github.io/docs/v5/order/smp).
    /// **Bot Implication**: Bots managing multiple sub-accounts or strategies use `smp_group` to coordinate orders and prevent internal matching.
    pub smp_group: u8,

    /// Order ID of the matching order in an SMP scenario, if applicable.
    ///
    /// This links the order to its counterpart in an SMP event, aiding in audit trails (https://bybit-exchange.github.io/docs/v5/order/smp).
    /// **Bot Implication**: Bots use `smp_order_id` for reconciliation and compliance reporting, ensuring transparency in SMP events.
    pub smp_order_id: String,

    /// The currency used for fees, e.g., "USDT".
    ///
    /// This specifies the currency in which trading fees are charged, impacting cost calculations (https://bybit-exchange.github.io/docs/v5/account/fee).
    /// **Bot Implication**: Bots must account for `fee_currency` in profitability models, as fees in volatile currencies like USDT can affect net returns.
    pub fee_currency: String,
}
