use crate::errors::{BybitContentError, ErrorKind, Result};
use error_chain::bail;
use hex::encode as hex_encode;
use hmac::{Hmac, Mac};
use reqwest::{
    header::{ HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE, USER_AGENT},
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
    pub fn new(api_key: Option<String>, secret_key: Option<String>, host: String) -> Self {
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
    pub async fn get<T: DeserializeOwned + Send + 'static>(
        &self,
        endpoint: API,
        request: Option<String>,
    ) -> Result<T> {
        let mut url: String = format!("{}/{}", self.host, String::from(endpoint));
        if let Some(request) = request {
            if !request.is_empty() {
                url.push_str(&format!("?{}", request));
            }
        }
        let client = &self.inner_client;
        let response = client.get(url.as_str()).send().await?;
        self.handler(response).await
    }

    /// Makes a signed HTTP GET request to the specified endpoint.
    pub async fn get_signed<T: DeserializeOwned + Send + 'static>(
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

            // Sign the request, passing the query string for signature
        let headers = self.build_signed_headers(false,true, timestamp, recv_window, Some(query_string)).await?;

        // Make the signed HTTP GET request
        let client = &self.inner_client;
        let response = client.get(url.as_str()).headers(headers).send().await?;

        // Handle the response
        self.handler(response).await
    }

    pub async fn post<T: DeserializeOwned + Send + 'static>(
        &self,
        endpoint: API,
        request: Option<String>,
    ) -> Result<T> {
        let mut url: String = format!("{}/{}", self.host, String::from(endpoint));
        if let Some(request) = request {
            if !request.is_empty() {
                url.push_str(format!("?{}", request).as_str());
            }
        }
        let client = &self.inner_client;
        let response = client.post(url.as_str()).send().await?;
        self.handler(response).await
    }

    /// Makes a signed HTTP POST request to the specified endpoint
    pub async fn post_signed<T: DeserializeOwned + Send + 'static>(
        &self,
        endpoint: API,
        timestamp: u64,
        recv_window: u64,
        raw_request_body: Option<String>,
    ) -> Result<T> {
        // Construct the full URL
        let url: String = format!("{}{}", self.host, String::from(endpoint));

        // Sign the request, passing the raw request body for signature
        let headers = self.build_signed_headers(true,true, timestamp, recv_window, raw_request_body.clone()).await?;

        // Make the signed HTTP POST request
        let client = &self.inner_client;
        let response = client
            .post(url.as_str())
            .headers(headers)
            .body(raw_request_body.unwrap_or_default())
            .send()
            .await?;

        // Handle the response
        self.handler(response).await
    }


    async fn build_signed_headers(
        &self,
        content_type: bool,
        signed: bool,
        timestamp: u64,
        recv_window: u64,
        request: Option<String>,
    ) -> Result<HeaderMap> {
        let mut custom_headers = HeaderMap::new();
        custom_headers.insert(USER_AGENT, HeaderValue::from_static("bybit-rs"));

        
        if signed {
            let signed_headers = self.sign_request(timestamp, recv_window, request.as_deref())?;
            for (key, value) in signed_headers {
                if let Some(header_key) = key {
                    custom_headers.insert(header_key, value);
                }
            }
        }
        
        if content_type {
            custom_headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        }
        Ok(custom_headers)
    }

    fn sign_request(
        &self,
        timestamp: u64,
        recv_window: u64,
        request: Option<&str>,
    ) -> Result<HeaderMap> {
        let mut mac = Hmac::<Sha256>::new_from_slice(self.secret_key.as_bytes()).unwrap();
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

    async fn handler<T: DeserializeOwned + Send + 'static>(
        &self,
        response: ReqwestResponse,
    ) -> Result<T> {
        match response.status() {
            StatusCode::OK => {
               let response = response.json::<T>().await?;
               Ok(response)
            }
            StatusCode::BAD_REQUEST => {
                let error: BybitContentError = response.json().await?;
                Err(ErrorKind::BybitError(error).into())
            }
             StatusCode::INTERNAL_SERVER_ERROR => {
                bail!("Internal Server Error");
            }
            StatusCode::SERVICE_UNAVAILABLE => {
                bail!("Service Unavailable");
            }
            StatusCode::UNAUTHORIZED => {
                bail!("Unauthorized");
            }
            status => bail!("Received error response: {:?}", status),
        }
    }
}
