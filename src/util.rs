use chrono::{NaiveDate, TimeZone, Utc};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde::Serialize;

use serde_json::Value;
use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn build_request<T: ToString>(parameters: &BTreeMap<String, T>) -> String {
    let mut request = String::with_capacity(parameters.iter().map(|(k, v)| k.len() + v.to_string().len() + 1).sum());
    for (key, value) in parameters {
        request.push_str(key);
        request.push('=');
        request.push_str(&value.to_string());
        request.push('&');
    }
    request.truncate(request.len() - 1);
    request
}

pub fn build_json_request<T: Serialize>(parameters: &BTreeMap<String, T>) -> String {
    serde_json::to_string(parameters).expect("Failed to serialize parameters to JSON")
}

pub fn to_i64(value: &Value) -> Option<i64> {
    value.as_i64()
}

pub fn to_u64(value: &Value) -> Option<u64> {
    value.as_u64()
}
pub fn get_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis() as u64
}

pub fn date_to_milliseconds(date_str: &str) -> u64 {
    let naive_date = NaiveDate::parse_from_str(date_str, "%d%m%y").unwrap();
    let naive_date_time = naive_date.and_hms_opt(0, 0, 0).unwrap();
    let datetime_utc = Utc.from_utc_datetime(&naive_date_time);
    datetime_utc.timestamp_millis() as u64
}

pub fn generate_random_uid(length: usize) -> String {
    let mut uid = String::with_capacity(length);
    for _ in 0..length {
        uid.push(thread_rng().sample(Alphanumeric) as char);
    }
    uid
}
