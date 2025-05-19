use crate::prelude::*;

/// Contains fee information for a pre-listing auction.
///
/// Specifies fees for the auction process. Not relevant for perpetual futures.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AuctionFeeInfo {
    /// The auction fee rate.
    ///
    /// The fee charged for participating in the auction. Not relevant for perpetuals.
    pub auction_fee_rate: String,
    /// The taker fee rate for the auction.
    ///
    /// The fee for taking liquidity during the auction. Not relevant for perpetuals.
    pub taker_fee_rate: String,
    /// The maker fee rate for the auction.
    ///
    /// The fee for providing liquidity during the auction. Not relevant for perpetuals.
    pub maker_fee_rate: String,
}
