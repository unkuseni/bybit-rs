use crate::prelude::*;

/// Represents a single futures instrument (e.g., a perpetual futures contract).
///
/// Contains detailed information about a futures contract, such as leverage, funding rates, and trading constraints. For perpetual futures, this is critical for configuring trading bots to comply with Bybit’s rules and manage risk.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FuturesInstrument {
    /// The trading pair symbol (e.g., "BTCUSDT").
    ///
    /// Identifies the specific perpetual futures contract. Bots must use this to match the instrument to their trading strategy.
    pub symbol: String,

    /// The type of futures contract (e.g., "perpetual").
    ///
    /// Indicates whether the contract is a perpetual futures contract or another type (e.g., quarterly futures). For perpetuals, this is typically `"perpetual"`. Bots should verify this to ensure they’re trading the correct instrument.
    pub contract_type: String,

    /// The trading status of the instrument (e.g., "Trading", "Settling").
    ///
    /// Indicates whether the contract is actively trading. Bots should only trade instruments with `"Trading"` status to avoid errors or unsupported operations.
    pub status: String,

    /// The base coin of the contract (e.g., "BTC").
    ///
    /// The underlying asset of the contract (e.g., BTC in `BTCUSDT`). Useful for bots filtering instruments by asset type.
    pub base_coin: String,

    /// The quote coin of the contract (e.g., "USDT").
    ///
    /// The currency used to quote the contract price (e.g., USDT in `BTCUSDT`). For linear perpetuals, this is typically USDT; for inverse, it’s the base coin (e.g., USD for `BTCUSD`). Bots should verify this for correct pricing calculations.
    pub quote_coin: String,

    /// The launch time of the contract (Unix timestamp in milliseconds).
    ///
    /// Indicates when the contract was listed on Bybit. Less critical for trading but useful for bots analyzing historical performance or contract maturity.
    #[serde(with = "string_to_u64")]
    pub launch_time: u64,

    /// The delivery time for non-perpetual futures (empty for perpetuals).
    ///
    /// For perpetual futures, this is an empty string since they have no expiry. Bots can ignore this for perpetuals but should check it for quarterly futures to understand settlement dates.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub delivery_time: String,

    /// The delivery fee rate for non-perpetual futures.
    ///
    /// For perpetual futures, this is typically empty or irrelevant since there’s no delivery. Bots can ignore this unless trading quarterly futures.
    pub delivery_fee_rate: String,

    /// The price scale (number of decimal places for price).
    ///
    /// Defines the precision for price quotes (e.g., `"2"` for two decimal places). Bots must use this to format order prices correctly to avoid rejection due to invalid precision.
    pub price_scale: String,

    /// The leverage constraints for the contract.
    ///
    /// Specifies the minimum, maximum, and step size for leverage. Bots must adhere to these constraints when setting leverage to avoid API errors and manage risk in perpetual futures.
    pub leverage_filter: LeverageFilter,

    /// The price constraints for the contract.
    ///
    /// Defines the minimum, maximum, and tick size for prices. Bots must use this to ensure order prices are within valid ranges and increments.
    pub price_filter: PriceFilter,

    /// The lot size constraints for the contract.
    ///
    /// Specifies the minimum, maximum, and step size for order quantities. Bots must comply with these to place valid orders and manage position sizes in perpetual futures.
    pub lot_size_filter: LotSizeFilter,

    /// Whether the contract supports unified margin trading.
    ///
    /// Indicates if the contract can be traded under Bybit’s unified margin account, which allows cross-margining across assets. Bots should check this to understand margin requirements and risk management options.
    pub unified_margin_trade: bool,

    /// The funding interval in minutes (e.g., 480 for 8 hours).
    ///
    /// Specifies how often funding rates are settled for perpetual futures (typically every 8 hours). Bots must monitor this to calculate funding costs, as frequent settlements can impact profitability for long-term positions.
    pub funding_interval: u64,

    /// The settlement coin for the contract (e.g., "USDT").
    ///
    /// The currency used for margin and settlement. For linear perpetuals, this is typically USDT; for inverse, it’s the base coin. Bots should use this to manage account balances and margin requirements.
    pub settle_coin: String,

    /// Indicates if copy trading is supported.
    ///
    /// Specifies whether the contract supports Bybit’s copy trading feature. Less relevant for bots unless integrating with social trading features.
    pub copy_trading: String,

    /// The upper bound for the funding rate.
    ///
    /// The maximum funding rate that can be applied during a funding interval. Bots should use this to estimate worst-case funding costs for long or short positions in perpetual futures.
    pub upper_funding_rate: String,

    /// The lower bound for the funding rate.
    ///
    /// The minimum funding rate (can be negative). Bots should use this to estimate funding costs or benefits, especially for short positions when rates are negative.
    pub lower_funding_rate: String,
}
