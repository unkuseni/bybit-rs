use crate::prelude::*;

/// Represents wallet data, including balances and margin metrics, for a Bybit account.
///
/// This struct provides a snapshot of the account’s financial state, used for monitoring margin, equity, and risk in perpetual futures trading (https://bybit-exchange.github.io/docs/v5/account/wallet-balance).
/// For trading bots, wallet data is critical for ensuring sufficient margin, avoiding liquidations, and optimizing capital allocation.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WalletData {
    /// Initial margin rate for the account, as a string (e.g., "0.1" for 10%).
    ///
    /// This represents the ratio of initial margin to total equity, a key risk metric in perpetual futures. A higher rate indicates greater leverage and risk (https://bybit-exchange.github.io/docs/v5/account/risk-limit).
    /// **Bot Implication**: Bots monitor `account_im_rate` to ensure compliance with Bybit’s margin requirements, adjusting positions to avoid margin calls.
    #[serde(rename = "accountIMRate", with = "string_to_float_optional")]
    pub account_im_rate: Option<f64>,

    /// Maintenance margin rate for the account, as a string (e.g., "0.05" for 5%).
    ///
    /// This is the minimum margin ratio required to maintain open positions. Falling below this triggers a margin call or liquidation (https://bybit-exchange.github.io/docs/v5/account/risk-limit).
    /// **Bot Implication**: Bots prioritize `account_mm_rate` to prevent forced liquidations, especially in volatile markets where margin requirements can spike.
    #[serde(rename = "accountMMRate", with = "string_to_float_optional")]
    pub account_mm_rate: Option<f64>,

    /// Total equity in the account, in USDT or the base currency.
    ///
    /// This represents the account’s net worth, including wallet balance and unrealized PNL (https://bybit-exchange.github.io/docs/v5/account/wallet-balance).
    /// **Bot Implication**: Bots use `total_equity` to assess overall account health and allocate capital across positions, ensuring sufficient buffer for market swings.
    #[serde(with = "string_to_float")]
    pub total_equity: f64,

    /// Total wallet balance, excluding unrealized PNL.
    ///
    /// This is the actual cash balance in the account, available for trading or withdrawal (https://bybit-exchange.github.io/docs/v5/account/wallet-balance).
    /// **Bot Implication**: Bots track `total_wallet_balance` to ensure liquidity for new trades and margin requirements, avoiding funding shortages.
    #[serde(with = "string_to_float")]
    pub total_wallet_balance: f64,

    /// Total margin balance, including wallet balance and unrealized PNL.
    ///
    /// This reflects the account’s total margin capacity, used to support open positions (https://bybit-exchange.github.io/docs/v5/account/wallet-balance).
    /// **Bot Implication**: Bots use `total_margin_balance` to calculate leverage limits and ensure positions are adequately margined.
    #[serde(with = "string_to_float_optional")]
    pub total_margin_balance: Option<f64>,

    /// Total available balance for new trades or withdrawals.
    ///
    /// This is the portion of the wallet balance not tied up in margin or open positions (https://bybit-exchange.github.io/docs/v5/account/wallet-balance).
    /// **Bot Implication**: Bots rely on `total_available_balance` to determine capacity for new trades, ensuring no over-allocation of funds.
    #[serde(with = "string_to_float_optional")]
    pub total_available_balance: Option<f64>,

    /// Total unrealized profit and loss for perpetual futures positions.
    ///
    /// This reflects the paper gains or losses on open positions, impacting equity and margin calculations (https://bybit-exchange.github.io/docs/v5/account/position).
    /// **Bot Implication**: Bots monitor `total_perp_upl` to assess position performance and adjust risk controls, as large unrealized losses can trigger liquidations.
    #[serde(rename = "totalPerpUPL", with = "string_to_float")]
    pub total_perp_upl: f64,

    /// Total initial margin required for open positions and orders.
    ///
    /// This is the collateral required to open and maintain positions, based on Bybit’s risk limits (https://bybit-exchange.github.io/docs/v5/account/risk-limit).
    /// **Bot Implication**: Bots track `total_initial_margin` to ensure sufficient margin allocation, preventing rejections or forced closures.
    #[serde(with = "string_to_float_optional")]
    pub total_initial_margin: Option<f64>,

    /// Total maintenance margin required to keep positions open.
    ///
    /// This is the minimum collateral needed to avoid liquidation, typically lower than initial margin (https://bybit-exchange.github.io/docs/v5/account/risk-limit).
    /// **Bot Implication**: Bots prioritize `total_maintenance_margin` to maintain account stability, as falling below this triggers risk management actions.
    #[serde(with = "string_to_float_optional")]
    pub total_maintenance_margin: Option<f64>,

    /// List of coin-specific balance data for the account.
    ///
    /// This breaks down the wallet by currency, providing granular balance and margin details (https://bybit-exchange.github.io/docs/v5/account/wallet-balance).
    /// **Bot Implication**: Bots use `coin` to manage multi-currency accounts, ensuring proper funding and collateral allocation per currency.
    pub coin: Vec<CoinData>,

    /// Loan-to-value ratio for the account, as a string (e.g., "0.2" for 20%).
    ///
    /// This measures borrowed funds relative to total equity, a leverage risk indicator in Bybit’s unified margin account (https://bybit-exchange.github.io/docs/v5/account/risk-limit).
    /// **Bot Implication**: Bots monitor `account_ltv` to control leverage risk, as high LTV increases liquidation probability in adverse markets.
    #[serde(rename = "accountLTV", with = "string_to_float_optional")]
    pub account_ltv: Option<f64>,

    /// Type of account, e.g., "UNIFIED", if specified.
    ///
    /// This indicates the account’s margin mode, such as unified (cross-margin) or isolated (per-position margin) (https://bybit-exchange.github.io/docs/v5/account/margin-mode).
    /// **Bot Implication**: Bots must align strategies with `account_type`, as margin modes affect risk calculations and liquidation mechanics.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
}
