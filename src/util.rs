use chrono::{NaiveDate, NaiveTime, TimeZone, Utc};

use serde_json::Value;
use std::collections::BTreeMap;
use std::time::Instant;

use std::error::Error;

pub fn build_request(parameters: &BTreeMap<String, String>) -> String {
    parameters
        .iter()
        .map(|(key, value)| format!("{}={}", key, value))
        .collect::<Vec<_>>()
        .join("&")
}

pub fn build_signed_request(
    mut parameters: BTreeMap<String, String>,
    recv_window: u64,
) -> Result<String, Box<dyn Error>> {
    let start = Instant::now();
    if recv_window > 0 {
        parameters
            .entry("recvWindow".into())
            .or_insert_with(|| recv_window.to_string());
    }

    let timestamp: u64 = get_timestamp(start);
    parameters
        .entry("timestamp".into())
        .or_insert_with(|| timestamp.to_string());

    Ok(build_request(&parameters))
}

pub fn build_signed_request_custom(
    mut parameters: BTreeMap<String, String>,
    recv_window: Option<u64>,
    start: Option<Instant>,
) -> Result<String, Box<dyn Error>> {
    if let Some(window) = recv_window {
        parameters.insert("recvWindow".to_string(), window.to_string());
    }

    let timestamp = get_timestamp(start.unwrap_or_else(Instant::now));
    parameters.insert("timestamp".to_string(), timestamp.to_string());

    Ok(build_request(&parameters))
}

pub fn to_i64(value: &Value) -> i64 {
    if let Some(val) = value.as_i64() {
        val
    } else {
        panic!("Invalid value")
    }
}

pub fn to_u64(value: &Value) -> u64 {
    if let Some(val) = value.as_u64() {
        val
    } else {
        panic!("Invalid value")
    }
}

pub fn get_timestamp(start: Instant) -> u64 {
    start.elapsed().as_millis() as u64
}

pub fn date_to_milliseconds(date_str: &str) -> u64 {
    let naive_date = NaiveDate::parse_from_str(date_str, "%d%m%y").unwrap();
    let naive_time = NaiveTime::from_hms_opt(0, 0, 0).unwrap(); // Using from_hms as it does not fail
    let naive_date_time = naive_date.and_time(naive_time);
    let datetime_utc = Utc.from_utc_datetime(&naive_date_time);
    datetime_utc.timestamp_millis() as u64

}