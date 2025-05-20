use crate::prelude::*;

/// Represents a single transaction log entry.
///
/// Details a specific transaction, such as a trade, funding fee, or margin adjustment, including associated costs and quantities. Bots use this to audit trading activity and calculate net performance in perpetual futures trading.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TransactionLogEntry {
    /// The unique identifier for the transaction.
    ///
    /// A unique ID assigned by Bybit to track the transaction. Bots use this to correlate transactions with other data, such as orders or executions.
    pub id: String,
    /// The trading pair symbol (e.g., "BTCUSDT").
    ///
    /// Identifies the perpetual futures contract or asset involved in the transaction. Bots use this to filter transactions by market.
    pub symbol: String,
    /// The trade side ("Buy" or "Sell").
    ///
    /// Indicates whether the transaction was a buy or sell action. Bots use this to track position direction and calculate net exposure.
    pub side: String,
    /// The funding fee (optional).
    ///
    /// The funding fee applied to the transaction, if applicable, in the settlement currency. Bots use this to calculate funding costs for perpetual futures positions.
    pub funding: Option<String>,
    /// The user-defined order link ID (optional).
    ///
    /// A custom identifier for the order, if set by the bot. Bots use this to track specific orders across transactions.
    pub order_link_id: Option<String>,
    /// The unique order ID.
    ///
    /// The Bybit-assigned ID for the order associated with the transaction. Bots use this to correlate transactions with specific orders.
    pub order_id: String,
    /// The transaction fee.
    ///
    /// The fee charged for the transaction, in the settlement currency. Bots use this to calculate trading costs and optimize fee-efficient strategies.
    #[serde(with = "string_to_float")]
    pub fee: f64,
    /// The balance change caused by the transaction.
    ///
    /// The net change in account balance due to the transaction, as a string (e.g., "+100.50" or "-50.25"). Bots use this to track account balance updates.
    pub change: String,
    /// The cash flow of the transaction.
    ///
    /// The net cash flow (positive or negative) resulting from the transaction, in the settlement currency. Bots use this to calculate liquidity impacts.
    #[serde(with = "string_to_float")]
    pub cash_flow: f64,
    /// The timestamp of the transaction.
    ///
    /// The time when the transaction occurred, as a string (e.g., "2025-05-19T13:07:00Z"). Bots use this to align transactions with market events.
    pub transaction_time: String,
    /// The type of transaction (e.g., "TRADE", "FUNDING").
    ///
    /// Specifies the nature of the transaction, such as trade execution or funding fee. Bots use this to categorize transactions for analysis.
    pub type_field: String,
    /// The fee rate applied to the transaction.
    ///
    /// The fee rate (e.g., "0.00075" for 0.075%) applied to the transaction. Bots use this to verify fee calculations and optimize trading costs.
    pub fee_rate: String,
    /// The bonus change, if any (optional).
    ///
    /// Any bonus or promotional balance changes applied to the transaction, in the settlement currency. Bots use this to account for special incentives.
    pub bonus_change: Option<String>,
    /// The position size affected by the transaction.
    ///
    /// The size of the position involved in the transaction, in base asset units. Bots use this to track position changes and calculate exposure.
    #[serde(with = "string_to_float")]
    pub size: f64,
    /// The quantity traded in the transaction.
    ///
    /// The amount of the base asset traded, in base asset units. Bots use this to verify trade execution details.
    #[serde(with = "string_to_float")]
    pub qty: f64,
    /// The resulting cash balance after the transaction.
    ///
    /// The account’s cash balance after the transaction, in the settlement currency. Bots use this to monitor available funds for trading.
    #[serde(with = "string_to_float")]
    pub cash_balance: f64,
    /// The currency of the transaction (e.g., "USDT").
    ///
    /// The settlement currency used for the transaction. Bots use this to ensure correct currency handling in multi-currency accounts.
    pub currency: String,
    /// The product category (e.g., "linear").
    ///
    /// The instrument type of the transaction, such as `linear` for USDT-margined perpetuals. Bots use this to filter transactions by contract type.
    pub category: String,
    /// The trade price.
    ///
    /// The price at which the trade was executed, as a string (e.g., "50000.00"). Bots use this to verify execution prices and calculate P&L.
    pub trade_price: String,
    /// The unique trade ID.
    ///
    /// A unique identifier for the trade execution associated with the transaction. Bots use this to track specific trade events.
    pub trade_id: String,
}
