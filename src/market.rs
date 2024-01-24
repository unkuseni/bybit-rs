use crate::api::{Market, API};
use crate::client::Client;
use crate::errors::Result;
use crate::model::{
    IndexPriceKlineResponse, IndexPriceKlineSummary, Instrument, InstrumentsInfoResponse, Kline, KlineResponse, MarkPriceKline, MarkPriceKlineResponse, MarkPriceKlineSummary, OrderBook, OrderBookResponse, PremiumIndexPriceKlineResponse, PremiumIndexPriceKlineSummary
};
use crate::util::{build_request, date_to_milliseconds};
use serde_json::Value;
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::convert::TryInto;

#[derive(Clone)]
pub struct MarketData {
    pub client: Client,
    pub recv_window: u64,
}

pub enum Category {
    Spot,
    Linear,
    Inverse,
    Option,
}
impl Category {
    pub fn as_str(&self) -> &str {
        match self {
            Category::Spot => "spot",
            Category::Linear => "linear",
            Category::Inverse => "inverse",
            Category::Option => "option",
        }
    }
}
/// Market Data endpoints

impl MarketData {
    /// Query for historical klines also known as candlesticks. Charts are returned in groups based on requested interval
    /// Covers: Spot / USDT perpetual / USDC contract / Inverse contract
    /// Symbol and Interval is required
    ///  Pass start and end as NaiveDate with the format DDMMYY

    pub async fn get_klines(
        &self,
        category: Option<Category>,
        symbol: String,
        interval: String,
        start: Option<Cow<'_, str>>,
        end: Option<Cow<'_, str>>,
        limit: Option<u64>,
    ) -> Result<Vec<Kline>> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        if let Some(cat) = category {
            parameters
                .entry("category".to_owned())
                .or_insert_with(|| cat.as_str().to_owned());
        } else {
            parameters
                .entry("category".to_owned())
                .or_insert_with(|| Category::Linear.as_str().to_owned());
        }
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("interval".into(),  interval.into());
        if let Some(start_str) = start.as_ref().map(|s| s.as_ref()) {
            let start_millis = date_to_milliseconds(start_str);
            parameters
                .entry("start".to_owned())
                .or_insert_with(|| start_millis.to_string());
        }
        if let Some(end_str) = end.as_ref().map(|s| s.as_ref()) {
            let end_millis = date_to_milliseconds(end_str);
            parameters
                .entry("end".to_owned())
                .or_insert_with(|| end_millis.to_string());
        }
        if let Some(l) = limit {
            parameters
                .entry("limit".to_owned())
                .or_insert_with(|| l.to_string());
        }
        let request = build_request(&parameters);
        let response: KlineResponse = self
            .client
            .get(API::Market(Market::Kline), Some(request))
            .await?;
        Ok(response.result.list)
    }

    /// Query for historical mark price klines. Charts are returned in groups based on the requested interval.
    /// Covers: USDT perpetual / USDC contract / Inverse contract
    /// Symbol and Interval is required
    ///  Pass start and end as NaiveDate with the format DDMMYY
    pub async fn get_mark_price_klines(
        &self,
        category: Option<Category>,
        symbol: String,
        interval: String,
        start: Option<Cow<'_, str>>,
        end: Option<Cow<'_, str>>,
        limit: Option<u64>,
    ) -> Result<MarkPriceKlineSummary> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        if let Some(category) = category {
            match category {
                Category::Linear | Category::Inverse => {
                    parameters.insert("category".to_owned(), category.as_str().to_owned());
                }
                _ => return Err("Category must be either Linear or Inverse".into()),
            }
        } else {
            parameters.insert("category".to_owned(), Category::Linear.as_str().to_string());
        }
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("interval".into(),  interval.into());
        if let Some(start_str) = start.as_ref().map(|s| s.as_ref()) {
            let start_millis = date_to_milliseconds(start_str);
            parameters
                .entry("start".to_owned())
                .or_insert_with(|| start_millis.to_string());
        }
        if let Some(end_str) = end.as_ref().map(|s| s.as_ref()) {
            let end_millis = date_to_milliseconds(end_str);
            parameters
                .entry("end".to_owned())
                .or_insert_with(|| end_millis.to_string());
        }

        if let Some(l) = limit {
            parameters
                .entry("limit".to_owned())
                .or_insert_with(|| l.to_string());
        }
        let request = build_request(&parameters);
        let response: MarkPriceKlineResponse = self
            .client
            .get(API::Market(Market::MarkPriceKline), Some(request))
            .await?;
        Ok(response.result)
    }

    /// Query for index price klines. Charts are returned in groups based on the requested interval
    /// Covers: Spot / USDT perpetual / USDC contract / Inverse contract
    /// Symbol and Interval is required
    ///  Pass start and end as NaiveDate with the format DDMMYY
    pub async fn get_index_price_klines(
        &self,
        category: Option<Category>,
        symbol: String,
        interval: String,
        start: Option<Cow<'_, str>>,
        end: Option<Cow<'_, str>>,
        limit: Option<u64>,
    ) -> Result<IndexPriceKlineSummary> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        if let Some(category) = category {
            match category {
                Category::Linear | Category::Inverse => {
                    parameters.insert("category".to_owned(), category.as_str().to_owned());
                }
                _ => return Err("Category must be either Linear or Inverse".into()),
            }
        } else {
            parameters.insert("category".to_owned(), Category::Linear.as_str().to_string());
        }
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("interval".into(),  interval.into());
        if let Some(start_str) = start.as_ref().map(|s| s.as_ref()) {
            let start_millis = date_to_milliseconds(start_str);
            parameters
                .entry("start".to_owned())
                .or_insert_with(|| start_millis.to_string());
        }
        if let Some(end_str) = end.as_ref().map(|s| s.as_ref()) {
            let end_millis = date_to_milliseconds(end_str);
            parameters
                .entry("end".to_owned())
                .or_insert_with(|| end_millis.to_string());
        }

        if let Some(l) = limit {
            parameters
                .entry("limit".to_owned())
                .or_insert_with(|| l.to_string());
        }
        let request = build_request(&parameters);
        let response: IndexPriceKlineResponse = self
            .client
            .get(API::Market(Market::IndexPriceKline), Some(request)).await?;
        Ok(response.result)
    }

    /// Query for PremiumIndexPriceKlines
    /// Covers: linear
    /// Symbol and Interval is required
    ///  Pass start and end as NaiveDate with the format DDMMYY
    pub async fn get_premium_index_price_klines(
        &self,
        symbol: String,
        interval: String,
        start: Option<Cow<'_, str>>,
        end: Option<Cow<'_, str>>,
        limit: Option<u64>,
    ) -> Result<PremiumIndexPriceKlineSummary> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("category".to_owned(), Category::Linear.as_str().to_string());
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("interval".into(),  interval.into());
        if let Some(start_str) = start.as_ref().map(|s| s.as_ref()) {
            let start_millis = date_to_milliseconds(start_str);
            parameters
                .entry("start".to_owned())
                .or_insert_with(|| start_millis.to_string());
        }
        if let Some(end_str) = end.as_ref().map(|s| s.as_ref()) {
            let end_millis = date_to_milliseconds(end_str);
            parameters
                .entry("end".to_owned())
                .or_insert_with(|| end_millis.to_string());
        }
        if let Some(l) = limit {
            parameters
                .entry("limit".to_owned())
                .or_insert_with(|| l.to_string());
        }
        let request = build_request(&parameters);
        let response: PremiumIndexPriceKlineResponse = self
            .client
            .get(API::Market(Market::PremiumIndexPriceKline), Some(request)).await?;
        Ok(response.result)
    }

    /// Query for instrument spec of online trading pairs
    /// covers: spot / usdt perpetual / usdc contract / inverse contract
    /// symbol is required
    pub async fn get_instrument_info(
        &self,
        category: Category,
        symbol: Option<Cow<'_, str>>,
        status: bool,
        base_coin: Option<Cow<'_, str>>,
        limit: Option<u64>,
    ) -> Result<Instrument> {
        let mut parameters = BTreeMap::new();
        parameters.insert("category".into(), category.as_str().into());
        if let Some(symbol) = symbol {
            parameters.insert("symbol".into(), symbol.into());
        }
        if status {
            parameters.insert("status".into(), "Trading".into());
        }
        if let Some(base_coin) = base_coin {
            parameters.insert("baseCoin".into(), base_coin.into());
        }
        if let Some(l) = limit {
            parameters.insert("limit".to_string(), l.to_string());
        }
        let request = build_request(&parameters);
        let response: InstrumentsInfoResponse = self
            .client
            .get(API::Market(Market::InstrumentsInfo), Some(request)).await?;
        Ok(response.result.list[0].clone())
    }

    /// Query for orderbook depth data
    ///Covers: Spot / USDT perpetual / USDC contract / Inverse contract / Option
    /// 50 for spot, 200 for contract, 25 for options
    /// Category is required
    /// Symbol is required
    /// Limit is optional spot:[1,  200], linear/inverse: [1,200], option: [1,25]
    pub async fn get_depth(
        &self,
        symbol: &str,
        category: Category,
        limit: Option<u64>,
    ) -> Result<OrderBook>
    {
        let symbol_cow = Cow::Borrowed(symbol).to_uppercase();
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("category".into(), category.as_str().into());
        parameters.insert("symbol".into(), symbol_cow.into());
        if let Some(l) = limit {
            parameters.insert("limit".to_string(), l.to_string());
        }
        let request = build_request(&parameters);
        let response: OrderBookResponse = self
            .client
            .get(API::Market(Market::OrderBook), Some(request)).await?;

        Ok(response.result)
    }
}
