use crate::prelude::*;

/// Parameters for setting trading stop conditions (take-profit/stop-loss).
///
/// Used to construct a request to the `/v5/position/trading-stop` endpoint to set take-profit and stop-loss conditions for a position. Bots use this to implement automated exit strategies and manage risk in perpetual futures trading. This struct supports partial take-profit/stop-loss modes, allowing precise control over position exits.
#[derive(Clone, Default)]
pub struct TradingStopRequest<'a> {
    /// The product category (e.g., Linear, Inverse).
    ///
    /// Specifies the instrument type, such as `Linear` for USDT-margined perpetual futures. Bots must set this correctly to target the desired contract type.
    pub category: Category,
    /// The trading pair symbol (e.g., "BTCUSDT").
    ///
    /// Identifies the perpetual futures contract for which trading stops are being set. Bots must specify a valid symbol to ensure the request targets the correct position.
    pub symbol: Cow<'a, str>,
    /// The take-profit price (optional).
    ///
    /// The price at which the position will close for a profit. Bots use this to lock in gains automatically, typically based on technical indicators or predefined targets.
    pub take_profit: Option<f64>,
    /// The stop-loss price (optional).
    ///
    /// The price at which the position will close to limit losses. Bots use this to manage downside risk, protecting capital during adverse market movements.
    pub stop_loss: Option<f64>,
    /// The trigger type for take-profit (e.g., "LastPrice", "MarkPrice") (optional).
    ///
    /// Specifies the price type used to trigger the take-profit order (e.g., `LastPrice` for the last traded price, `MarkPrice` for the mark price). Bots should choose a trigger type that aligns with their strategy to balance responsiveness and stability.
    pub tp_trigger_by: Option<Cow<'a, str>>,
    /// The trigger type for stop-loss (e.g., "LastPrice", "MarkPrice") (optional).
    ///
    /// Specifies the price type used to trigger the stop-loss order. Bots should select a trigger type that minimizes slippage while ensuring timely execution in volatile markets.
    pub sl_trigger_by: Option<Cow<'a, str>>,
    /// The take-profit/stop-loss mode (e.g., "Full", "Partial") (optional).
    ///
    /// Specifies whether the trading stops apply to the entire position (`Full`) or a portion (`Partial`). Bots use this to implement granular exit strategies, such as scaling out of positions.
    pub tpsl_mode: Option<Cow<'a, str>>,
    /// The order type for take-profit (optional).
    ///
    /// Specifies whether the take-profit order is a `Limit` or `Market` order. Bots can use `Limit` orders to target specific exit prices, reducing slippage in less volatile markets.
    pub tp_order_type: Option<OrderType>,
    /// The order type for stop-loss (optional).
    ///
    /// Specifies whether the stop-loss order is a `Limit` or `Market` order. Bots typically use `Market` orders for stop-losses to ensure execution during rapid price declines.
    pub sl_order_type: Option<OrderType>,
    /// The size of the take-profit order (optional).
    ///
    /// The quantity of the position to close when the take-profit is triggered, used in `Partial` mode. Bots use this to scale out of positions incrementally, optimizing profit capture.
    pub tp_size: Option<f64>,
    /// The size of the stop-loss order (optional).
    ///
    /// The quantity of the position to close when the stop-loss is triggered, used in `Partial` mode. Bots use this to limit losses on specific portions of a position.
    pub sl_size: Option<f64>,
    /// The limit price for the take-profit order (optional).
    ///
    /// The specific price for a `Limit` take-profit order. Bots use this to ensure the take-profit order executes at a favorable price, avoiding slippage in stable markets.
    pub tp_limit_price: Option<f64>,
    /// The limit price for the stop-loss order (optional).
    ///
    /// The specific price for a `Limit` stop-loss order. Bots use this cautiously, as limit orders may not execute in fast-moving markets, increasing risk exposure.
    pub sl_limit_price: Option<f64>,
    /// The position index (e.g., 0 for one-way mode, 1 or 2 for hedge mode).
    ///
    /// Specifies the position type in Bybit’s position management system. Bots must set this correctly to target the intended position (e.g., long or short in hedge mode).
    pub position_idx: i32,
}

impl<'a> TradingStopRequest<'a> {
    /// Constructs a new TradingStop request with specified parameters.
    ///
    /// Allows full customization of the trading stop request. Bots should use this to define the exact symbol, category, and trading stop parameters to align with their risk management strategy.
    pub fn new(
        category: Category,
        symbol: &'a str,
        take_profit: Option<f64>,
        stop_loss: Option<f64>,
        tp_trigger_by: Option<&'a str>,
        sl_trigger_by: Option<&'a str>,
        tpsl_mode: Option<&'a str>,
        tp_order_type: Option<OrderType>,
        sl_order_type: Option<OrderType>,
        tp_size: Option<f64>,
        sl_size: Option<f64>,
        tp_limit_price: Option<f64>,
        sl_limit_price: Option<f64>,
        position_idx: i32,
    ) -> Self {
        Self {
            category,
            symbol: Cow::Borrowed(symbol),
            take_profit,
            stop_loss,
            tp_trigger_by: tp_trigger_by.map(Cow::Borrowed),
            sl_trigger_by: sl_trigger_by.map(Cow::Borrowed),
            tpsl_mode: tpsl_mode.map(Cow::Borrowed),
            tp_order_type,
            sl_order_type,
            tp_size,
            sl_size,
            tp_limit_price,
            sl_limit_price,
            position_idx,
        }
    }
    /// Creates a default TradingStop request.
    ///
    /// Returns a request with `category` set to `Linear`, `symbol` set to `"BTCUSDT"`, `position_idx` set to `1`, and all other fields unset. Suitable for testing but should be customized for production to match specific trading needs.
    pub fn default() -> TradingStopRequest<'a> {
        TradingStopRequest::new(
            Category::Linear,
            "BTCUSDT",
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            1,
        )
    }
}
