use crate::prelude::*;

/// ----------------------------------------
/// RESPONSE STRUCTS FOR MARKET REQUESTS
/// ----------------------------------------
/// Contains the server time in seconds and nanoseconds.
///
/// This struct is part of the `ServerTimeResponse` and provides high-precision server time, which is essential for timestamp-sensitive API requests in trading bots.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ServerTime {
    /// The server time in seconds since the Unix epoch.
    ///
    /// This is the primary timestamp used for validating API request timestamps. Trading bots must ensure their request timestamps are within a small window (e.g., ±5 seconds) of this value to avoid `10004` (timestamp error) rejections. For example, if `time_second` is `1697051234`, the bot's request timestamp should be very close to this value.
    #[serde(with = "string_to_u64")]
    pub time_second: u64,

    /// The nanosecond component of the server time.
    ///
    /// Provides sub-second precision for the server time. While not typically used directly in API requests, it can be useful for high-frequency trading bots requiring precise timing for latency measurements or event ordering. For most bots, this can be ignored unless nanosecond precision is critical.
    #[serde(with = "string_to_u64")]
    pub time_nano: u64,
}
