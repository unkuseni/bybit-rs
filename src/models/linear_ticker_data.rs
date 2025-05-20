use crate::prelude::*;

/// Structure for linear perpetual futures ticker data.
///
/// Contains ticker metrics specific to linear perpetual futures, such as funding rates and open interest. Bots use this for real-time market analysis and risk management in USDT-margined contracts.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LinearTickerData {
    /// The trading pair symbol (e.g., "BTCUSDT").
    ///
    /// Identifies the perpetual futures contract for the ticker data. Bots use this to verify the correct market.
    pub symbol: String,

    /// The tick direction of the last price change.
    ///
    /// Indicates whether the last price change was an uptick or downtick (e.g., "PlusTick"). Bots use this to analyze short-term price momentum.
    #[serde(rename = "tickDirection")]
    pub tick_direction: String,

    /// The 24-hour price change percentage.
    ///
    /// The percentage change in price over the last 24 hours. Bots use this to assess market trends and volatility.
    #[serde(rename = "price24hPcnt", with = "string_to_float")]
    pub price_24h_pcnt: f64,

    /// The last traded price.
    ///
    /// The most recent price at which the contract was traded. Bots use this for real-time price tracking and technical analysis.
    #[serde(with = "string_to_float")]
    pub last_price: f64,

    /// The price 24 hours ago.
    ///
    /// The price of the contract 24 hours prior. Bots use this to calculate price changes and validate `price_24h_pcnt`.
    #[serde(rename = "prevPrice24h", with = "string_to_float")]
    pub prev_price_24h: f64,

    /// The highest price in the last 24 hours.
    ///
    /// The peak price reached in the last 24 hours. Bots use this to identify resistance levels and assess volatility.
    #[serde(rename = "highPrice24h", with = "string_to_float")]
    pub high_price_24h: f64,

    /// The lowest price in the last 24 hours.
    ///
    /// The lowest price reached in the last 24 hours. Bots use this to identify support levels and assess volatility.
    #[serde(rename = "lowPrice24h", with = "string_to_float")]
    pub low_price_24h: f64,

    /// The price 1 hour ago.
    ///
    /// The price of the contract 1 hour prior. Bots use this to calculate short-term price changes and momentum.
    #[serde(rename = "prevPrice1h", with = "string_to_float")]
    pub prev_price_1h: f64,

    /// The current mark price.
    ///
    /// The mark price used for P&L calculations in perpetual futures. Bots use this to calculate unrealized P&L and assess position health.
    #[serde(with = "string_to_float")]
    pub mark_price: f64,

    /// The current index price.
    ///
    /// The index price, based on external spot markets, used for reference in perpetual futures. Bots use this to compare with mark price for funding rate calculations.
    #[serde(with = "string_to_float")]
    pub index_price: f64,

    /// The open interest in contracts.
    ///
    /// The total number of open contracts in the market. Bots use this to gauge market participation and liquidity.
    #[serde(with = "string_to_float")]
    pub open_interest: f64,

    /// The open interest value in settlement currency.
    ///
    /// The monetary value of open interest (`open_interest` * `mark_price`). Bots use this to assess market exposure and leverage levels.
    #[serde(with = "string_to_float")]
    pub open_interest_value: f64,

    /// The 24-hour trading turnover.
    ///
    /// The total trading value in the last 24 hours, in settlement currency. Bots use this to assess market activity and liquidity.
    #[serde(rename = "turnover24h", with = "string_to_float")]
    pub turnover_24h: f64,

    /// The 24-hour trading volume.
    ///
    /// The total quantity of contracts traded in the last 24 hours. Bots use this to analyze market activity and trading intensity.
    #[serde(rename = "volume24h", with = "string_to_float")]
    pub volume_24h: f64,

    /// The timestamp of the next funding event in milliseconds.
    ///
    /// Indicates when the next funding rate payment will occur. Bots use this to schedule funding fee calculations and position adjustments.
    #[serde(with = "string_to_u64")]
    pub next_funding_time: u64,

    /// The current funding rate.
    ///
    /// The funding rate applied to positions, as a decimal (e.g., 0.0001 for 0.01%). Bots use this to calculate funding costs or profits for long/short positions.
    #[serde(with = "string_to_float")]
    pub funding_rate: f64,

    /// The best bid price.
    ///
    /// The highest price at which someone is willing to buy. Bots use this to assess buy-side liquidity and calculate spreads.
    #[serde(rename = "bid1Price", with = "string_to_float")]
    pub bid_price: f64,

    /// The best bid size.
    ///
    /// The quantity available at the best bid price. Bots use this to evaluate buy-side liquidity and potential slippage.
    #[serde(rename = "bid1Size", with = "string_to_float")]
    pub bid_size: f64,

    /// The best ask price.
    ///
    /// The lowest price at which someone is willing to sell. Bots use this to assess sell-side liquidity and calculate spreads.
    #[serde(rename = "ask1Price", with = "string_to_float")]
    pub ask_price: f64,

    /// The best ask size.
    ///
    /// The quantity available at the best ask price. Bots use this to evaluate sell-side liquidity and potential slippage.
    #[serde(rename = "ask1Size", with = "string_to_float")]
    pub ask_size: f64,
}
