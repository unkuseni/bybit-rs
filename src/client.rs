use tokio::net::TcpStream;

use crate::api::{WebsocketAPI, API};
use crate::errors::{BybitContentError, BybitError};
use crate::util::{generate_random_uid, get_timestamp};
use hex::encode as hex_encode;
use hmac::{Hmac, Mac};
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE, USER_AGENT},
    Client as ReqwestClient, Response as ReqwestResponse, StatusCode,
};

use futures::sink::SinkExt;
use serde::de::DeserializeOwned;
use serde_json::json;
use sha2::Sha256;
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::{connect_async, tungstenite::Message as WsMessage, MaybeTlsStream};
use url::Url as WsUrl;

#[derive(Clone)]
pub struct Client {
    api_key: String,
    secret_key: String,
    host: String,
    inner_client: ReqwestClient,
}

impl Client {
    pub fn new(api_key: Option<String>, secret_key: Option<String>, host: String) -> Self {
        let inner_client = ReqwestClient::builder()
            .build()
            .expect("Failed to build reqwest client");

        Client {
            api_key: api_key.unwrap_or_default(),
            secret_key: secret_key.unwrap_or_default(),
            host,
            inner_client,
        }
    }
    pub async fn get<T: DeserializeOwned + Send + 'static>(
        &self,
        endpoint: API,
        request: Option<String>,
    ) -> Result<T, BybitError> {
        let url = {
            let mut url = format!("{}/{}", self.host, String::from(endpoint));
            if let Some(request) = request {
                if !request.is_empty() {
                    url.push_str("?");
                    url.push_str(&request);
                }
            }
            url
        };

        let response = self.inner_client.get(url).send().await?;
        self.handler(response).await
    }
    /// Makes a signed HTTP GET request to the specified endpoint.
    pub async fn get_signed<T: DeserializeOwned + Send + 'static>(
        &self,
        endpoint: API,
        recv_window: u128,
        request: Option<String>,
    ) -> Result<T, BybitError> {
        // Construct the full URL
        let mut url: String = format!("{}/{}", self.host, String::from(endpoint));
        let query_string = request.unwrap_or_default();
        if !query_string.is_empty() {
            url.push_str(format!("?{}", query_string).as_str());
        }

        // Sign the request, passing the query string for signature
        let headers = self.build_signed_headers(false, true, recv_window, Some(query_string))?;

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
    ) -> Result<T, BybitError> {
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
        recv_window: u128,
        raw_request_body: Option<String>,
    ) -> Result<T, BybitError> {
        // Construct the full URL
        let url: String = format!("{}{}", self.host, String::from(endpoint));

        // Sign the request, passing the raw request body for signature
        let headers =
            self.build_signed_headers(true, true, recv_window, raw_request_body.clone())?;

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

    fn build_signed_headers<'str>(
        &self,
        content_type: bool,
        signed: bool,
        recv_window: u128,
        request: Option<String>,
    ) -> Result<HeaderMap, BybitError> {
        let mut custom_headers = HeaderMap::new();
        custom_headers.insert(USER_AGENT, HeaderValue::from_static("bybit-rs"));
        let timestamp = get_timestamp().to_string();
        let window = recv_window.to_string();
        let signature = self.sign_message(&timestamp, &window, request);

        let signature_header = HeaderName::from_static("x-bapi-sign");
        let api_key_header = HeaderName::from_static("x-bapi-api-key");
        let timestamp_header = HeaderName::from_static("x-bapi-timestamp");
        let recv_window_header = HeaderName::from_static("x-bapi-recv-window");

        if signed {
            custom_headers.insert(
                signature_header,
                HeaderValue::from_str(&signature.to_owned())?,
            );
            custom_headers.insert(
                api_key_header,
                HeaderValue::from_str(&self.api_key.to_owned())?,
            );
        }
        custom_headers.insert(
            timestamp_header,
            HeaderValue::from_str(&timestamp.to_owned())?,
        );
        custom_headers.insert(
            recv_window_header,
            HeaderValue::from_str(&window.to_owned())?,
        );
        if content_type {
            custom_headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        }
        Ok(custom_headers).map_err(|e| BybitError::ReqError(e))
    }

    fn sign_message(&self, timestamp: &str, recv_window: &str, request: Option<String>) -> String {
        let mut mac = Hmac::<Sha256>::new_from_slice(self.secret_key.as_bytes()).unwrap();
        let mut sign_message = format!("{}{}{}", timestamp, self.api_key, recv_window);
        if let Some(req) = request {
            sign_message.push_str(&req);
        }

        mac.update(sign_message.as_bytes());
        let hex_signature = hex_encode(mac.finalize().into_bytes());

        hex_signature
    }

    fn _sign_post_message(
        &self,
        timestamp: &str,
        recv_window: &str,
        request: Option<String>,
    ) -> String {
        let mut mac = Hmac::<Sha256>::new_from_slice(self.secret_key.as_bytes()).unwrap();
        mac.update(timestamp.as_bytes());
        mac.update(self.api_key.as_bytes());
        mac.update(recv_window.as_bytes());
        if let Some(req) = request {
            mac.update(req.as_bytes());
        }
        let hex_signature = hex_encode(mac.finalize().into_bytes());

        hex_signature
    }

    async fn handler<T: DeserializeOwned + Send + 'static>(
        &self,
        response: ReqwestResponse,
    ) -> Result<T, BybitError> {
        match response.status() {
            StatusCode::OK => {
                let response = response.json::<T>().await?;
                Ok(response)
            }
            StatusCode::BAD_REQUEST => {
                let error: BybitContentError = response.json().await.map_err(BybitError::from)?;
                Err(BybitError::BybitError(error).into())
            }
            StatusCode::INTERNAL_SERVER_ERROR => Err(BybitError::InternalServerError),
            StatusCode::SERVICE_UNAVAILABLE => Err(BybitError::ServiceUnavailable),
            StatusCode::UNAUTHORIZED => Err(BybitError::Unauthorized),
            status => Err(BybitError::StatusCode(status.as_u16())),
        }
    }

    pub async fn wss_connect(
        &self,
        endpoint: WebsocketAPI,
        request_body: Option<String>,
        private: bool,
        alive_dur: Option<u64>,
    ) -> Result<WebSocketStream<MaybeTlsStream<TcpStream>>, BybitError> {
        let unparsed_url = format!("{}{}", self.host, String::from(endpoint)).to_string();
        let url = WsUrl::parse(unparsed_url.as_str())?;
        let expiry_time = alive_dur.unwrap_or(0) * 1000 * 60;
        let expires = get_timestamp() + expiry_time;

        let mut mac = Hmac::<Sha256>::new_from_slice(self.secret_key.as_bytes()).unwrap();
        mac.update(format!("GET/realtime{expires}").as_bytes());
        let signature = hex_encode(mac.finalize().into_bytes());
        let uuid = generate_random_uid(5);

        match connect_async(url).await {
            Ok((mut ws_stream, _)) => {
                let auth_msg = json!({
                    "req_id": uuid,
                    "op": "auth",
                    "args": [self.api_key, expires, signature]
                });
                if private {
                    ws_stream
                        .send(WsMessage::Text(auth_msg.to_string()))
                        .await?;
                }
                let request = request_body.unwrap_or_else(String::new);
                ws_stream.send(WsMessage::Text(request)).await?;
                Ok(ws_stream)
            }
            Err(err) => Err(BybitError::Tungstenite(err)),
        }
    }
}
