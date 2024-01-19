use std::time::SystemTime;

use crate::api::API;
use crate::errors::{BybitContentError, ErrorKind, Result};
use crate::util::get_timestamp as util;
use error_chain::bail;
use hex::encode as hex_encode;
use hmac::{Hmac, Mac};
use once_cell::sync::OnceCell;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE, DATE, USER_AGENT};
use reqwest::Client as ReqwestClient;
use reqwest::Response;
use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use sha2::Sha256;

#[derive(Clone)]
pub struct Client {
    api_key: String,
    secret_key: String,
    host: String,
    inner_client: ReqwestClient,
}

impl Client {
    pub fn new(api_key: Option<String>, secret_key: Option<String>, host: String) -> Self {
        static INNER_CLIENT: OnceCell<ReqwestClient> = OnceCell::new();

        let inner_client = INNER_CLIENT.get_or_init(|| {
            ReqwestClient::builder()
                .pool_idle_timeout(None)
                .build()
                .expect("Failed to create reqwest client")
        });

        Client {
            api_key: api_key.unwrap_or_default(),
            secret_key: secret_key.unwrap_or_default(),
            host,
            inner_client: inner_client.clone(),
        }
    }

    // Helper function to create a signature for a POST request
    fn sign_post_request(
        &self,
        timestamp: i64,
        recv_window: i64,
        raw_request_body: &str,
    ) -> String {
        let param_str = format!(
            "{}{}{}{}",
            timestamp, self.api_key, recv_window, raw_request_body
        );
        let mut hmac =
            Hmac::<Sha256>::new_from_slice(self.secret_key.as_bytes()).expect("HMAC can take key");
        hmac.update(param_str.as_bytes());
        hex::encode(hmac.finalize().into_bytes())
    }

    // Helper function to create a signature for a GET request
    fn sign_get_request(&self, timestamp: &str, recv_window: &str, query_string: &str) -> String {
        let param_str = format!(
            "{}{}{}{}",
            timestamp, self.api_key, recv_window, query_string
        );
        let mut hmac =
            Hmac::<Sha256>::new_from_slice(self.secret_key.as_bytes()).expect("HMAC can take key");
        hmac.update(param_str.as_bytes());
        hex::encode(hmac.finalize().into_bytes())
    }

    // Call this function to sign a request with the appropriate method based on the HTTP method
    pub fn sign_request(
        &self,
        method: &str,
        timestamp: &str,
        recv_window: &str,
        body: Option<&str>,
        query_string: Option<&str>,
    ) -> String {
        match method {
            "POST" => {
                let raw_request_body = body.expect("POST requests should have a body");
                self.sign_post_request(
                    timestamp.parse().expect("Invalid timestamp"),
                    recv_window.parse().expect("Invalid recv_window"),
                    raw_request_body,
                )
            }
            "GET" => {
                let query_string = query_string.expect("GET requests should have a query string");
                self.sign_get_request(timestamp, recv_window, query_string)
            }
            _ => panic!("Unsupported HTTP method"),
        }
    }

    // Example usage:
    // let sign = client.sign_request("POST", "1658385579423", "5000", Some("{\"category\": \"option\"}"), None);
    // let sign = client.sign_request("GET", "1658384314791", "5000", None, Some("category=option&symbol=BTC-29JUL22-25000-C"));

    fn build_headers(&self, signed: bool, content_type: bool) -> Result<HeaderMap> {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static("Bybit/0.1.0"));

        if content_type {
            headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        }
        if signed {
            headers.insert(
                HeaderName::from_static("x-bapi-api-key"),
                HeaderValue::from_str(&self.api_key)?,
            );

            headers.insert(
                HeaderName::from_static("x-bapi-signature"),
                HeaderValue::from_str(&signature).map_err(|_| ErrorKind::HeaderValueError)?,
            );
        }
        let timestamp = util(SystemTime::now())?;
        headers.insert(
            HeaderName::from_static("x-bapi-timestamp"),
            HeaderValue::from_str(&timestamp.to_string())
                .map_err(|_| ErrorKind::HeaderValueError)?,
        );
        headers.insert(
            HeaderName::from_static("x-bapi-recv-window"),
            HeaderValue::from_static("5000"),
        );
    }

    //     pub fn get_signed<T: DeserializeOwned>(
    //     &self, endpoint: API, request: Option<String>,
    // ) -> Result<T> {
    //     // Perform a signed GET request to the specified endpoint with an optional request body
    //     let url = self.sign_request(endpoint, request); // Generate the signed URL for the request
    //     let client = &self.inner_client; // Get a reference to the inner HTTP client
    //     let response = client
    //         .get(url.as_str()) // Create a GET request to the signed URL
    //         .headers(self.build_headers(true)?) // Set the headers for the request
    //         .send()?; // Send the request and obtain the response

    //     self.handler(response) // Handle the response and return the deserialized result
    // }
}
