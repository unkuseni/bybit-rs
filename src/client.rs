use crate::errors::{BybitContentError, ErrorKind, Result};
use error_chain::bail;
use hex::encode as hex_encode;
use hmac::{Hmac, Mac};
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE, USER_AGENT},
    Client as ReqwestClient, Response as ReqwestResponse, StatusCode,
};

use crate::api::API;
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
    pub fn new(api_key: String, secret_key: String, host: String) -> Self {
        Client {
            api_key: api_key.unwrap_or_default(),
            secret_key: secret_key.unwrap_or_default(),
            host,
            inner_client: ReqwestClient::builder()
                .pool_idle_timeout(None) // Set the idle connection timeout
                .build()
                .unwrap(),
        }
    }

    pub fn get<T: DeserializeOwned>(&self, endpoint: API, request: Option<String>) -> Result<T> {
        let mut url: String = format!("{}/{}", self.host, String::from(endpoint));
        if let Some(request) = request {
            if !request.is_empty() {
                url.push_str(format!("?{}", request).as_str());
            }
        }
        let client = &self.inner_client;
        let response = client.get(url.as_str()).send()?;
        self.handler(response)
    }


    /// Makes a signed HTTP GET request to the specified endpoint.
    pub fn get_signed<T: DeserializeOwned>(
        &self,
        endpoint: API,
        timestamp: u64,
        recv_window: u64,
        request: Option<String>,
    ) -> Result<T> {
        // Construct the full URL
        let mut url: String = format!("{}/{}", self.host, String::from(endpoint));
        let query_string = request.unwrap_or_default();
        if !query_string.is_empty() {
            url.push_str(format!("?{}", query_string).as_str());
        }

        // Sign the request
        let headers = self.sign_request(timestamp, recv_window, Some(query_string.as_str()))?;

        // Make the signed HTTP GET request
        let client = &self.inner_client;
        let response = client.get(url.as_str()).headers(headers).send()?;

        // Handle the response
        self.handler(response)
    }

    pub fn post<T: DeserializeOwned>(&self, endpoint: API, request: Option<String>) -> Result<T> {
        let mut url: String = format!("{}/{}", self.host, String::from(endpoint));
        if let Some(request) = request {
            if !request.is_empty() {
                url.push_str(format!("?{}", request).as_str());
            }
        }
        let client = &self.inner_client;
        let response = client.post(url.as_str()).send()?;
        self.handler(response)
    }

    /// Makes a signed HTTP POST request to the specified endpoint
    pub fn post_signed<T: DeserializeOwned>(
        &self,
        endpoint: API,
        timestamp: u64,
        recv_window: u64,
        raw_request_body: Option<String>,
    ) -> Result<T> {
        // Construct the full URL
        let url: String = format!("{}{}", self.host, String::from(endpoint));

        // Sign the request, passing the raw request body for signature
        let headers = self.sign_request(timestamp, recv_window, raw_request_body.as_deref())?;

        // Make the signed HTTP POST request
        let client = &self.inner_client;
        let response = client
            .post(url.as_str())
            .headers(headers)
            .body(raw_request_body.unwrap_or_default())
            .send()?;

        // Handle the response
        self.handler(response)
    }
    // Build headers for HTTPS requests
    const USER_AGENT: &'static str = "bybit-rs";
    const CONTENT_TYPE: &'static str = "application/json";

    fn build_headers(&self, content_type: bool, signed: bool) -> Result<HeaderMap> {
        let mut custom_headers = HeaderMap::new();
        custom_headers.insert(USER_AGENT, HeaderValue::from_static(USER_AGENT));
        if signed {
            custom_headers.insert(
                HeaderName::from_static("X-BAPI-SIGN"),
                HeaderValue::from_static("secret"),
            );
            custom_headers.insert(
                HeaderName::from_static("X-BAPI-API-KEY"),
                HeaderValue::from_str(&self.api_key)?,
            );
        }
        if content_type {
            custom_headers.insert(CONTENT_TYPE, HeaderValue::from_static(CONTENT_TYPE));
        }
        Ok(custom_headers)
    }

    fn build_signed_headers(
        &self,
        content_type: bool,
        signed: bool,
        timestamp: u64,
        recv_window: u64,
        request: Option<String>,
    ) -> Result<HeaderMap> {
        let mut custom_headers = HeaderMap::new();
        custom_headers.insert(USER_AGENT, HeaderValue::from_static(USER_AGENT));

        if content_type {
            custom_headers.insert(CONTENT_TYPE, HeaderValue::from_static(CONTENT_TYPE));
        }

        if signed {
            let signed_headers = self.sign_request(timestamp, recv_window, request)?;
            for (key, value) in signed_headers {
                custom_headers.insert(key, value);
            }
        }

        Ok(custom_headers)
    }

    fn sign_request(
        &self,
        timestamp: u64,
        recv_window: u64,
        request: Option<&str>,
    ) -> Result<HeaderMap, Box<dyn std::error::Error>> {
        let mut mac = Hmac::<Sha256>::new_varkey(self.secret_key.as_bytes())?;
        let mut sign_message = format!("{}{}{}", timestamp, self.api_key, recv_window);
        if let Some(req) = request {
            sign_message.push_str(req);
        }

        mac.update(sign_message.as_bytes());
        let hex_signature = hex_encode(mac.finalize().into_bytes());

        let mut headers = HeaderMap::new();
        headers.insert(
            HeaderName::from_static("X-BAPI-SIGN"),
            HeaderValue::from_str(&hex_signature)?,
        );
        headers.insert(
            HeaderName::from_static("X-BAPI-API-KEY"),
            HeaderValue::from_str(&self.api_key)?,
        );

        Ok(headers)
    }

    fn handler<T: DeserializeOwned>(&self, response: ReqwestResponse) -> Result<T> {
        match response.status() {
            StatusCode::OK => Ok(response.json::<T>()?),
            StatusCode::INTERNAL_SERVER_ERROR => {
                bail!("Internal Server Error");
            }
            StatusCode::SERVICE_UNAVAILABLE => {
                bail!("Service Unavailable");
            }
            StatusCode::UNAUTHORIZED => {
                bail!("Unauthorized");
            }
            StatusCode::BAD_REQUEST => {
                let error: BybitContentError = response.json()?;

                Err(ErrorKind::BybitError(error).into())
            }
            s => {
                bail!(format!("Received response: {:?}", s));
            }
        }
    }
}
