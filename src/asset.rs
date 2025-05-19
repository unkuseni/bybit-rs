#![allow(unused_imports, unreachable_code, unused_variables)]
use crate::api::{Asset, API};
use crate::client::Client;
use serde_json::{json, Value};

use crate::util::{build_json_request, build_request};

#[derive(Clone)]
pub struct AssetManager {
    pub client: Client,
    pub recv_window: u16,
}

impl AssetManager {}
