use crate::prelude::*;

/// Represents a single order with detailed attributes.
///
/// This struct provides comprehensive details about an order, including its execution
/// status, prices, quantities, and fees. In perpetual futures trading on Bybit, this
/// struct is used to track the lifecycle of an order, from placement to execution or
/// cancellation. Perpetual futures are financial derivatives that allow leveraged
/// trading without an expiration date, balanced by funding rates. Bots use this struct
/// to monitor order progress, calculate profitability, manage positions, and handle
/// risk. Fields like `avg_price`, `cum_exec_fee`, and `take_profit` are critical for
/// performance analysis and risk management in volatile markets.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    /// The unique identifier of the order provided by Bybit.
    ///
    /// This is a system-generated ID assigned when the order is created. In perpetual
    /// futures, `order_id` is essential for tracking orders across API requests (e.g.,
    ///
    /// amending, canceling, or querying status). Bots must store and reference this ID
    /// to manage orders accurately, especially in high-frequency trading where multiple
    /// orders are active. Losing track of `order_id` can lead to orphaned orders or
    /// mismanaged positions, impacting strategy execution.
    pub order_id: String,

    /// The user-defined identifier of the order.
    ///
    /// A custom ID set by the trader to track the order. In perpetual futures, where
    /// bots may manage numerous orders, `order_link_id` provides a way to correlate
    /// orders with specific strategies or logic. Bots should use unique, descriptive
    /// IDs to avoid confusion and ensure robust order tracking. If not set, it’s an
    /// empty string, but bots should always assign a value for traceability.
    pub order_link_id: String,

    /// The identifier for block trades (optional).
    ///
    /// Represents the ID of a block trade, which is a large order executed off the
    /// public order book, typically for institutional traders. In perpetual futures,
    ///
    /// this field is usually `None` for retail bots, as block trades are less common.
    /// For bots handling large orders, this field helps track such trades, ensuring
    /// compliance with Bybit’s reporting requirements. Bots should log this for
    /// auditing if applicable.
    #[serde(
        deserialize_with = "empty_string_as_none",
        skip_serializing_if = "is_empty_or_none"
    )]
    pub block_trade_id: Option<String>,

    /// The trading pair symbol, e.g., "BTCUSDT".
    ///
    /// Identifies the asset pair of the order (e.g., BTCUSDT for Bitcoin against USDT).
    /// In perpetual futures, symbols are typically USDT-margined (Linear category).
    /// Bots must validate that the symbol is supported by Bybit and align order
    /// parameters (e.g., price, quantity) with the symbol’s contract specifications
    /// (e.g., tick size, minimum quantity). This field is critical for correlating
    /// orders with market data and ensuring strategy consistency.
    pub symbol: String,

    /// The price of the order.
    ///
    /// For limit orders, this is the specified price at which the order is placed; for
    /// market orders, it’s the executed price. In perpetual futures, price must adhere
    /// to the symbol’s tick size (e.g., $0.5 for BTCUSDT). Bots should use this field
    /// to verify execution against market conditions and calculate profitability. For
    /// limit orders, setting prices too far from the market may prevent execution,
    ///
    /// while market orders risk slippage in volatile markets.
    #[serde(with = "string_to_float")]
    pub price: f64,

    /// The quantity of the order.
    ///
    /// Represents the size of the order in the base asset (e.g., BTC for BTCUSDT).
    /// In perpetual futures, quantity must comply with Bybit’s minimum and maximum
    /// limits, which vary by symbol. Bots should validate this against account balance,
    ///
    /// margin requirements, and symbol constraints to avoid rejections. Over-sizing
    /// quantities can increase liquidation risk, especially with leverage, so bots
    /// must carefully manage this field.
    #[serde(with = "string_to_float")]
    pub qty: f64,

    /// The side of the order (Buy or Sell).
    ///
    /// Indicates whether the order is to buy (long) or sell (short). In perpetual
    /// futures, Buy orders open long positions or close short ones, while Sell orders
    /// open short positions or close long ones. Bots must align this with their market
    /// outlook and position management strategy. Incorrect sides can lead to unintended
    /// exposure, so bots should include validation logic to prevent errors.
    pub side: Side,

    /// Indicates if leverage was used (empty string for false).
    ///
    /// A non-empty string (e.g., "1") indicates the order used leverage, common in
    /// perpetual futures to amplify position sizes. Leverage increases potential
    /// returns but also liquidation risk if prices move adversely. Bots should check
    /// this field to confirm margin usage and monitor account health, ensuring
    /// sufficient collateral to avoid forced liquidations. For non-leveraged orders,
    ///
    /// this is an empty string.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub is_leverage: String,

    /// The position index (0 for one-way, 1 or 2 for hedge mode).
    ///
    /// Specifies the position mode: 0 for one-way mode (single position per symbol),
    ///
    /// 1 for long, or 2 for short in hedge mode (allowing simultaneous long and short
    /// positions). In perpetual futures, bots must ensure this matches the account’s
    /// position mode to avoid order rejections. Mismanaging `position_idx` can lead to
    /// unintended position netting or increased margin requirements, disrupting
    /// strategy execution.
    pub position_idx: i32,

    /// The current status of the order (e.g., "New", "Filled", "Cancelled").
    ///
    /// Indicates the order’s lifecycle stage, such as "New" (pending), "PartiallyFilled",
    ///
    /// "Filled", or "Cancelled". In perpetual futures, bots should monitor this to track
    /// execution progress and handle edge cases (e.g., partial fills or cancellations).
    /// For example, a "PartiallyFilled" status requires bots to adjust position sizes
    /// and risk calculations. This field is critical for state management and strategy
    /// adaptation.
    pub order_status: String,

    /// The reason for cancellation (if applicable).
    ///
    /// Specifies why an order was canceled, such as "CancelByUser" or "Timeout". In
    /// perpetual futures, understanding cancellation reasons helps bots diagnose issues
    /// (e.g., market conditions preventing execution) and refine strategies. Bots
    /// should log this field for auditing and use it to trigger fallback logic, such
    /// as resubmitting orders with adjusted parameters.
    pub cancel_type: String,

    /// The reason for order rejection (if applicable).
    ///
    /// Explains why an order was rejected by Bybit, such as "InsufficientBalance" or
    /// "InvalidPrice". In perpetual futures, rejections can occur due to margin
    /// constraints, invalid parameters, or market limits. Bots must handle this field
    /// to diagnose and resolve issues, implementing error-handling logic to adjust
    /// order parameters (e.g., reducing quantity) or retrying after funding the account.
    pub reject_reason: String,

    /// The average execution price (optional).
    ///
    /// The weighted average price of executed portions for partially or fully filled
    /// orders. In perpetual futures, bots use this to calculate realized profitability
    /// and compare against market prices. For example, a lower `avg_price` than
    /// expected for a Buy order indicates slippage. If `None`, the order has not been
    /// filled. Bots should monitor this to assess execution quality and optimize order
    /// types (e.g., preferring limit orders to reduce slippage).
    #[serde(with = "string_to_float_optional")]
    pub avg_price: Option<f64>,

    /// The remaining quantity to be executed.
    ///
    /// Indicates the unfilled portion of the order’s quantity. In perpetual futures,
    ///
    /// bots should monitor this to determine if an order is partially filled and adjust
    /// position sizing or risk calculations. A non-zero `leaves_qty` for a limit order
    /// may indicate the price is too far from the market, prompting bots to amend the
    /// order or cancel and resubmit at a better price.
    #[serde(with = "string_to_float")]
    pub leaves_qty: f64,

    /// The remaining value to be executed.
    ///
    /// The monetary value of the unfilled portion (typically `leaves_qty` multiplied
    /// by the order price). In perpetual futures, bots can use this to assess exposure
    /// and margin requirements for pending orders. This field helps quantify the
    /// potential impact of unfilled orders on account health, especially in leveraged
    /// trading scenarios.
    #[serde(with = "string_to_float")]
    pub leaves_value: f64,

    /// The cumulative executed quantity.
    ///
    /// The total quantity filled so far. In perpetual futures, bots should use this
    /// to track order progress and update position sizes. For example, if
    /// `cum_exec_qty` is less than `qty`, the order is partially filled, requiring
    /// bots to manage the open position and any remaining risk. This field is critical
    /// for accurate position tracking and strategy execution.
    #[serde(with = "string_to_float")]
    pub cum_exec_qty: f64,

    /// The cumulative executed value.
    ///
    /// The total monetary value of filled portions (typically `cum_exec_qty` multiplied
    /// by the execution prices). In perpetual futures, bots use this to calculate the
    /// cost basis of a position and assess profitability. This field helps bots
    /// determine the financial impact of executed trades, especially for leveraged
    /// positions where margin requirements are dynamic.
    #[serde(with = "string_to_float")]
    pub cum_exec_value: f64,

    /// The cumulative execution fees.
    ///
    /// The total fees incurred for executed portions of the order. In perpetual
    /// futures, fees vary based on order type: maker orders (e.g., PostOnly limit
    /// orders) typically have lower or negative fees, while taker orders (e.g., market
    /// orders) have higher fees. Bots should monitor this closely, as fees impact
    /// profitability, especially in high-frequency trading. Optimizing for maker orders
    /// can reduce costs, and bots should log this field for performance analysis.
    #[serde(with = "string_to_float")]
    pub cum_exec_fee: f64,

    /// The time-in-force policy of the order.
    ///
    /// Specifies how long the order was active, such as "GTC" (Good Till Canceled),
    ///
    /// "IOC" (Immediate or Cancel), "FOK" (Fill or Kill), or "PostOnly". In perpetual
    /// futures, TIF affects execution strategy and fees. For example, PostOnly orders
    /// optimize for maker fees but may not execute in fast markets. Bots should verify
    /// this field to ensure the order’s behavior aligns with the strategy and adjust
    /// TIF for future orders if needed.
    pub time_in_force: String,

    /// The type of order (Limit or Market).
    ///
    /// Indicates whether the order was a limit order (executed at a specified price)
    /// or a market order (executed at the best available price). In perpetual futures,
    ///
    /// limit orders offer price control but risk non-execution, while market orders
    /// ensure execution but may incur slippage and higher fees. Bots should use this
    /// field to confirm order type and assess execution performance, optimizing for
    /// cost or speed based on strategy goals.
    pub order_type: OrderType,

    /// The type of stop order (optional, e.g., "StopLoss").
    ///
    /// Specifies if the order was a conditional stop order, such as "StopLoss" or
    /// "TakeProfit". In perpetual futures, stop orders are critical for risk
    /// management, automatically closing positions at predefined levels. If `None`,
    ///
    /// the order is not a stop order. Bots should use this field to identify risk
    /// management orders and ensure they align with the strategy’s exit conditions.
    #[serde(
        deserialize_with = "empty_string_as_none",
        skip_serializing_if = "is_empty_or_none"
    )]
    pub stop_order_type: Option<String>,

    /// The implied volatility for options orders (optional).
    ///
    /// Represents the expected volatility for options orders, not applicable to
    /// perpetual futures. In Bybit’s API, this field is relevant only for options
    /// trading, where volatility affects pricing. Bots trading perpetual futures can
    /// ignore this field, but those expanding to options must source accurate
    /// volatility data to interpret this correctly.
    #[serde(
        deserialize_with = "empty_string_as_none",
        skip_serializing_if = "is_empty_or_none"
    )]
    pub order_iv: Option<String>,

    /// The trigger price for conditional orders (optional).
    ///
    /// The price at which a conditional order (e.g., stop-loss or take-profit)
    /// activates. In perpetual futures, trigger prices are essential for risk
    /// management, defining exit points for profitable or losing positions. If `None`,
    ///
    /// the order is not conditional. Bots should verify this field to ensure risk
    /// management settings are correct and adjust trigger prices based on market
    /// volatility and strategy goals.
    #[serde(with = "string_to_float_optional")]
    pub trigger_price: Option<f64>,

    /// The take-profit price (optional).
    ///
    /// The price at which the position is closed to lock in profits. In perpetual
    /// futures, take-profit (TP) is critical for securing gains in volatile markets.
    /// If `None`, no TP is set. Bots should use this field to confirm TP settings and
    /// calculate realized profits, ensuring the price is realistic given market
    /// conditions and symbol tick size constraints.
    #[serde(with = "string_to_float_optional")]
    pub take_profit: Option<f64>,

    /// The stop-loss price (optional).
    ///
    /// The price at which the position is closed to limit losses. In perpetual
    /// futures, stop-loss (SL) is essential to prevent significant drawdowns,
    ///
    /// especially with leverage. If `None`, no SL is set. Bots should use this field
    /// to confirm SL settings and manage risk, setting SL prices based on risk
    /// tolerance and volatility to avoid premature exits or excessive losses.
    #[serde(with = "string_to_float_optional")]
    pub stop_loss: Option<f64>,

    /// The price type for triggering take-profit.
    ///
    /// Specifies the price metric used for TP triggering, such as "LastPrice",
    ///
    /// "IndexPrice", or "MarkPrice". In perpetual futures, choosing a reliable metric
    /// like "MarkPrice" (less susceptible to manipulation) ensures consistent
    /// execution. Bots should verify this field to ensure TP triggers as intended,

    /// especially in volatile markets where price discrepancies can occur.
    pub tp_trigger_by: String,

    /// The price type for triggering stop-loss.
    ///
    /// Specifies the price metric used for SL triggering, similar to `tp_trigger_by`.
    /// Bots should select a stable metric (e.g., "MarkPrice") to protect against
    /// manipulation-driven triggers in perpetual futures. This field ensures SL
    /// activates reliably, safeguarding positions from adverse price movements.
    pub sl_trigger_by: String,

    /// The direction of the trigger price (0, 1, or 2).
    ///
    /// Indicates whether the trigger activates when the price rises (1) or falls (2);
    /// 0 indicates no direction (non-conditional orders). In perpetual futures, this
    /// field is critical for conditional orders like stop-loss or take-profit. For
    /// example, a stop-loss for a long position typically has `trigger_direction` set
    /// to 2 (falling price). Bots must validate this to ensure risk management logic
    /// aligns with market expectations.
    pub trigger_direction: i32,

    /// The price type for triggering conditional orders (optional).
    ///
    /// Specifies the price metric for triggering conditional orders (e.g.,
    ///
    /// "LastPrice", "MarkPrice"). If `None`, the order is not conditional. In
    /// perpetual futures, bots should choose a reliable metric to ensure consistent
    /// execution, especially for high-frequency strategies where timing is critical.
    #[serde(
        deserialize_with = "empty_string_as_none",
        skip_serializing_if = "is_empty_or_none"
    )]
    pub trigger_by: Option<String>,

    /// The market price when the order was created (optional).
    ///
    /// The reference market price at the time of order placement, typically the last
    /// traded price. In perpetual futures, bots can use this to compare against
    /// execution prices (`avg_price`) and assess slippage or market movement since
    /// order creation. If `None`, no reference price was recorded. This field helps
    /// bots evaluate order timing and market conditions.
    #[serde(with = "string_to_float_optional")]
    pub last_price_on_created: Option<f64>,

    /// Indicates if the order reduces an existing position.
    ///
    /// When `true`, the order only closes an existing position, preventing unintended
    /// increases in exposure. In perpetual futures, `reduce_only` is critical for
    /// position management, ensuring bots exit positions without opening opposing
    /// ones. Bots should set and verify this field when closing positions to avoid
    /// margin issues or unexpected leverage.
    pub reduce_only: bool,

    /// Indicates if the order closes the position when triggered.
    ///
    /// When `true`, the order closes the position upon triggering, typically used for
    /// stop-loss or take-profit orders. In perpetual futures, this ensures automatic
    /// position closure at predefined levels, aiding risk management. Bots should
    /// verify this field for risk-limiting orders to prevent unintended position
    /// retention.
    pub close_on_trigger: bool,

    /// The type of self-match prevention.
    ///
    /// Specifies the self-match prevention (SMP) mechanism to prevent a trader’s own
    /// orders from matching against each other, which could manipulate market
    /// perception. In perpetual futures, this is relevant for large or institutional
    /// bots to comply with exchange rules. Retail bots typically see an empty string
    /// unless SMP is configured. Bots should log this for compliance auditing.
    pub smp_type: String,

    /// The self-match prevention group identifier.
    ///
    /// Groups orders for SMP purposes, assigning them to a specific SMP group. In
    /// perpetual futures, this is relevant for advanced bots managing large order
    /// books. Retail bots can usually ignore this unless participating in SMP
    /// mechanisms. Bots should use this to ensure compliance with Bybit’s trading
    /// rules.
    pub smp_group: i32,

    /// The identifier of the matched SMP order (optional).
    ///
    /// Indicates if an order was affected by SMP, identifying the matched order. In
    /// perpetual futures, this is typically `None` for retail bots but relevant for
    /// institutional traders. Bots should log this for auditing and compliance,
    ///
    /// ensuring SMP rules are followed to avoid penalties.
    #[serde(
        deserialize_with = "empty_string_as_none",
        skip_serializing_if = "is_empty_or_none"
    )]
    pub smp_order_id: Option<String>,

    /// The take-profit/stop-loss mode (e.g., "Full", "Partial").
    ///
    /// Specifies whether TP/SL applies to the entire position ("Full") or a portion
    /// ("Partial"). In perpetual futures, "Full" ensures complete position closure at
    /// TP/SL levels, simplifying risk management, while "Partial" suits complex exit
    /// strategies like scaling out. Bots should verify this field to confirm exit
    /// strategy alignment and adjust TP/SL settings accordingly.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub tpsl_mode: String,

    /// The limit price for take-profit orders (optional).
    ///
    /// The exact price for a limit-based take-profit order. In perpetual futures,
    ///
    /// this allows precise profit-taking but risks non-execution if the market doesn’t
    /// reach the price. If `None`, the TP is not limit-based. Bots should set this
    /// based on market depth and volatility to balance execution likelihood and
    /// profitability, ensuring compliance with tick size constraints.
    #[serde(with = "string_to_float_optional")]
    pub tp_limit_price: Option<f64>,

    /// The limit price for stop-loss orders (optional).
    ///
    /// The exact price for a limit-based stop-loss order. Similar to `tp_limit_price`,
    ///
    /// bots must balance precision with execution risk, ensuring the price is
    /// achievable in volatile perpetual futures markets to protect against losses. If
    /// `None`, the SL is not limit-based. Bots should validate this against market
    /// conditions to ensure effective risk management.
    #[serde(with = "string_to_float_optional")]
    pub sl_limit_price: Option<f64>,

    /// The placement type of the order (optional).
    ///
    /// Indicates how the order was placed, such as via API, web, or mobile app. In
    /// perpetual futures, this is typically `None` for API-based orders but may be
    /// used for auditing or tracking order sources. Bots should log this for
    /// traceability, especially in multi-platform trading environments.
    #[serde(
        deserialize_with = "empty_string_as_none",
        skip_serializing_if = "is_empty_or_none"
    )]
    pub place_type: Option<String>,

    /// The timestamp when the order was created (in milliseconds).
    ///
    /// The time the order was placed, in Unix epoch milliseconds. In perpetual
    /// futures, bots use this to track order age and correlate with market events.
    /// For example, comparing `created_time` with `updated_time` helps assess order
    /// execution latency. Bots should log this for auditing and performance analysis.
    #[serde(with = "string_to_u64")]
    pub created_time: u64,

    /// The timestamp when the order was last updated (in milliseconds).
    ///
    /// The time of the most recent update to the order (e.g., partial fill,
    ///
    /// cancellation), in Unix epoch milliseconds. In perpetual futures, bots use this
    /// to monitor order progress and detect changes in status or execution. Comparing
    /// `updated_time` with `created_time` helps evaluate market responsiveness and
    /// strategy timing. Bots should use this for real-time state management.
    #[serde(with = "string_to_u64")]
    pub updated_time: u64,
}
