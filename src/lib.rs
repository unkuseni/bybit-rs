mod account;
mod api;
mod asset;
mod client;
mod config;
mod errors;
mod general;
mod market;
mod models;
mod position;
mod serde_helpers;
mod trade;
mod util;
mod ws;

pub mod prelude {

    pub use crate::account::*;
    pub use crate::api::*;
    pub use crate::asset::*;
    pub use crate::client::*;
    pub use crate::config::*;
    pub use crate::errors::*;
    pub use crate::general::*;
    pub use crate::market::*;
    pub use crate::models::*;
    pub use crate::position::*;
    pub use crate::serde_helpers::*;
    pub use crate::trade::*;
    pub use crate::util::*;
    pub use crate::ws::*;

    pub use core::f64;
    pub use serde::{Deserialize, Deserializer, Serialize, Serializer};
    pub use serde_json::{from_value, json, Value};
    pub use std::str::FromStr;
    pub use std::{borrow::Cow, collections::BTreeMap};
    pub use thiserror::Error;

    pub use std::any::type_name;

    pub use tokio::net::TcpStream;

    pub use hex::encode as hex_encode;
    pub use hmac::{Hmac, Mac};
    pub use reqwest::{
        header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE, USER_AGENT},
        Client as ReqwestClient, Response as ReqwestResponse, StatusCode,
    };

    pub use futures::sink::SinkExt;
    pub use log::error;
    pub use serde::de::DeserializeOwned;
    pub use sha2::Sha256;
    pub use tokio_tungstenite::WebSocketStream;
    pub use tokio_tungstenite::{connect_async, tungstenite::Message as WsMessage, MaybeTlsStream};
    pub use url::Url as WsUrl;
}

pub use prelude::*;
