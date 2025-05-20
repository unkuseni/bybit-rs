use crate::prelude::*;

/// Represents the fee rates for a trading pair on Bybit.
///
/// This struct is used to store the maker and taker fee rates for a specific symbol, which are critical for calculating trading costs in perpetual futures trading. Fees impact the profitability of trading strategies, especially for high-frequency trading bots.
///
/// # Bybit API Reference
/// According to the Bybit API documentation (https://bybit-exchange.github.io/docs/v5/intro), fee rates are provided per symbol and differ between maker (adding liquidity) and taker (removing liquidity) orders. These rates are typically expressed as percentages (e.g., "0.0002" for 0.02%).
///
/// # Perpetual Futures Context
/// In perpetual futures, fees are charged on each trade and can significantly affect profitability, especially for strategies with thin margins. Maker fees are generally lower to incentivize liquidity provision, while taker fees are higher. Trading bots must account for these fees in cost calculations to optimize profit and loss (PnL).
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FeeRate {
    /// The trading pair symbol (e.g., "BTCUSDT").
    ///
    /// This identifies the specific market to which the fee rates apply. For trading bots, this field is essential for mapping fee structures to specific markets, ensuring accurate cost calculations per trade.
    pub symbol: String,
    /// The fee rate charged for maker orders (as a string, e.g., "0.0001" for 0.01%).
    ///
    /// Maker orders add liquidity to the order book by placing limit orders that are not immediately matched. Lower maker fees incentivize bots to provide liquidity, which can be a strategy for reducing trading costs. Bots should monitor this rate to optimize order placement strategies, especially in market-making algorithms.
    pub maker_fee_rate: String,
    /// The fee rate charged for taker orders (as a string, e.g., "0.0006" for 0.06%).
    ///
    /// Taker orders remove liquidity by matching existing orders (e.g., market orders). Higher taker fees mean bots executing immediate trades pay more, impacting profitability. Bots should compare maker vs. taker fees to decide whether to place limit or market orders based on strategy goals.
    pub taker_fee_rate: String,
}
