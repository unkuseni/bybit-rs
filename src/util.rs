use itertools::Itertools;
use serde_json::Value;
use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH}; // Add this import for step 1

use std::error::Error;

/// Builds a request string from a map of parameters.
///
/// # Arguments
///
/// * `parameters` - A reference to a map of parameters.
///
/// # Returns
///
/// A string representing the built request.
pub fn build_request(parameters: BTreeMap<String, String>) -> String {
    // Convert the map of parameters into an iterator of key-value pairs,
    // format each pair as a string in the format "key=value",
    // collect the formatted strings into a vector,
    // and then join the vector elements with the "&" separator.
    parameters
        .into_iter()
        .map(|(key, value)| format!("{}={}", key, value))
        .collect::<Vec<_>>()
        .join("&")
}

/// Builds a signed request string from a map of parameters and a receive window.
///
/// # Arguments
///
/// * `parameters` - A map of parameters.
/// * `recv_window` - The receive window value.
///
/// # Returns
///
/// A Result containing the built signed request string if successful, or an error if any.
pub fn build_signed_request(
    parameters: BTreeMap<String, String>,
    recv_window: u64,
) -> Result<String, Box<dyn Error>> {
    // Get the current time.
    let start = SystemTime::now();
    // Call the build_signed_request_custom function with the given parameters, receive window, and start time.
    build_signed_request_custom(parameters, recv_window, start)
}

/// Builds a signed request string from a map of parameters, a receive window, and a start time.
///
/// # Arguments
///
/// * `parameters` - A map of parameters.
/// * `recv_window` - The receive window value.
/// * `start` - The start time.
///
/// # Returns
///
/// A Result containing the built signed request string if successful, or an error if any.
pub fn build_signed_request_custom(
    mut parameters: BTreeMap<String, String>,
    recv_window: u64,
    start: SystemTime,
) -> Result<String, Box<dyn Error>> {
    // If the receive window is greater than 0, insert the "recvWindow" parameter with its value into the parameters map.
    if recv_window > 0 {
        parameters
            .entry("recvWindow".into())
            .or_insert_with(|| recv_window.to_string());
    }
    // Get the current timestamp and insert it into the parameters map.
    let timestamp: u64 = get_timestamp(start)?;
    parameters
        .entry("timestamp".into())
        .or_insert_with(|| timestamp.to_string());

    // Build the request string using the build_request function.
    Ok(build_request(parameters))
}

/// Converts a serde_json Value to an i64.
///
/// # Arguments
///
/// * `value` - The serde_json Value to convert.
///
/// # Returns
///
/// The converted i64 value.
pub fn to_i64(value: &Value) -> i64 {
    // Convert the serde_json Value to an i64, or panic if the conversion fails.
    value.as_i64().expect("Invalid value")
}

/// Converts a serde_json Value to a u64.
///
/// # Arguments
///
/// * `value` - The serde_json Value to convert.
///
/// # Returns
///
/// The converted u64 value.
pub fn to_u64(value: &Value) -> u64 {
    // Convert the serde_json Value to a u64, or panic if the conversion fails.
    value.as_u64().expect("Invalid value")
}

/// Gets the timestamp in milliseconds since the UNIX epoch.
///
/// # Arguments
///
/// * `start` - The start time.
///
/// # Returns
///
/// A Result containing the timestamp in milliseconds if successful, or an error if any.
pub fn get_timestamp(start: SystemTime) -> Result<u64, Box<dyn Error>> {
    // Calculate the duration between the start time and the UNIX epoch,
    // convert the duration to milliseconds,
    // and return the result.
    start
        .duration_since(UNIX_EPOCH)
        .map(|since_epoch| since_epoch.as_millis() as u64)
        .map_err(Into::into)
}