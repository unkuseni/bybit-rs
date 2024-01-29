use rand::distributions::Alphanumeric;
use rand::{Rng, thread_rng};
use chrono::{NaiveDate, NaiveTime, TimeZone, Utc};
use serde::Serialize;

use serde_json::Value;
use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};
use std::fmt::Write;

pub fn build_request<T: ToString>(parameters: &BTreeMap<String, T>) -> String {
    let mut request = String::with_capacity(parameters.iter().map(|(k, v)| k.len() + v.to_string().len() + 1).sum::<usize>() + parameters.len() * 2);
    for (key, value) in parameters {
        if !request.is_empty() {
            request.push('&');
        }
        write!(request, "{}={}", key, value.to_string()).expect("Failed to write to String");
    }
    request
}


pub fn build_json_request<T: Serialize>(parameters: &BTreeMap<String, T>) -> String {
    serde_json::to_string(parameters).expect("Failed to serialize parameters to JSON")
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
pub fn get_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis() as u64
}


pub fn date_to_milliseconds(date_str: &str) -> u64 {
    let naive_date = NaiveDate::parse_from_str(date_str, "%d%m%y").unwrap();
    let naive_time = NaiveTime::from_hms_opt(0, 0, 0).unwrap(); // Using from_hms as it does not fail
    let naive_date_time = naive_date.and_time(naive_time);
    let datetime_utc = Utc.from_utc_datetime(&naive_date_time);
    datetime_utc.timestamp_millis() as u64

}


 pub fn generate_random_uid(length: usize) -> String {
    let uid: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();
    uid
}