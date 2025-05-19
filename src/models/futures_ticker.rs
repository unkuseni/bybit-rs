use crate::prelude::*;

/// Represents ticker data for a futures contract (including perpetuals).
///
/// Contains real-time market data for a perpetual futures contract, such as prices, volumes, open interest, and funding rates. Bots use this for price monitoring, liquidity analysis, and funding cost estimation.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FuturesTicker {
    /// The trading pair symbol (e.g., "BTCUSDT").
    ///
    /// Identifies the perpetual futures contract. Bots should verify this matches their target pair.
    pub symbol: String,
    /// The last traded price.
    ///
    /// The most recent price at which the contract was traded. Bots use this for real-time price tracking and as a reference for order placement.
    #[serde(with = "string_to_float")]
    pub last_price: f64,
    /// The index price.
    ///
    /// The spot market price of the underlying asset, aggregated from multiple exchanges. In perpetual futures, this anchors the mark price to prevent manipulation. Bots use this to predict mark price movements and funding rates.
    #[serde(with = "string_to_float")]
    pub index_price: f64,
    /// The mark price.
    ///
    /// A smoothed price used for funding rate calculations and liquidation triggers in perpetual futures. Bots must monitor this to avoid liquidations, as positions are closed if the mark price hits the liquidation price.
    #[serde(with = "string_to_float")]
    pub mark_price: f64,
    /// The price 24 hours ago.
    ///
    /// The price of the contract 24 hours prior. Bots use this to calculate daily price changes and assess market trends.
    #[serde(with = "string_to_float")]
    pub prev_price_24h: f64,
    /// The percentage price change over the last 24 hours.
    ///
    /// Calculated as `(last_price - prev_price_24h) / prev_price_24h`. Bots use this to identify trends and volatility in perpetual futures.
    #[serde(rename = "price24hPcnt", with = "string_to_float")]
    pub daily_change_percentage: f64,
    /// The highest price in the last 24 hours.
    ///
    /// Indicates the peak price. Bots use this to assess resistance levels and volatility.
    #[serde(rename = "highPrice24h", with = "string_to_float")]
    pub high_24h: f64,
    /// The lowest price in the last 24 hours.
    ///
    /// Indicates the trough price. Bots use this to assess support levels and volatility.
    #[serde(rename = "lowPrice24h", with = "string_to_float")]
    pub low_24h: f64,
    /// The price 1 hour ago.
    ///
    /// The price of the contract 1 hour prior. Bots use this for short-term trend analysis.
    #[serde(with = "string_to_float")]
    pub prev_price_1h: f64,
    /// The total open interest.
    ///
    /// The total number of open contracts (in base asset units, e.g., BTC). High open interest indicates strong market participation, which can affect liquidity and volatility in perpetual futures. Bots use this to gauge market sentiment.
    #[serde(with = "string_to_float")]
    pub open_interest: f64,
    /// The value of open interest in the quote asset.
    ///
    /// The total value of open contracts in the quote asset (e.g., USDT). Bots use this to assess the financial scale of market participation.
    #[serde(with = "string_to_float")]
    pub open_interest_value: f64,
    /// The trading turnover in the last 24 hours (in quote asset).
    ///
    /// The total value traded in the quote asset (e.g., USDT). Bots use this to assess market activity and liquidity.
    #[serde(with = "string_to_float")]
    pub turnover_24h: f64,
    /// The trading volume in the last 24 hours (in base asset).
    ///
    /// The total quantity traded in the base asset (e.g., BTC). Bots use this to confirm trade signals, as high volume often indicates strong trends.
    #[serde(with = "string_to_float")]
    pub volume_24h: f64,
    /// The current funding rate.
    ///
    /// The rate paid between long and short position holders every funding interval (typically 8 hours). Positive rates mean longs pay shorts; negative rates mean shorts pay longs. Bots must monitor this to estimate funding costs, which can significantly impact profitability for long-term positions in perpetual futures.
    pub funding_rate: String,
    /// The timestamp of the next funding settlement (Unix timestamp in milliseconds).
    ///
    /// Indicates when the next funding rate will be applied. Bots should use this to time position adjustments to minimize funding costs.
    #[serde(with = "string_to_u64")]
    pub next_funding_time: u64,
    /// The predicted delivery price (empty for perpetuals).
    ///
    /// For perpetual futures, this is typically empty since there’s no delivery. Bots can ignore this.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub predicted_delivery_price: String,
    /// The basis rate (empty for perpetuals).
    ///
    /// For perpetual futures, this is typically empty. Bots can ignore this.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub basis_rate: String,
    /// The delivery fee rate (empty for perpetuals).
    ///
    /// For perpetual futures, this is typically empty. Bots can ignore this.
    pub delivery_fee_rate: String,
    /// The delivery time (empty for perpetuals).
    ///
    /// For perpetual futures, this is typically empty. Bots can ignore this.
    #[serde(with = "string_to_u64")]
    pub delivery_time: u64,
    /// The size of the best ask order.
    ///
    /// The quantity available at the best ask price. Bots use this to assess immediate selling liquidity.
    #[serde(rename = "ask1Size", with = "string_to_float")]
    pub ask_size: f64,
    /// The best bid price.
    ///
    /// The highest price buyers are willing to pay. Bots use this to determine the current market bid price for sell orders.
    #[serde(rename = "bid1Price", with = "string_to_float")]
    pub bid_price: f64,
    /// The best ask price.
    ///
    /// The lowest price sellers are willing to accept. Bots use this to determine the current market ask price for buy orders.
    #[serde(rename = "ask1Price", with = "string_to_float")]
    pub ask_price: f64,
    /// The size of the best bid order.
    ///
    /// The quantity available at the best bid price. Bots use this to assess immediate buying liquidity.
    #[serde(rename = "bid1Size", with = "string_to_float")]
    pub bid_size: f64,
    /// The basis (empty for perpetuals).
    ///
    /// For perpetual futures, this is typically empty. Bots can ignore this.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub basis: String,
}
