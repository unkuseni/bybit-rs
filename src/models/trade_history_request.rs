use crate::prelude::*;

/// Parameters for requesting trade history data.
///
/// Used to construct a request to the `/v5/execution/list` endpoint to retrieve historical trade executions. Bots use this to analyze past trades, calculate performance metrics, and refine trading strategies for perpetual futures.
#[derive(Clone, Default)]
pub struct TradeHistoryRequest<'a> {
    /// The product category (e.g., Linear, Inverse).
    ///
    /// Specifies the instrument type. Bots must set this to fetch trade history for the correct contract type (e.g., `Linear` for USDT-margined perpetuals).
    pub category: Category,

    /// The trading pair symbol (e.g., "BTCUSDT").
    ///
    /// Optionally filters trades by symbol. If unset, trades for all symbols in the category are returned. Bots should specify this for targeted analysis.
    pub symbol: Option<Cow<'a, str>>,

    /// The order ID.
    ///
    /// Optionally filters trades by a specific order ID. Useful for bots tracking executions for a particular order.
    pub order_id: Option<Cow<'a, str>>,

    /// The user-defined order link ID.
    ///
    /// Optionally filters trades by a custom order identifier. Bots can use this to correlate trades with specific strategies.
    pub order_link_id: Option<Cow<'a, str>>,

    /// The base coin (e.g., "BTC").
    ///
    /// Optionally filters trades by the base asset. Useful for bots analyzing trades across multiple pairs of the same asset.
    pub base_coin: Option<Cow<'a, str>>,

    /// The start time for the trade history (Unix timestamp in milliseconds).
    ///
    /// Defines the beginning of the time range. Bots should set this for historical trade analysis, such as performance over a specific period.
    pub start_time: Option<u64>,

    /// The end time for the trade history (Unix timestamp in milliseconds).
    ///
    /// Defines the end of the time range. Bots should set this to limit data to a specific period, optimizing performance.
    pub end_time: Option<u64>,

    /// The execution type (e.g., "Trade", "Funding").
    ///
    /// Optionally filters trades by execution type. Bots can use this to focus on specific trade events, such as excluding funding fee executions.
    pub exec_type: Option<Cow<'a, str>>,

    /// The maximum number of records to return (1-50, default: 50).
    ///
    /// Controls the number of trade records returned. Bots should set a reasonable limit to balance data completeness with performance.
    pub limit: Option<u64>,
}
