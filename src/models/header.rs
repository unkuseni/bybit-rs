use crate::prelude::*;

/// Header structure for trade stream responses.
///
/// Contains metadata for trade stream API responses, including rate limit information and timestamps. Bots use this to monitor API usage, ensure compliance with rate limits, and track response timing in real-time trading.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Header {
    /// The API rate limit for the endpoint.
    ///
    /// Specifies the total number of requests allowed within the rate limit window. Bots use this to manage API request pacing and avoid rate limit errors.
    #[serde(rename = "X-Bapi-Limit")]
    pub x_bapi_limit: String,

    /// The current rate limit status.
    ///
    /// Indicates the number of remaining requests within the rate limit window. Bots use this to dynamically adjust request frequency to stay within limits.
    #[serde(rename = "X-Bapi-Limit-Status")]
    pub x_bapi_limit_status: String,

    /// The timestamp when the rate limit window resets.
    ///
    /// Specifies when the rate limit counter will reset, as a string (e.g., "1625097600000"). Bots use this to schedule requests around reset times.
    #[serde(rename = "X-Bapi-Limit-Reset-Timestamp")]
    pub x_bapi_limit_reset_timestamp: String,

    /// The unique trace ID for the request.
    ///
    /// A unique identifier for tracking the request through Bybit’s system. Bots use this for debugging and correlating requests with responses.
    #[serde(rename = "Traceid")]
    pub traceid: String,

    /// The current server timestamp.
    ///
    /// The server’s current time, as a string (e.g., "1625097599123"). Bots use this to measure latency and ensure response freshness in real-time trading.
    #[serde(rename = "Timenow")]
    pub timenow: String,
}
