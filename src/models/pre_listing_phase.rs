use crate::prelude::*;

/// Represents a single pre-listing phase.
///
/// Details a phase in the pre-listing auction process. Not relevant for perpetual futures.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PreListingPhase {
    /// The phase name (e.g., "DutchAuction").
    ///
    /// Identifies the auction phase. Not relevant for perpetuals.
    pub phase: String,
    /// The start time of the phase (Unix timestamp in milliseconds).
    ///
    /// Marks the beginning of the phase. Not relevant for perpetuals.
    #[serde(with = "string_to_u64")]
    pub start_time: u64,
    /// The end time of the phase (Unix timestamp in milliseconds).
    ///
    /// Marks the end of the phase. Not relevant for perpetuals.
    #[serde(with = "string_to_u64")]
    pub end_time: u64,
}
