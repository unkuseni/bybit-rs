use crate::errors::{Error, ErrorKind, Result};
use serde::{Deserialize, Serialize};
use serde_json::{from_value, Value};
use std::convert::TryFrom;

#[derive(Deserialize, Clone)]
pub struct Empty {}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ServerTimeResponse {
  pub ret_code: i16,
  pub ret_msg: String,
  pub result: ServerTime,
  pub ret_ext_info: Empty,
  pub time: u64
}

#[derive(Serialize, Deserialize, Clone,Debug)]
#[serde(rename_all = "camelCase")]
pub struct ServerTime {
  pub time_second: u64,
  pub time_nano: u64,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct KlineResponse {
  pub ret_code: i16,
  pub ret_msg: String,
  pub result: KlineSummary,
  pub ret_ext_info: Empty,
  pub time: u64
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct KlineSummary {
  pub symbol: String,
  pub category: String,
  pub list: Vec<Kline>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Kline {
  pub start_time: u64,
  pub open_price: String,
  pub high_price: String,
  pub low_price: String,
  pub close_price: String,
  pub volume: f32,
  #[serde(rename = "turnover")]
  pub quote_asset_volume: f32,
  
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MarkPriceKlineResponse {
  pub ret_code: i16,
  pub ret_msg: String,
  pub result: MarkPriceKlineSummary,
  pub ret_ext_info: Empty,
  pub time: u64
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MarkPriceKlineSummary {
  pub symbol: String,
  pub category: String,
  pub list: Vec<MarkPriceKline>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MarkPriceKline {
  pub start_time: u64,
  pub open_price: String,
  pub high_price: String,
  pub low_price: String,
  pub close_price: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IndexPriceKlineResponse {
  pub ret_code: i16,
  pub ret_msg: String,
  pub result: IndexPriceKlineSummary,
  pub ret_ext_info: Empty,
  pub time: u64
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IndexPriceKlineSummary {
  pub symbol: String,
  pub category: String,
  pub list: Vec<IndexPriceKline>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IndexPriceKline {
  pub start_time: u64,
  pub open_price: String,
  pub high_price: String,
  pub low_price: String,
  pub close_price: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PremiumIndexPriceKlineResponse {
  pub ret_code: i16,
  pub ret_msg: String,
  pub result: MarkPrice,
  pub ret_ext_info: Empty,
  pub time: u64
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PremiumIndexPriceKlineSummary {
  pub symbol: String,
  pub category: String,
  pub list: Vec<IndexPriceKline>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PremiumIndexPriceKline {
  pub start_time: u64,
  pub open_price: String,
  pub high_price: String,
  pub low_price: String,
  pub close_price: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InstrumentsInfoResponse {
  pub ret_code: i16,
  pub ret_msg: String,
  pub result: InstrumentInfo,
  pub ret_ext_info: Empty,
  pub time: u64
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InstrumentInfo {
  pub category: String,
  pub list: Vec<Instrument>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Instrument {
  symbol: String,
  contract_type: String,
  status: String,
  base_coin: String,
  quote_coin: String,
  launch_time: u64,
  delivery_time: u64,
  delivery_fee_rate: String,
  price_scale: String,
  leverage_filter: LeverageFilter,
  price_filter: PriceFilter,
  lot_size_filter: LotSizeFilter,
  unified_margin_trade: bool,
  funding_interval: u64,
  settle_coin: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LeverageFilter {
  min_leverage: String,
  max_leverage: String,
  leverage_step: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PriceFilter {
  min_price: String,
  max_price: String,
  tick_size: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LotSizeFilter {
  min_order_qty: String,
  max_order_qty: String,
  qty_step: String,
  post_only_max_order_qty: bool,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderBookResponse {
  pub ret_code: i16,
  pub ret_msg: String,
  pub result: OrderBook,
  pub ret_ext_info: Empty,
  pub time: u64
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderBook {
  #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "a")]
    pub asks: Vec<Ask>,
    #[serde(rename = "b")]
    pub bids: Vec<Bid>,
    #[serde(rename = "ts")]
    pub timestamp: u64,
    #[serde(rename = "u")]
    pub update_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Ask {
    pub price: String,
    pub qty: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Bid {
    pub price: String,
    pub qty: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TickersResponse {
  pub ret_code: i16,
  pub ret_msg: String,
  pub result: Tickers,
  pub ret_ext_info: Empty,
  pub time: u64
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Tickers {
  pub category: String,
  pub list: Vec<Ticker>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Ticker {
  pub symbol: String,
  pub last_price: String,
  pub index_price: String,
  pub mark_price: String,
  pub prev_price_24h: String,
  #[serde(rename = "price_24h_pcnt")]
  pub daily_change_percentage: String,
  #[serde(rename = "high_price_24h")]
  pub high_24h: String,
  #[serde(rename = "low_price_24h")]
  pub low_24h: String,
  pub prev_price_1h: String,
  pub open_interest: String,
  pub open_interest_value: String,
  pub turnover_24h: String,
  pub volume_24h: String, 
  pub funding_rate: String,
  pub next_funding_time: u64,
  pub predicted_delivery_price: String,
  pub basis_rate : String,
  pub delivery_fee_rate: String,
  pub delivery_time: u64,
  #[serde(rename = "ask1Size")]
  pub ask_size: String,
  #[serde(rename = "bid1Price")]
  pub bid_price: String,
  #[serde(rename = "ask1Price")]
  pub ask_price: String,
  #[serde(rename = "bid1Size")]
  pub bid_size: String,
  pub basis: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FundingRateResponse {
  pub ret_code: i16,
  pub ret_msg: String,
  pub result: FundingRate,
  pub ret_ext_info: Empty,
  pub time: u64
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FundingRateSummary {
  pub category: String,
  pub list: Vec<FundingRate>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FundingRate {
  pub symbol: String,
  pub funding_rate: String,
  pub funding_rate_timestamp: u64,
}


#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RecentTradeListResponse {
  pub ret_code: i16,
  pub ret_msg: String,
  pub result: RecentTradeList,
  pub ret_ext_info: Empty,
  pub time: u64
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RecentTradeList {
  pub category: String,
  pub list: Vec<RecentTrade>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RecentTrade {
  pub exec_id: u64,
  pub symbol: String,
  pub price: String,
  #[serde(rename = "size")]
  pub qty: String,
  pub side: String,
  pub timestamp: u64,
  pub is_block_trade: bool,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OpeninterestResponse {
  pub ret_code: i16,
  pub ret_msg: String,
  pub result: OpenInterest,
  pub ret_ext_info: Empty,
  pub time: u64
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OpenInterestSummary {
  pub symbol: String,
  pub list: Vec<OpenInterest>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OpenInterest {
  pub open_interest: String,
  pub timestamp: u64,
}