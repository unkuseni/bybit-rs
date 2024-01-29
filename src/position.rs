use crate::api::{Position, API};
use crate::client::Client;
use crate::errors::Result;
use crate::model::*;
use crate::util::{build_request, date_to_milliseconds, build_json_request};

#[derive(Clone)]
pub struct PositionManager {
    pub client: Client,
    pub recv_window: u64,
}

impl PositionManager {
    pub async fn get_info(&self) {}
}
