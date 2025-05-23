#![allow(unused_imports, unreachable_code, unused_variables)]
use crate::prelude::*;

#[derive(Clone)]
pub struct AssetManager {
    pub client: Client,
    pub recv_window: u16,
}

impl AssetManager {
    pub async fn get_delivery_record(
        &self,
        category: Category,
        symbol: Option<&str>,
        start_time: Option<u64>,
        end_time: Option<u64>,
        limit: Option<u64>,
        exp_date: Option<&str>,
        cursor: Option<&str>,
    ) -> Result<DeliveryRecordResponse, BybitError> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("category".into(), category.as_str().into());
        if let Some(s) = symbol {
            parameters.insert("symbol".into(), s.into());
        }
        if let Some(s) = start_time {
            parameters.insert("startTime".into(), s.to_string());
        }
        if let Some(s) = end_time {
            parameters.insert("endTime".into(), s.to_string());
        }
        if let Some(s) = limit {
            parameters.insert("limit".into(), s.to_string());
        }
        if let Some(s) = exp_date {
            parameters.insert("expDate".into(), s.into());
        }
        if let Some(s) = cursor {
            parameters.insert("cursor".into(), s.into());
        }
        let request = build_request(&parameters);

        let response: DeliveryRecordResponse = self
            .client
            .get(API::Asset(Asset::DeliveryRecord), Some(request))
            .await?;

        Ok(response)
    }
}
