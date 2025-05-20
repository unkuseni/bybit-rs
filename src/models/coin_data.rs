use crate::prelude::*;

/// Represents balance and margin data for a specific currency in the account.
///
/// This struct details the financial state of a single currency (e.g., USDT, BTC) within a Bybit account, used for multi-currency margin management (https://bybit-exchange.github.io/docs/v5/account/wallet-balance).
/// For trading bots, `CoinData` is critical for allocating collateral, managing borrowing, and tracking currency-specific PNL in perpetual futures.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CoinData {
    /// The currency, e.g., "USDT", "BTC".
    ///
    /// This identifies the coin for which balance and margin data is provided, critical for multi-currency accounts (https://bybit-exchange.github.io/docs/v5/account/wallet-balance).
    /// **Bot Implication**: Bots use `coin` to route funds and collateral correctly, ensuring proper currency alignment for trades and margin.
    pub coin: String,

    /// Total equity for the currency, including wallet balance and unrealized PNL.
    ///
    /// This represents the net worth of the currency in the account, used for leverage and risk calculations (https://bybit-exchange.github.io/docs/v5/account/wallet-balance).
    /// **Bot Implication**: Bots monitor `equity` to assess currency-specific financial health, guiding allocation and risk decisions.
    #[serde(with = "string_to_float")]
    pub equity: f64,

    /// USD value of the currency’s equity, converted at the current market rate.
    ///
    /// This normalizes the currency’s equity to USD for cross-currency comparisons and risk aggregation (https://bybit-exchange.github.io/docs/v5/account/wallet-balance).
    /// **Bot Implication**: Bots use `usd_value` to standardize risk exposure across currencies, simplifying account-level risk management.
    #[serde(with = "string_to_float")]
    pub usd_value: f64,

    /// Available balance for the currency, excluding margin and locked funds.
    ///
    /// This is the liquid balance available for trading or withdrawal in the specific currency (https://bybit-exchange.github.io/docs/v5/account/wallet-balance).
    /// **Bot Implication**: Bots track `wallet_balance` to ensure sufficient liquidity for new positions or margin top-ups in the currency.
    #[serde(with = "string_to_float")]
    pub wallet_balance: f64,

    /// Amount available for withdrawal in the currency, if specified.
    ///
    /// This reflects the portion of the wallet balance that can be withdrawn, accounting for restrictions like pending orders (https://bybit-exchange.github.io/docs/v5/account/wallet-balance).
    /// **Bot Implication**: Bots use `available_to_withdraw` to manage cash flow, ensuring funds are accessible when needed without disrupting trading.
    #[serde(with = "string_to_float_optional")]
    pub available_to_withdraw: Option<f64>,

    /// Amount available to borrow in the currency, if specified.
    ///
    /// This indicates the additional leverage available for the currency in Bybit’s unified margin account (https://bybit-exchange.github.io/docs/v5/account/borrow).
    /// **Bot Implication**: bots use `available_to_borrow` to expand positions, but must balance borrowing with liquidation risks in volatile markets.
    #[serde(with = "string_to_float_optional")]
    pub available_to_borrow: Option<f64>,

    /// Total borrowed amount in the currency.
    ///
    /// This tracks loans taken in the currency, impacting leverage and interest costs (https://bybit-exchange.github.io/docs/v5/account/borrow).
    /// **Bot Implication**: Bots monitor `borrow_amount` to manage interest expenses and ensure borrowed funds don’t exceed risk thresholds.
    #[serde(with = "string_to_float")]
    pub borrow_amount: f64,

    /// Accrued interest on borrowed funds in the currency.
    ///
    /// This represents the cost of borrowing, which accumulates over time and affects profitability (https://bybit-exchange.github.io/docs/v5/account/borrow).
    /// **Bot Implication**: Bots track `accrued_interest` to optimize borrowing strategies, minimizing costs while maintaining leverage.
    #[serde(with = "string_to_float")]
    pub accrued_interest: f64,

    /// Initial margin for orders in the currency.
    ///
    /// This is the collateral required for open orders in the currency, impacting available balance (https://bybit-exchange.github.io/docs/v5/account/risk-limit).
    /// **Bot Implication**: Bots use `total_order_im` to allocate margin efficiently, ensuring orders don’t overcommit funds.
    #[serde(rename = "totalOrderIM", with = "string_to_float")]
    pub total_order_im: f64,

    /// Initial margin for positions in the currency, as a string.
    ///
    /// This is the collateral tied up in open positions, expressed as a string for precision (https://bybit-exchange.github.io/docs/v5/account/risk-limit).
    /// **Bot Implication**: Bots monitor `total_position_im` to manage position margin, preventing over-leveraging or margin shortages.
    #[serde(rename = "totalPositionIM", with = "string_to_float")]
    pub total_position_im: f64,

    /// Maintenance margin for positions in the currency, as a string.
    ///
    /// This is the minimum collateral needed to maintain positions, critical for avoiding liquidations (https://bybit-exchange.github.io/docs/v5/account/risk-limit).
    /// **Bot Implication**: Bots prioritize `total_position_mm` to ensure positions remain funded, adjusting leverage or topping up margin as needed.
    #[serde(rename = "totalPositionMM", with = "string_to_float")]
    pub total_position_mm: f64,

    /// Unrealized profit and loss for positions in the currency.
    ///
    /// This reflects paper gains or losses on open positions, affecting equity and margin (https://bybit-exchange.github.io/docs/v5/account/position).
    /// **Bot Implication**: Bots use `unrealised_pnl` to evaluate position performance and adjust risk controls, as large losses can trigger margin calls.
    #[serde(with = "string_to_float")]
    pub unrealised_pnl: f64,

    /// Cumulative realized profit and loss for the currency.
    ///
    /// This tracks the net realized gains or losses from closed positions, impacting account equity (https://bybit-exchange.github.io/docs/v5/account/position).
    /// **Bot Implication**: Bots use `cum_realised_pnl` to assess strategy performance and update capital allocation based on realized outcomes.
    #[serde(with = "string_to_float")]
    pub cum_realised_pnl: f64,

    /// Bonus balance in the currency, e.g., promotional credits.
    ///
    /// This represents non-withdrawable funds, often from Bybit promotions, used for trading (https://bybit-exchange.github.io/docs/v5/account/wallet-balance).
    /// **Bot Implication**: Bots account for `bonus` in balance calculations, but must adjust for withdrawal restrictions when planning cash flow.
    #[serde(with = "string_to_float")]
    pub bonus: f64,

    /// Indicates if the currency can be used as collateral.
    ///
    /// When `true`, the currency is eligible for margin calculations in Bybit’s unified account (https://bybit-exchange.github.io/docs/v5/account/margin-mode).
    /// **Bot Implication**: Bots check `collateral_switch` to ensure only eligible currencies are used for margin, avoiding funding errors.
    pub collateral_switch: bool,

    /// Indicates if the currency is used as margin collateral.
    ///
    /// When `true`, the currency actively contributes to margin requirements (https://bybit-exchange.github.io/docs/v5/account/margin-mode).
    /// **Bot Implication**: Bots prioritize `margin_collateral` currencies for margin allocation, optimizing collateral efficiency.
    pub margin_collateral: bool,

    /// Amount of the currency locked, e.g., for pending orders, as a string.
    ///
    /// This represents funds tied up in non-withdrawable states, reducing available balance (https://bybit-exchange.github.io/docs/v5/account/wallet-balance).
    /// **Bot Implication**: Bots track `locked` to avoid overestimating available funds, ensuring accurate liquidity planning.
    #[serde(with = "string_to_float")]
    pub locked: f64,

    /// Quantity of the currency used for spot hedging.
    ///
    /// This reflects the amount allocated to spot positions for hedging futures exposure, as a string (https://bybit-exchange.github.io/docs/v5/account/position).
    /// **Bot Implication**: Bots use `spot_hedging_qty` to balance futures and spot positions, minimizing directional risk in hedging strategies.
    #[serde(with = "string_to_float")]
    pub spot_hedging_qty: f64,
}
