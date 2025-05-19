use crate::prelude::*;

/// Details the account configuration and status.
///
/// Part of the `AccountInfoResponse`, this struct provides specific account settings, such as margin mode and unified margin status. Bots use this to verify account configuration and adjust trading strategies accordingly.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountInfo {
    /// The margin mode of the account (e.g., "ISOLATED_MARGIN", "REGULAR_MARGIN").
    ///
    /// Indicates whether the account uses isolated margin (per-position margin) or cross margin (shared margin across positions). Bots use this to manage risk and margin allocation in perpetual futures trading.
    pub margin_mode: String,
    /// The timestamp of the last account update in milliseconds.
    ///
    /// Indicates when the account settings were last modified. Bots use this to track configuration changes and ensure data freshness.
    #[serde(with = "string_to_u64")]
    pub updated_time: u64,
    /// The unified margin status (e.g., 1 for enabled, 0 for disabled).
    ///
    /// Indicates whether the account uses Bybit’s Unified Trading Account (UTA) for cross-asset margin sharing. Bots use this to determine margin flexibility and optimize capital allocation.
    pub unified_margin_status: i8,
    /// The Discounted Commission Plan (DCP) status.
    ///
    /// Indicates whether the account is enrolled in a discounted fee plan. Bots can use this to calculate trading costs more accurately if special fee structures apply.
    pub dcp_status: String,
    /// The time window for certain account restrictions (in seconds).
    ///
    /// Specifies the duration of any temporary restrictions, such as withdrawal limits after enabling UTA. Bots should monitor this to comply with account constraints.
    pub time_window: i32,
    /// The Self-Managed Portfolio (SMP) group ID.
    ///
    /// Identifies the SMP group for the account, used for copy trading or portfolio management. Bots use this to manage group-specific trading rules and restrictions.
    pub smp_group: i8,
    /// Whether the account is a master trader.
    ///
    /// If `true`, the account is a master trader in Bybit’s copy trading system. Bots use this to enable or restrict copy trading features based on account role.
    pub is_master_trader: bool,
    /// The spot hedging status (e.g., "ENABLED", "DISABLED").
    ///
    /// Indicates whether spot hedging is enabled, allowing offsetting positions between spot and futures. Bots use this to implement hedging strategies across markets.
    pub spot_hedging_status: String,
}
