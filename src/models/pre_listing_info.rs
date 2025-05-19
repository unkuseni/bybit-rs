use crate::prelude::*;

/// Contains pre-listing information for an instrument.
///
/// Provides details about instruments in the pre-listing phase (e.g., auctions). Not typically relevant for perpetual futures, which are already listed, but included for completeness.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PreListingInfo {
    /// The current auction phase (e.g., "DutchAuction").
    ///
    /// Indicates the stage of the pre-listing auction. Not relevant for perpetuals.
    pub cur_auction_phase: String,
    /// A list of pre-listing phases.
    ///
    /// Details the schedule of the auction phases. Not relevant for perpetuals.
    pub phases: Vec<PreListingPhase>,
    /// Auction fee information.
    ///
    /// Specifies fees for the pre-listing auction. Not relevant for perpetuals.
    pub auction_fee_info: AuctionFeeInfo,
}
