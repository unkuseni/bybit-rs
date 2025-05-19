#![allow(unused_imports)]
use crate::errors::BybitError;
use core::f64;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{from_value, Value};
use std::{borrow::Cow, collections::BTreeMap};
use thiserror::Error;

#[derive(Serialize, Default, Deserialize, Clone, Debug)]
pub struct Empty {}

/// ----------------------------------------
///  RESPONSE STRUCTS FOR MARKET REQUESTS
/// ----------------------------------------

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ServerTimeResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    pub result: ServerTime,
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ServerTime {
    #[serde(with = "string_to_u64")]
    pub time_second: u64,
    #[serde(with = "string_to_u64")]
    pub time_nano: u64,
}

#[derive(Clone, Default)]
pub struct KlineRequest<'a> {
    pub category: Option<Category>,
    pub symbol: Cow<'a, str>,
    pub interval: Cow<'a, str>,
    pub start: Option<Cow<'a, str>>,
    pub end: Option<Cow<'a, str>>,
    pub limit: Option<u64>,
}

impl<'a> KlineRequest<'a> {
    pub fn default() -> KlineRequest<'a> {
        KlineRequest::new(None, "BTCUSDT", "", None, None, None)
    }
    pub fn new(
        category: Option<Category>,
        symbol: &'a str,
        interval: &'a str,
        start: Option<&'a str>,
        end: Option<&'a str>,
        limit: Option<u64>,
    ) -> KlineRequest<'a> {
        KlineRequest {
            category,
            symbol: Cow::Borrowed(symbol),
            interval: Cow::Borrowed(interval),
            start: start.map(Cow::Borrowed),
            end: end.map(Cow::Borrowed),
            limit,
        }
    }
}
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct KlineResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    pub result: KlineSummary,
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct KlineSummary {
    pub symbol: String,
    pub category: String,
    pub list: Vec<Kline>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Kline {
    #[serde(with = "string_to_u64")]
    pub start_time: u64,
    pub open_price: String,
    pub high_price: String,
    pub low_price: String,
    pub close_price: String,
    pub volume: String,
    pub quote_asset_volume: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MarkPriceKlineResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    pub result: MarkPriceKlineSummary,
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MarkPriceKlineSummary {
    pub symbol: String,
    pub category: String,
    pub list: Vec<MarkPriceKline>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MarkPriceKline {
    #[serde(with = "string_to_u64")]
    pub start_time: u64,
    pub open_price: String,
    pub high_price: String,
    pub low_price: String,
    pub close_price: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IndexPriceKlineResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    pub result: IndexPriceKlineSummary,
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IndexPriceKlineSummary {
    pub symbol: String,
    pub category: String,
    pub list: Vec<IndexPriceKline>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IndexPriceKline {
    #[serde(with = "string_to_u64")]
    pub start_time: u64,
    pub open_price: String,
    pub high_price: String,
    pub low_price: String,
    pub close_price: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PremiumIndexPriceKlineResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    pub result: PremiumIndexPriceKlineSummary,
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PremiumIndexPriceKlineSummary {
    pub symbol: String,
    pub category: String,
    pub list: Vec<PremiumIndexPriceKline>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PremiumIndexPriceKline {
    #[serde(with = "string_to_u64")]
    pub start_time: u64,
    pub open_price: String,
    pub high_price: String,
    pub low_price: String,
    pub close_price: String,
}

#[derive(Clone, Default)]
pub struct InstrumentRequest<'a> {
    pub category: Category,
    pub symbol: Option<Cow<'a, str>>,
    pub status: Option<bool>,
    pub base_coin: Option<Cow<'a, str>>,
    pub limit: Option<u64>,
}
impl<'a> InstrumentRequest<'a> {
    pub fn default() -> InstrumentRequest<'a> {
        InstrumentRequest::new(Category::Linear, Some("BTCUSDT"), None, None, None)
    }
    pub fn new(
        category: Category,
        symbol: Option<&'a str>,
        status: Option<bool>,
        base_coin: Option<&'a str>,
        limit: Option<u64>,
    ) -> InstrumentRequest<'a> {
        InstrumentRequest {
            category,
            symbol: symbol.map(Cow::Borrowed),
            status,
            base_coin: base_coin.map(Cow::Borrowed),
            limit,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InstrumentInfoResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    pub result: InstrumentInfo,
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum InstrumentInfo {
    Futures(FuturesInstrumentsInfo),
    Spot(SpotInstrumentsInfo),
    Options(OptionsInstrument),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FuturesInstrumentsInfo {
    pub category: String,
    pub list: Vec<FuturesInstrument>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub next_page_cursor: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FuturesInstrument {
    pub symbol: String,
    pub contract_type: String,
    pub status: String,
    pub base_coin: String,
    pub quote_coin: String,
    #[serde(with = "string_to_u64")]
    pub launch_time: u64,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub delivery_time: String,
    pub delivery_fee_rate: String,
    pub price_scale: String,
    pub leverage_filter: LeverageFilter,
    pub price_filter: PriceFilter,
    pub lot_size_filter: LotSizeFilter,
    pub unified_margin_trade: bool,
    pub funding_interval: u64,
    pub settle_coin: String,
    pub copy_trading: String,
    pub upper_funding_rate: String,
    pub lower_funding_rate: String,
    // #[serde(rename = "isPreListing" )]
    // pub is_pre_listing: bool,
    // #[serde(rename = "preListingInfo")]
    // pub pre_listing_info: PreListingInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SpotInstrumentsInfo {
    pub category: String,
    pub list: Vec<SpotInstrument>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub next_page_cursor: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SpotInstrument {
    pub symbol: String,
    pub base_coin: String,
    pub quote_coin: String,
    #[serde(with = "string_to_u64")]
    pub innovation: u64,
    pub status: String,
    pub margin_trading: String,
    #[serde(with = "string_to_u64")]
    pub st_tag: u64,
    pub lot_size_filter: LotSizeFilter,
    pub price_filter: PriceFilter,
    pub risk_parameters: RiskParameters,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OptionsInstrument {
    pub symbol: String,
    pub status: String,
    pub base_coin: String,
    pub quote_coin: String,
    pub settle_coin: String,
    pub option_type: String,
    #[serde(with = "string_to_u64")]
    pub launch_time: u64,
    #[serde(with = "string_to_u64")]
    pub delivery_time: u64,
    pub delivery_fee_rate: String,
    pub price_filter: PriceFilter,
    pub lot_size_filter: LotSizeFilter,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PreListingInfo {
    pub cur_auction_phase: String,
    pub phases: Vec<PreListingPhase>,
    pub auction_fee_info: AuctionFeeInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PreListingPhase {
    pub phase: String,
    #[serde(with = "string_to_u64")]
    pub start_time: u64,
    #[serde(with = "string_to_u64")]
    pub end_time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AuctionFeeInfo {
    pub auction_fee_rate: String,
    pub taker_fee_rate: String,
    pub maker_fee_rate: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RiskParameters {
    pub limit_parameter: String,
    pub market_parameter: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LeverageFilter {
    pub min_leverage: String,
    pub max_leverage: String,
    pub leverage_step: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PriceFilter {
    #[serde(skip_serializing_if = "is_empty_or_none")]
    pub min_price: Option<String>,
    #[serde(skip_serializing_if = "is_empty_or_none")]
    pub max_price: Option<String>,
    #[serde(with = "string_to_float")]
    pub tick_size: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LotSizeFilter {
    #[serde(skip_serializing_if = "is_empty_or_none")]
    pub base_precision: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_precision: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_mkt_order_qty: Option<String>,
    #[serde(with = "string_to_float")]
    pub min_order_qty: f64,
    #[serde(with = "string_to_float")]
    pub max_order_qty: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_order_amt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_order_amt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qty_step: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_only_max_order_qty: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_notional_value: Option<String>,
}

#[derive(Clone, Default)]
pub struct OrderbookRequest<'a> {
    pub symbol: Cow<'a, str>,
    pub category: Category,
    pub limit: Option<u64>,
}

impl<'a> OrderbookRequest<'a> {
    pub fn default() -> OrderbookRequest<'a> {
        OrderbookRequest::new("BTCUSDT", Category::Linear, None)
    }

    pub fn new(symbol: &'a str, category: Category, limit: Option<u64>) -> OrderbookRequest<'a> {
        OrderbookRequest {
            symbol: Cow::Borrowed(symbol),
            category,
            limit,
        }
    }
}
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderBookResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    pub result: OrderBook,
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
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
    #[serde(with = "string_to_float")]
    pub price: f64,
    #[serde(with = "string_to_float")]
    pub qty: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Bid {
    #[serde(with = "string_to_float")]
    pub price: f64,
    #[serde(with = "string_to_float")]
    pub qty: f64,
}

impl Bid {
    pub fn new(price: f64, qty: f64) -> Bid {
        Bid { price, qty }
    }
}
impl Ask {
    pub fn new(price: f64, qty: f64) -> Ask {
        Ask { price, qty }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TickerResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    pub result: TickersInfo,
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TickersInfo {
    pub category: String,
    pub list: Vec<TickerData>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum TickerData {
    Spot(SpotTicker),
    Futures(FuturesTicker),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FuturesTicker {
    pub symbol: String,
    #[serde(with = "string_to_float")]
    pub last_price: f64,
    #[serde(with = "string_to_float")]
    pub index_price: f64,
    #[serde(with = "string_to_float")]
    pub mark_price: f64,
    #[serde(with = "string_to_float")]
    pub prev_price_24h: f64,
    #[serde(rename = "price24hPcnt", with = "string_to_float")]
    pub daily_change_percentage: f64,
    #[serde(rename = "highPrice24h", with = "string_to_float")]
    pub high_24h: f64,
    #[serde(rename = "lowPrice24h", with = "string_to_float")]
    pub low_24h: f64,
    #[serde(with = "string_to_float")]
    pub prev_price_1h: f64,
    #[serde(with = "string_to_float")]
    pub open_interest: f64,
    #[serde(with = "string_to_float")]
    pub open_interest_value: f64,
    #[serde(with = "string_to_float")]
    pub turnover_24h: f64,
    #[serde(with = "string_to_float")]
    pub volume_24h: f64,
    pub funding_rate: String,
    #[serde(with = "string_to_u64")]
    pub next_funding_time: u64,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub predicted_delivery_price: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub basis_rate: String,
    pub delivery_fee_rate: String,
    #[serde(with = "string_to_u64")]
    pub delivery_time: u64,
    #[serde(rename = "ask1Size", with = "string_to_float")]
    pub ask_size: f64,
    #[serde(rename = "bid1Price", with = "string_to_float")]
    pub bid_price: f64,
    #[serde(rename = "ask1Price", with = "string_to_float")]
    pub ask_price: f64,
    #[serde(rename = "bid1Size", with = "string_to_float")]
    pub bid_size: f64,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub basis: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SpotTicker {
    pub symbol: String,
    #[serde(rename = "bid1Price", with = "string_to_float")]
    pub bid_price: f64,
    #[serde(rename = "bid1Size", with = "string_to_float")]
    pub bid_size: f64,
    #[serde(rename = "ask1Price", with = "string_to_float")]
    pub ask_price: f64,
    #[serde(rename = "ask1Size", with = "string_to_float")]
    pub ask_size: f64,
    #[serde(with = "string_to_float")]
    pub last_price: f64,
    #[serde(rename = "prevPrice24h", with = "string_to_float")]
    pub prev_price_24h: f64,
    #[serde(rename = "price24hPcnt", with = "string_to_float")]
    pub daily_change_percentage: f64,
    #[serde(rename = "highPrice24h", with = "string_to_float")]
    pub high_24h: f64,
    #[serde(rename = "lowPrice24h", with = "string_to_float")]
    pub low_24h: f64,
    #[serde(with = "string_to_float")]
    pub turnover_24h: f64,
    #[serde(with = "string_to_float")]
    pub volume_24h: f64,
    #[serde(with = "string_to_float")]
    pub usd_index_price: f64,
}

#[derive(Clone, Default)]
pub struct FundingHistoryRequest<'a> {
    pub category: Category,
    pub symbol: Cow<'a, str>,
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
    pub limit: Option<u64>,
}
impl<'a> FundingHistoryRequest<'a> {
    pub fn default() -> FundingHistoryRequest<'a> {
        FundingHistoryRequest::new(Category::Linear, "BTCUSDT", None, None, None)
    }
    pub fn new(
        category: Category,
        symbol: &'a str,
        start_time: Option<u64>,
        end_time: Option<u64>,
        limit: Option<u64>,
    ) -> FundingHistoryRequest<'a> {
        FundingHistoryRequest {
            category,
            symbol: Cow::Borrowed(symbol),
            start_time,
            end_time,
            limit,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FundingRateResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    pub result: FundingRateSummary,
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FundingRateSummary {
    pub category: String,
    pub list: Vec<FundingRate>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FundingRate {
    pub symbol: String,
    #[serde(with = "string_to_float")]
    pub funding_rate: f64,
    #[serde(with = "string_to_u64")]
    pub funding_rate_timestamp: u64,
}

#[derive(Clone, Default)]
pub struct RecentTradesRequest<'a> {
    pub category: Category,
    pub symbol: Option<Cow<'a, str>>,
    pub base_coin: Option<Cow<'a, str>>,
    pub limit: Option<u64>,
}
impl<'a> RecentTradesRequest<'a> {
    pub fn default() -> RecentTradesRequest<'a> {
        RecentTradesRequest::new(Category::Linear, Some("BTCUSDT"), None, None)
    }
    pub fn new(
        category: Category,
        symbol: Option<&'a str>,
        base_coin: Option<&'a str>,
        limit: Option<u64>,
    ) -> RecentTradesRequest<'a> {
        RecentTradesRequest {
            category,
            symbol: symbol.map(Cow::Borrowed),
            base_coin: base_coin.map(Cow::Borrowed),
            limit,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RecentTradesResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    pub result: RecentTrades,
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RecentTrades {
    pub category: String,
    pub list: Vec<RecentTrade>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RecentTrade {
    pub exec_id: String,
    pub symbol: String,
    #[serde(with = "string_to_float")]
    pub price: f64,
    #[serde(rename = "size", with = "string_to_float")]
    pub qty: f64,
    pub side: String,
    #[serde(rename = "time", with = "string_to_u64")]
    pub timestamp: u64,
    pub is_block_trade: bool,
}

#[derive(Clone, Default)]
pub struct OpenInterestRequest<'a> {
    pub category: Category,
    pub symbol: Cow<'a, str>,
    pub interval: Cow<'a, str>,
    pub start: Option<Cow<'a, str>>,
    pub end: Option<Cow<'a, str>>,
    pub limit: Option<u64>,
}

impl<'a> OpenInterestRequest<'a> {
    pub fn default() -> OpenInterestRequest<'a> {
        OpenInterestRequest::new(Category::Linear, "BTCUSDT", "4h", None, None, None)
    }
    pub fn new(
        category: Category,
        symbol: &'a str,
        interval: &'a str,
        start: Option<&'a str>,
        end: Option<&'a str>,
        limit: Option<u64>,
    ) -> OpenInterestRequest<'a> {
        OpenInterestRequest {
            category,
            symbol: Cow::Borrowed(symbol),
            interval: Cow::Borrowed(interval),
            start: start.map(Cow::Borrowed),
            end: end.map(Cow::Borrowed),
            limit,
        }
    }
}
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OpeninterestResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    pub result: OpenInterestSummary,
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OpenInterestSummary {
    pub symbol: String,
    pub category: String,
    pub list: Vec<OpenInterest>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub next_page_cursor: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OpenInterest {
    #[serde(with = "string_to_float")]
    pub open_interest: f64,
    #[serde(with = "string_to_u64")]
    pub timestamp: u64,
}

#[derive(Clone, Default)]
pub struct HistoricalVolatilityRequest<'a> {
    pub base_coin: Option<Cow<'a, str>>,
    pub period: Option<Cow<'a, str>>,
    pub start: Option<Cow<'a, str>>,
    pub end: Option<Cow<'a, str>>,
}

impl<'a> HistoricalVolatilityRequest<'a> {
    pub fn default() -> HistoricalVolatilityRequest<'a> {
        HistoricalVolatilityRequest::new(Some("BTC"), None, None, None)
    }
    pub fn new(
        base_coin: Option<&'a str>,
        period: Option<&'a str>,
        start: Option<&'a str>,
        end: Option<&'a str>,
    ) -> HistoricalVolatilityRequest<'a> {
        HistoricalVolatilityRequest {
            base_coin: base_coin.map(Cow::Borrowed),
            period: period.map(Cow::Borrowed),
            start: start.map(Cow::Borrowed),
            end: end.map(Cow::Borrowed),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalVolatilityResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    pub category: String,
    pub result: Vec<HistoricalVolatility>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalVolatility {
    pub period: u64,
    #[serde(with = "string_to_float")]
    pub value: f64,
    #[serde(rename = "time", with = "string_to_u64")]
    pub timestamp: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InsuranceResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    pub result: InsuranceSummary,
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InsuranceSummary {
    #[serde(with = "string_to_u64")]
    pub updated_time: u64,
    pub list: Vec<Insurance>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Insurance {
    pub coin: String,
    #[serde(with = "string_to_float")]
    pub balance: f64,
    pub value: String,
}

#[derive(Clone, Default)]
pub struct RiskLimitRequest<'a> {
    pub category: Category,
    pub symbol: Option<Cow<'a, str>>,
}

impl<'a> RiskLimitRequest<'a> {
    pub fn default() -> RiskLimitRequest<'a> {
        RiskLimitRequest::new(Category::Linear, None)
    }
    pub fn new(category: Category, symbol: Option<&'a str>) -> RiskLimitRequest<'a> {
        RiskLimitRequest {
            category,
            symbol: symbol.map(Cow::Borrowed),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RiskLimitResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    pub result: RiskLimitSummary,
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RiskLimitSummary {
    pub category: String,
    pub list: Vec<RiskLimit>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RiskLimit {
    pub id: u64,
    pub symbol: String,
    #[serde(with = "string_to_float")]
    pub risk_limit_value: f64,
    #[serde(with = "string_to_float")]
    pub maintenance_margin: f64,
    #[serde(with = "string_to_float")]
    pub initial_margin: f64,
    pub is_lowest_risk: u8,
    pub max_leverage: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryPriceResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    pub result: DeliveryPriceSummary,
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryPriceSummary {
    pub category: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    pub list: Vec<DeliveryPrice>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryPrice {
    pub symbol: String,
    pub delivery_price: String,
    #[serde(with = "string_to_u64")]
    pub delivery_time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LongShortRatioResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    pub result: LongShortRatioSummary,
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LongShortRatioSummary {
    pub list: Vec<LongShortRatio>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LongShortRatio {
    pub symbol: String,
    #[serde(with = "string_to_float")]
    pub buy_ratio: f64,
    #[serde(with = "string_to_float")]
    pub sell_ratio: f64,
    #[serde(with = "string_to_u64")]
    pub timestamp: u64,
}

/// --------------------------------------------------
///  REQUEST & RESPONSE STRUCTS FOR TRADE
/// --------------------------------------------------
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Category {
    Spot,
    #[default]
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

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub enum Side {
    #[default]
    Buy,
    Sell,
}

impl Side {
    pub fn as_str(&self) -> &str {
        match self {
            Side::Buy => "Buy",
            Side::Sell => "Sell",
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub enum OrderType {
    Limit,
    #[default]
    Market,
}

impl OrderType {
    pub fn as_str(&self) -> &str {
        match self {
            OrderType::Limit => "Limit",
            OrderType::Market => "Market",
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub enum TimeInForce {
    #[default]
    GTC,
    IOC,
    FOK,
    PostOnly,
}

impl TimeInForce {
    pub fn as_str(&self) -> &str {
        match self {
            TimeInForce::GTC => "GTC",
            TimeInForce::IOC => "IOC",
            TimeInForce::FOK => "FOK",
            TimeInForce::PostOnly => "PostOnly",
        }
    }
}
#[derive(Clone, Default, Serialize)]
pub struct OrderRequest<'a> {
    pub category: Category,                 // String
    pub symbol: Cow<'a, str>,               // String
    pub is_leverage: Option<bool>,          // Integer
    pub side: Side,                         // String
    pub order_type: OrderType,              // String
    pub qty: f64,                           // String
    pub market_unit: Option<f64>,           // String
    pub price: Option<f64>,                 // String
    pub trigger_direction: Option<bool>,    // String
    pub order_filter: Option<Cow<'a, str>>, // String
    pub trigger_price: Option<f64>,
    pub trigger_by: Option<Cow<'a, str>>,    // String
    pub order_iv: Option<f64>,               // String
    pub time_in_force: Option<Cow<'a, str>>, // String
    pub position_idx: Option<u8>,
    pub order_link_id: Option<Cow<'a, str>>,
    pub take_profit: Option<f64>,
    pub stop_loss: Option<f64>,
    pub tp_trigger_by: Option<Cow<'a, str>>,
    pub sl_trigger_by: Option<Cow<'a, str>>,
    pub reduce_only: Option<bool>,
    pub close_on_trigger: Option<bool>,
    pub smp_type: Option<Cow<'a, str>>,
    pub mmp: Option<bool>,
    pub tpsl_mode: Option<Cow<'a, str>>,
    pub tp_limit_price: Option<f64>,
    pub sl_limit_price: Option<f64>,
    pub tp_order_type: Option<Cow<'a, str>>,
    pub sl_order_type: Option<Cow<'a, str>>,
}

impl<'a> OrderRequest<'a> {
    pub fn default() -> Self {
        Self {
            category: Category::Linear,
            symbol: Cow::Borrowed("BTCUSDT"),
            is_leverage: None,
            side: Side::default(),
            order_type: OrderType::Market,
            qty: 0.00,
            market_unit: None,
            price: None,
            trigger_direction: None,
            order_filter: None,
            trigger_price: None,
            trigger_by: None,
            order_iv: None,
            time_in_force: None,
            position_idx: None,
            order_link_id: None,
            take_profit: None,
            stop_loss: None,
            tp_trigger_by: None,
            sl_trigger_by: None,
            reduce_only: None,
            close_on_trigger: None,
            smp_type: None,
            mmp: None,
            tpsl_mode: None,
            tp_limit_price: None,
            sl_limit_price: None,
            tp_order_type: None,
            sl_order_type: None,
        }
    }
    pub fn custom(
        category: Category,
        symbol: &'a str,
        leverage: Option<bool>,
        side: Side,
        order_type: OrderType,
        qty: f64,
        market_unit: Option<f64>,
        price: Option<f64>,
        trigger_direction: Option<bool>,
        order_filter: Option<&'a str>,
        trigger_price: Option<f64>,
        trigger_by: Option<&'a str>,
        order_iv: Option<f64>,
        time_in_force: Option<&'a str>,
        position_idx: Option<u8>,
        order_link_id: Option<&'a str>,
        take_profit: Option<f64>,
        stop_loss: Option<f64>,
        tp_trigger_by: Option<&'a str>,
        sl_trigger_by: Option<&'a str>,
        reduce_only: Option<bool>,
        close_on_trigger: Option<bool>,
        smp_type: Option<&'a str>,
        mmp: Option<bool>,
        tpsl_mode: Option<&'a str>,
        tp_limit_price: Option<f64>,
        sl_limit_price: Option<f64>,
        tp_order_type: Option<&'a str>,
        sl_order_type: Option<&'a str>,
    ) -> Self {
        Self {
            category,
            symbol: Cow::Borrowed(symbol),
            is_leverage: leverage,
            side,
            order_type,
            qty,
            market_unit,
            price,
            trigger_direction,
            order_filter: order_filter.map(Cow::Borrowed),
            trigger_price,
            trigger_by: trigger_by.map(Cow::Borrowed),
            order_iv,
            time_in_force: time_in_force.map(Cow::Borrowed),
            position_idx,
            order_link_id: order_link_id.map(Cow::Borrowed),
            take_profit,
            stop_loss,
            tp_trigger_by: tp_trigger_by.map(Cow::Borrowed),
            sl_trigger_by: sl_trigger_by.map(Cow::Borrowed),
            reduce_only,
            close_on_trigger,
            smp_type: smp_type.map(Cow::Borrowed),
            mmp,
            tpsl_mode: tpsl_mode.map(Cow::Borrowed),
            tp_limit_price,
            sl_limit_price,
            tp_order_type: tp_order_type.map(Cow::Borrowed),
            sl_order_type: sl_order_type.map(Cow::Borrowed),
        }
    }
    pub fn spot_limit_with_market_tpsl(
        symbol: &'a str,
        side: Side,
        qty: f64,
        price: f64,
        tp: f64,
        sl: f64,
    ) -> Self {
        Self {
            category: Category::Spot,
            symbol: Cow::Borrowed(symbol),
            side,
            order_type: OrderType::Limit,
            qty,
            price: Some(price),
            time_in_force: Some(Cow::Borrowed(TimeInForce::PostOnly.as_str())),
            take_profit: Some(tp),
            stop_loss: Some(sl),
            tp_order_type: Some(Cow::Borrowed("Market")),
            sl_order_type: Some(Cow::Borrowed("Market")),
            ..Self::default()
        }
    }
    pub fn spot_limit_with_limit_tpsl(
        symbol: &'a str,
        side: Side,
        qty: f64,
        price: f64,
        tp: f64,
        sl: f64,
    ) -> Self {
        Self {
            category: Category::Spot,
            symbol: Cow::Borrowed(symbol),
            side,
            order_type: OrderType::Limit,
            qty,
            price: Some(price),
            time_in_force: Some(Cow::Borrowed(TimeInForce::PostOnly.as_str())),
            take_profit: Some(tp),
            stop_loss: Some(sl),
            tp_limit_price: Some(tp),
            sl_limit_price: Some(sl),
            tp_order_type: Some(Cow::Borrowed("Limit")),
            sl_order_type: Some(Cow::Borrowed("Limit")),
            ..Self::default()
        }
    }
    pub fn spot_postonly(symbol: &'a str, side: Side, qty: f64, price: f64) -> Self {
        Self {
            category: Category::Spot,
            symbol: Cow::Borrowed(symbol),
            side,
            order_type: OrderType::Limit,
            qty,
            price: Some(price),
            time_in_force: Some(Cow::Borrowed(TimeInForce::PostOnly.as_str())),
            ..Self::default()
        }
    }
    pub fn spot_tpsl(
        symbol: &'a str,
        side: Side,
        price: f64,
        qty: f64,
        order_link_id: Option<&'a str>,
    ) -> Self {
        Self {
            category: Category::Spot,
            symbol: Cow::Borrowed(symbol),
            side,
            order_type: OrderType::Limit,
            qty,
            price: Some(price),
            time_in_force: Some(Cow::Borrowed(TimeInForce::GTC.as_str())),
            order_link_id: order_link_id.map(Cow::Borrowed),
            order_filter: Some(Cow::Borrowed("tpslOrder")),
            ..Self::default()
        }
    }
    pub fn spot_margin(symbol: &'a str, side: Side, qty: f64, price: f64) -> Self {
        Self {
            category: Category::Spot,
            symbol: Cow::Borrowed(symbol),
            side,
            order_type: OrderType::Market,
            qty,
            price: Some(price),
            time_in_force: Some(Cow::Borrowed(TimeInForce::PostOnly.as_str())),
            is_leverage: Some(true),
            ..Self::default()
        }
    }

    pub fn spot_market(symbol: &'a str, side: Side, qty: f64) -> Self {
        Self {
            category: Category::Spot,
            symbol: Cow::Borrowed(symbol),
            side,
            order_type: OrderType::Market,
            qty,
            time_in_force: Some(Cow::Borrowed(TimeInForce::IOC.as_str())),
            ..Self::default()
        }
    }

    pub fn futures_limit_with_market_tpsl(
        symbol: &'a str,
        side: Side,
        qty: f64,
        price: f64,
        tp: f64,
        sl: f64,
    ) -> Self {
        Self {
            category: Category::Linear,
            symbol: Cow::Borrowed(symbol),
            side,
            order_type: OrderType::Limit,
            qty,
            price: Some(price),
            time_in_force: Some(Cow::Borrowed(TimeInForce::PostOnly.as_str())),
            reduce_only: Some(false),
            take_profit: Some(tp),
            stop_loss: Some(sl),
            tpsl_mode: Some(Cow::Borrowed("Full")),
            tp_order_type: Some(Cow::Borrowed("Market")),
            sl_order_type: Some(Cow::Borrowed("Market")),
            ..Self::default()
        }
    }

    pub fn futures_limit_with_limit_tpsl(
        symbol: &'a str,
        side: Side,
        qty: f64,
        price: f64,
        tp: f64,
        sl: f64,
    ) -> Self {
        Self {
            category: Category::Linear,
            symbol: Cow::Borrowed(symbol),
            side,
            order_type: OrderType::Limit,
            qty,
            price: Some(price),
            time_in_force: Some(Cow::Borrowed(TimeInForce::PostOnly.as_str())),
            reduce_only: Some(false),
            take_profit: Some(tp),
            stop_loss: Some(sl),
            tpsl_mode: Some(Cow::Borrowed("Partial")),
            tp_order_type: Some(Cow::Borrowed("Limit")),
            sl_order_type: Some(Cow::Borrowed("Limit")),
            tp_limit_price: Some(tp),
            sl_limit_price: Some(sl),
            ..Self::default()
        }
    }

    pub fn futures_market(symbol: &'a str, side: Side, qty: f64) -> Self {
        Self {
            category: Category::Linear,
            symbol: Cow::Borrowed(symbol),
            side,
            order_type: OrderType::Market,
            qty,
            time_in_force: Some(Cow::Borrowed(TimeInForce::IOC.as_str())),
            reduce_only: Some(false),
            ..Self::default()
        }
    }

    pub fn futures_close_limit(
        symbol: &'a str,
        side: Side,
        qty: f64,
        price: f64,
        order_link_id: &'a str,
    ) -> Self {
        Self {
            category: Category::Linear,
            symbol: Cow::Borrowed(symbol),
            side,
            order_type: OrderType::Limit,
            qty,
            price: Some(price),
            time_in_force: Some(Cow::Borrowed(TimeInForce::GTC.as_str())),
            order_link_id: Some(Cow::Borrowed(order_link_id)),
            reduce_only: Some(true),
            ..Self::default()
        }
    }

    pub fn futures_market_close(symbol: &'a str, side: Side, qty: f64) -> Self {
        Self {
            category: Category::Linear,
            symbol: Cow::Borrowed(symbol),
            side,
            order_type: OrderType::Market,
            qty,
            time_in_force: Some(Cow::Borrowed(TimeInForce::IOC.as_str())),
            reduce_only: Some(true),
            ..Self::default()
        }
    }
}
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AmendOrderResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    pub result: OrderStatus,
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Clone, Default, Serialize)]
pub struct AmendOrderRequest<'a> {
    pub category: Category,   // String
    pub symbol: Cow<'a, str>, // String
    pub order_id: Option<Cow<'a, str>>,
    pub order_link_id: Option<Cow<'a, str>>,
    pub order_iv: Option<f64>, // String
    pub trigger_price: Option<f64>,
    pub qty: f64,           // String
    pub price: Option<f64>, // String
    pub tpsl_mode: Option<Cow<'a, str>>,
    pub take_profit: Option<f64>,
    pub stop_loss: Option<f64>,
    pub tp_trigger_by: Option<Cow<'a, str>>,
    pub sl_trigger_by: Option<Cow<'a, str>>,
    pub trigger_by: Option<Cow<'a, str>>, // String
    pub tp_limit_price: Option<f64>,
    pub sl_limit_price: Option<f64>,
}

impl<'a> AmendOrderRequest<'a> {
    pub fn default() -> Self {
        Self {
            category: Category::Linear,
            symbol: Cow::Borrowed("BTCUSDT"),
            order_id: None,
            order_link_id: None,
            order_iv: None,
            trigger_price: None,
            qty: 0.00,
            price: None,
            tpsl_mode: None,
            take_profit: None,
            stop_loss: None,
            tp_trigger_by: None,
            sl_trigger_by: None,
            trigger_by: None,
            tp_limit_price: None,
            sl_limit_price: None,
        }
    }
    pub fn custom(
        category: Category,
        symbol: &'a str,
        order_id: Option<&'a str>,
        order_link_id: Option<&'a str>,
        order_iv: Option<f64>,
        trigger_price: Option<f64>,
        qty: f64,
        price: Option<f64>,
        tpsl_mode: Option<&'a str>,
        take_profit: Option<f64>,
        stop_loss: Option<f64>,
        tp_trigger_by: Option<&'a str>,
        sl_trigger_by: Option<&'a str>,
        trigger_by: Option<&'a str>,
        tp_limit_price: Option<f64>,
        sl_limit_price: Option<f64>,
    ) -> Self {
        Self {
            category,
            symbol: Cow::Borrowed(symbol),
            order_id: order_id.map(Cow::Borrowed),
            order_link_id: order_link_id.map(Cow::Borrowed),
            order_iv,
            trigger_price,
            qty,
            price,
            tpsl_mode: tpsl_mode.map(Cow::Borrowed),
            take_profit,
            stop_loss,
            tp_trigger_by: tp_trigger_by.map(Cow::Borrowed),
            sl_trigger_by: sl_trigger_by.map(Cow::Borrowed),
            trigger_by: trigger_by.map(Cow::Borrowed),
            tp_limit_price,
            sl_limit_price,
        }
    }
}

#[derive(Clone, Serialize)]
pub struct CancelOrderRequest<'a> {
    pub category: Category,
    pub symbol: Cow<'a, str>,
    pub order_id: Option<Cow<'a, str>>,
    pub order_link_id: Option<Cow<'a, str>>,
    pub order_filter: Option<Cow<'a, str>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    pub result: OrderStatus,
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Clone, Default)]
pub struct OpenOrdersRequest<'a> {
    pub category: Category,
    pub symbol: Cow<'a, str>,
    pub base_coin: Option<Cow<'a, str>>,
    pub settle_coin: Option<Cow<'a, str>>,
    pub order_id: Option<Cow<'a, str>>,
    pub order_link_id: Option<Cow<'a, str>>,
    pub open_only: Option<usize>,
    pub order_filter: Option<Cow<'a, str>>,
    pub limit: Option<usize>,
}

impl<'a> OpenOrdersRequest<'a> {
    pub fn default() -> Self {
        Self {
            category: Category::Linear,
            symbol: Cow::Borrowed("BTCUSDT"),
            base_coin: None,
            settle_coin: None,
            order_id: None,
            order_link_id: None,
            open_only: None,
            order_filter: None,
            limit: None,
        }
    }

    pub fn custom(
        category: Category,
        symbol: &'a str,
        base_coin: Option<&'a str>,
        settle_coin: Option<&'a str>,
        order_id: Option<&'a str>,
        order_link_id: Option<&'a str>,
        open_only: usize,
        order_filter: Option<&'a str>,
        limit: Option<usize>,
    ) -> Self {
        Self {
            category,
            symbol: Cow::Borrowed(symbol),
            base_coin: base_coin.map(Cow::Borrowed),
            settle_coin: settle_coin.map(Cow::Borrowed),
            order_id: order_id.map(Cow::Borrowed),
            order_link_id: order_link_id.map(Cow::Borrowed),
            open_only: match open_only {
                0..=2 => Some(open_only),
                _ => None,
            },
            order_filter: order_filter.map(Cow::Borrowed),
            limit,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OpenOrdersResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    pub result: OrderHistory,
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderStatus {
    pub order_id: String,
    pub order_link_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    pub result: OrderStatus,
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Clone, Default)]
pub struct OrderHistoryRequest<'a> {
    pub category: Category,
    pub symbol: Option<Cow<'a, str>>,
    pub base_coin: Option<Cow<'a, str>>,
    pub settle_coin: Option<Cow<'a, str>>,
    pub order_id: Option<Cow<'a, str>>,
    pub order_link_id: Option<Cow<'a, str>>,
    pub order_filter: Option<Cow<'a, str>>,
    pub order_status: Option<Cow<'a, str>>,
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
    pub limit: Option<u64>,
}

impl<'a> OrderHistoryRequest<'a> {
    pub fn default() -> Self {
        Self {
            category: Category::Linear,
            symbol: None,
            base_coin: None,
            settle_coin: None,
            order_id: None,
            order_link_id: None,
            order_filter: None,
            order_status: None,
            start_time: None,
            end_time: None,
            limit: None,
        }
    }
    pub fn new(
        category: Category,
        symbol: Option<&'a str>,
        base_coin: Option<&'a str>,
        settle_coin: Option<&'a str>,
        order_id: Option<&'a str>,
        order_link_id: Option<&'a str>,
        order_filter: Option<&'a str>,
        order_status: Option<&'a str>,
        start_time: Option<u64>,
        end_time: Option<u64>,
        limit: Option<u64>,
    ) -> Self {
        Self {
            category,
            symbol: symbol.map(Cow::Borrowed),
            base_coin: base_coin.map(Cow::Borrowed),
            settle_coin: settle_coin.map(Cow::Borrowed),
            order_id: order_id.map(Cow::Borrowed),
            order_link_id: order_link_id.map(Cow::Borrowed),
            order_filter: order_filter.map(Cow::Borrowed),
            order_status: order_status.map(Cow::Borrowed),
            start_time,
            end_time,
            limit,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderHistoryResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    pub result: OrderHistory,
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderHistory {
    pub category: String,
    pub list: Vec<Order>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub next_page_cursor: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub order_id: String,
    pub order_link_id: String,

    #[serde(
        deserialize_with = "empty_string_as_none",
        skip_serializing_if = "is_empty_or_none"
    )]
    pub block_trade_id: Option<String>,

    pub symbol: String,

    #[serde(with = "string_to_float")]
    pub price: f64,

    #[serde(with = "string_to_float")]
    pub qty: f64,

    pub side: Side,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub is_leverage: String,

    pub position_idx: i32,
    pub order_status: String,
    pub cancel_type: String,
    pub reject_reason: String,

    #[serde(with = "string_to_float_optional")]
    pub avg_price: Option<f64>,

    #[serde(with = "string_to_float")]
    pub leaves_qty: f64,

    #[serde(with = "string_to_float")]
    pub leaves_value: f64,

    #[serde(with = "string_to_float")]
    pub cum_exec_qty: f64,

    #[serde(with = "string_to_float")]
    pub cum_exec_value: f64,

    #[serde(with = "string_to_float")]
    pub cum_exec_fee: f64,

    pub time_in_force: String,
    pub order_type: OrderType,

    #[serde(
        deserialize_with = "empty_string_as_none",
        skip_serializing_if = "is_empty_or_none"
    )]
    pub stop_order_type: Option<String>,

    #[serde(
        deserialize_with = "empty_string_as_none",
        skip_serializing_if = "is_empty_or_none"
    )]
    pub order_iv: Option<String>,

    #[serde(with = "string_to_float_optional")]
    pub trigger_price: Option<f64>,

    #[serde(with = "string_to_float_optional")]
    pub take_profit: Option<f64>,

    #[serde(with = "string_to_float_optional")]
    pub stop_loss: Option<f64>,

    pub tp_trigger_by: String,

    pub sl_trigger_by: String,

    pub trigger_direction: i32,

    #[serde(
        deserialize_with = "empty_string_as_none",
        skip_serializing_if = "is_empty_or_none"
    )]
    pub trigger_by: Option<String>,

    #[serde(with = "string_to_float_optional")]
    pub last_price_on_created: Option<f64>,

    pub reduce_only: bool,

    pub close_on_trigger: bool,

    pub smp_type: String,

    pub smp_group: i32,

    #[serde(
        deserialize_with = "empty_string_as_none",
        skip_serializing_if = "is_empty_or_none"
    )]
    pub smp_order_id: Option<String>,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub tpsl_mode: String,

    #[serde(with = "string_to_float_optional")]
    pub tp_limit_price: Option<f64>,

    #[serde(with = "string_to_float_optional")]
    pub sl_limit_price: Option<f64>,

    #[serde(
        deserialize_with = "empty_string_as_none",
        skip_serializing_if = "is_empty_or_none"
    )]
    pub place_type: Option<String>,

    #[serde(with = "string_to_u64")]
    pub created_time: u64,

    #[serde(with = "string_to_u64")]
    pub updated_time: u64,
}

// Custom function to check if Option<String> is None or Some("")
fn is_empty_or_none(opt: &Option<String>) -> bool {
    opt.as_ref().is_none_or(|s| s.is_empty())
}

fn empty_string_as_none<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(ref s) if s.trim().is_empty() => Ok(None),
        Some(s) => {
            let kind = T::deserialize(serde_json::Value::String(s))
                .map(Some)
                .map_err(serde::de::Error::custom)?;
            Ok(kind)
        }
        None => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_orders() {
        let json = r#"
        {
                "orderId": "fd4300ae-7847-404e-b947-b46980a4d140",
                "orderLinkId": "test-000005",
                "blockTradeId": "",
                "symbol": "ETHUSDT",
                "price": "1600.00",
                "qty": "0.10",
                "side": "Buy",
                "isLeverage": "",
                "positionIdx": 1,
                "orderStatus": "New",
                "cancelType": "UNKNOWN",
                "rejectReason": "EC_NoError",
                "avgPrice": "0",
                "leavesQty": "0.10",
                "leavesValue": "160",
                "cumExecQty": "0.00",
                "cumExecValue": "0",
                "cumExecFee": "0",
                "timeInForce": "GTC",
                "orderType": "Limit",
                "stopOrderType": "UNKNOWN",
                "orderIv": "",
                "triggerPrice": "0.00",
                "takeProfit": "2500.00",
                "stopLoss": "1500.00",
                "tpTriggerBy": "LastPrice",
                "slTriggerBy": "LastPrice",
                "triggerDirection": 0,
                "triggerBy": "UNKNOWN",
                "lastPriceOnCreated": "",
                "reduceOnly": false,
                "closeOnTrigger": false,
                "smpType": "None",
                "smpGroup": 0,
                "smpOrderId": "",
                "tpslMode": "Full",
                "tpLimitPrice": "",
                "slLimitPrice": "",
                "placeType": "",
                "createdTime": "1684738540559",
                "updatedTime": "1684738540561"
            }
        "#;
        let order: Order = serde_json::from_str(json).unwrap();
        assert_eq!(order.order_id, "fd4300ae-7847-404e-b947-b46980a4d140");
    }
}

#[derive(Clone, Default)]
pub struct CancelallRequest<'a> {
    pub category: Category,
    pub symbol: &'a str,
    pub base_coin: Option<&'a str>,
    pub settle_coin: Option<&'a str>,
    pub order_filter: Option<&'a str>,
    pub stop_order_type: Option<&'a str>,
}

impl<'a> CancelallRequest<'a> {
    pub fn default() -> Self {
        Self {
            category: Category::Linear,
            symbol: "BTCUSDT",
            base_coin: None,
            settle_coin: None,
            order_filter: None,
            stop_order_type: None,
        }
    }
    pub fn new(
        category: Category,
        symbol: &'a str,
        base_coin: Option<&'a str>,
        settle_coin: Option<&'a str>,
        order_filter: Option<&'a str>,
        stop_order_type: Option<&'a str>,
    ) -> Self {
        Self {
            category,
            symbol,
            base_coin,
            settle_coin,
            order_filter,
            stop_order_type,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CancelallResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    pub result: CancelledList,
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CancelledList {
    pub list: Vec<OrderStatus>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TradeHistoryResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    pub result: TradeHistorySummary,
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TradeHistorySummary {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub next_page_cursor: String,
    pub category: String,
    pub list: Vec<TradeHistory>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TradeHistory {
    pub symbol: String,
    pub order_type: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub underlying_price: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub order_link_id: String,
    pub side: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub index_price: String,
    pub order_id: String,
    pub stop_order_type: String,
    pub leaves_qty: String,
    pub exec_time: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub fee_currency: String,
    pub is_maker: bool,
    #[serde(with = "string_to_float")]
    pub exec_fee: f64,
    pub fee_rate: String,
    pub exec_id: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub trade_iv: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub block_trade_id: String,
    #[serde(with = "string_to_float")]
    pub mark_price: f64,
    #[serde(with = "string_to_float")]
    pub exec_price: f64,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub mark_iv: String,
    #[serde(with = "string_to_float")]
    pub order_qty: f64,
    #[serde(with = "string_to_float")]
    pub order_price: f64,
    #[serde(with = "string_to_float")]
    pub exec_value: f64,
    pub exec_type: String,
    #[serde(with = "string_to_float")]
    pub exec_qty: f64,
    #[serde(
        rename = "closedSize",
        default,
        skip_serializing_if = "String::is_empty"
    )]
    pub closed_size: String,
    pub seq: u64,
}

#[derive(Clone, Default)]
pub struct TradeHistoryRequest<'a> {
    pub category: Category,
    pub symbol: Option<Cow<'a, str>>,
    pub order_id: Option<Cow<'a, str>>,
    pub order_link_id: Option<Cow<'a, str>>,
    pub base_coin: Option<Cow<'a, str>>,
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
    pub exec_type: Option<Cow<'a, str>>,
    pub limit: Option<u64>,
}

impl<'a> TradeHistoryRequest<'a> {
    pub fn default() -> TradeHistoryRequest<'a> {
        TradeHistoryRequest::new(
            Category::Linear,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
    }
    pub fn new(
        category: Category,
        symbol: Option<&'a str>,
        order_id: Option<&'a str>,
        order_link_id: Option<&'a str>,
        base_coin: Option<&'a str>,
        start_time: Option<u64>,
        end_time: Option<u64>,
        exec_type: Option<&'a str>,
        limit: Option<u64>,
    ) -> TradeHistoryRequest<'a> {
        TradeHistoryRequest {
            category,
            symbol: symbol.map(Cow::Borrowed),
            order_id: order_id.map(Cow::Borrowed),
            order_link_id: order_link_id.map(Cow::Borrowed),
            base_coin: base_coin.map(Cow::Borrowed),
            start_time,
            end_time,
            exec_type: exec_type.map(Cow::Borrowed),
            limit,
        }
    }
}

#[derive(Clone, Default)]
pub struct BatchPlaceRequest<'a> {
    pub category: Category,
    pub requests: Vec<OrderRequest<'a>>,
}
impl<'a> BatchPlaceRequest<'a> {
    pub fn new(category: Category, requests: Vec<OrderRequest<'a>>) -> BatchPlaceRequest<'a> {
        BatchPlaceRequest { category, requests }
    }
}
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BatchPlaceResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    pub result: BatchedOrderList,
    pub ret_ext_info: OrderConfirmationList,
    pub time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BatchedOrderList {
    pub list: Vec<BatchedOrder>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BatchedOrder {
    pub category: String,
    pub symbol: String,
    pub order_id: String,
    pub order_link_id: String,
    pub create_at: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderConfirmationList {
    pub list: Vec<OrderConfirmation>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderConfirmation {
    pub code: i16,
    pub msg: String,
}

#[derive(Clone, Default)]
pub struct BatchAmendRequest<'a> {
    pub category: Category,
    pub requests: Vec<AmendOrderRequest<'a>>,
}

impl<'a> BatchAmendRequest<'a> {
    pub fn new(category: Category, requests: Vec<AmendOrderRequest<'a>>) -> BatchAmendRequest<'a> {
        BatchAmendRequest { category, requests }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BatchAmendResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    pub result: AmendedOrderList,
    pub ret_ext_info: OrderConfirmationList,
    pub time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AmendedOrderList {
    pub list: Vec<AmendedOrder>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AmendedOrder {
    pub category: String,
    pub symbol: String,
    pub order_id: String,
    pub order_link_id: String,
}

#[derive(Clone, Default)]
pub struct BatchCancelRequest<'a> {
    pub category: Category,
    pub requests: Vec<CancelOrderRequest<'a>>,
}

impl<'a> BatchCancelRequest<'a> {
    pub fn new(
        category: Category,
        requests: Vec<CancelOrderRequest<'a>>,
    ) -> BatchCancelRequest<'a> {
        BatchCancelRequest { category, requests }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BatchCancelResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    pub result: CanceledOrderList,
    pub ret_ext_info: OrderConfirmationList,
    pub time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CanceledOrderList {
    pub list: Vec<CanceledOrder>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CanceledOrder {
    pub category: String,
    pub symbol: String,
    pub order_id: String,
    pub order_link_id: String,
}

#[derive(Clone)]
pub enum RequestType<'a> {
    Create(BatchPlaceRequest<'a>),
    Amend(BatchAmendRequest<'a>),
    Cancel(BatchCancelRequest<'a>),
}

// ----------------------------------------------------------
// POSITION STRUCTS SECTION
// ------------------------------------------------------

#[derive(Clone, Default)]
pub struct PositionRequest<'a> {
    pub category: Category,
    pub symbol: Option<Cow<'a, str>>,
    pub base_coin: Option<Cow<'a, str>>,
    pub settle_coin: Option<Cow<'a, str>>,
    pub limit: Option<usize>,
}

impl<'a> PositionRequest<'a> {
    pub fn default() -> Self {
        Self::new(Category::Linear, None, None, None, None)
    }
    pub fn new(
        category: Category,
        symbol: Option<&'a str>,
        base_coin: Option<&'a str>,
        settle_coin: Option<&'a str>,
        limit: Option<usize>,
    ) -> Self {
        Self {
            category,
            symbol: symbol.map(Cow::Borrowed),
            base_coin: base_coin.map(Cow::Borrowed),
            settle_coin: settle_coin.map(Cow::Borrowed),
            limit,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InfoResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    pub result: InfoResult,
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InfoResult {
    pub list: Vec<PositionInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    pub category: String,
}

mod string_to_float_default_zero {
    use serde::{Deserialize, Deserializer, Serializer};
    use std::str::FromStr;

    // Serialization: Convert f64 to string
    pub fn serialize<S>(value: &f64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&value.to_string())
    }

    // Deserialization: Parse string to f64, return 0.0 for empty string
    pub fn deserialize<'de, D>(deserializer: D) -> Result<f64, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s.is_empty() {
            Ok(0.0) // Return 0.0 for empty string
        } else {
            f64::from_str(&s).map_err(serde::de::Error::custom)
        }
    }
}

mod string_to_float_optional {
    use serde::{Deserialize, Deserializer, Serializer};
    use std::str::FromStr;

    // Serialization: Convert Option<f64> to string
    pub fn serialize<S>(value: &Option<f64>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(v) => serializer.serialize_str(&v.to_string()),
            None => serializer.serialize_str(""),
        }
    }

    // Deserialization: Parse string to Option<f64>, return None for empty string
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s.is_empty() {
            Ok(None) // Return None for empty string
        } else {
            f64::from_str(&s)
                .map(Some)
                .map_err(serde::de::Error::custom)
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PositionInfo {
    pub position_idx: i32,

    pub risk_id: i32,

    #[serde(with = "string_to_float")]
    pub risk_limit_value: f64,

    pub symbol: String,

    #[serde(deserialize_with = "empty_string_as_none")]
    pub side: Option<Side>,

    #[serde(with = "string_to_float")]
    pub size: f64,

    #[serde(with = "string_to_float")]
    pub avg_price: f64,

    #[serde(with = "string_to_float_optional")]
    pub position_value: Option<f64>,

    pub trade_mode: i32,

    pub position_status: String,

    pub auto_add_margin: i32,

    pub adl_rank_indicator: i32,

    #[serde(with = "string_to_float")]
    pub leverage: f64,

    #[serde(with = "string_to_float")]
    pub position_balance: f64,

    #[serde(with = "string_to_float")]
    pub mark_price: f64,

    #[serde(with = "string_to_float_optional")]
    pub liq_price: Option<f64>,

    #[serde(with = "string_to_float_optional")]
    pub bust_price: Option<f64>,

    #[serde(rename = "positionMM", with = "string_to_float")]
    pub position_mm: f64,

    #[serde(rename = "positionIM", with = "string_to_float")]
    pub position_im: f64,

    pub tpsl_mode: String,

    #[serde(with = "string_to_float_optional")]
    pub take_profit: Option<f64>,

    #[serde(with = "string_to_float_optional")]
    pub stop_loss: Option<f64>,

    #[serde(with = "string_to_float")]
    pub trailing_stop: f64,

    #[serde(with = "string_to_float_optional")]
    pub unrealised_pnl: Option<f64>,

    #[serde(with = "string_to_float")]
    pub cum_realised_pnl: f64,

    pub seq: i64,

    pub is_reduce_only: bool,

    #[serde(with = "string_to_u64_optional")]
    pub mmr_sys_updated_time: Option<u64>,

    #[serde(with = "string_to_u64_optional")]
    pub leverage_sys_updated_time: Option<u64>,

    #[serde(with = "string_to_u64")]
    pub created_time: u64,

    #[serde(with = "string_to_u64")]
    pub updated_time: u64,
}

#[cfg(test)]
mod test_decode_position_info {
    use super::*;

    #[test]
    fn test_deserialize() {
        let json = r#"
             {
                "positionIdx": 0,
                "riskId": 1,
                "riskLimitValue": "150",
                "symbol": "BTCUSD",
                "side": "Sell",
                "size": "300",
                "avgPrice": "27464.50441675",
                "positionValue": "0.01092319",
                "tradeMode": 0,
                "positionStatus": "Normal",
                "autoAddMargin": 1,
                "adlRankIndicator": 2,
                "leverage": "10",
                "positionBalance": "0.00139186",
                "markPrice": "28224.50",
                "liqPrice": "",
                "bustPrice": "999999.00",
                "positionMM": "0.0000015",
                "positionIM": "0.00010923",
                "tpslMode": "Full",
                "takeProfit": "0.00",
                "stopLoss": "0.00",
                "trailingStop": "0.00",
                "unrealisedPnl": "-0.00029413",
                "curRealisedPnl": "0.00013123",
                "cumRealisedPnl": "-0.00096902",
                "seq": 5723621632,
                "isReduceOnly": false,
                "mmrSysUpdatedTime": "1676538056444",
                "leverageSysUpdatedTime": "1676538056333",
                "sessionAvgPrice": "",
                "createdTime": "1676538056258",
                "updatedTime": "1697673600012"
            }
        "#;
        let result = serde_json::from_str::<PositionInfo>(json);
        let result = result.unwrap();
        assert_eq!(result.cum_realised_pnl, Some(-0.00096902));
        assert_eq!(result.mmr_sys_updated_time, Some(1676538056444));
        assert_eq!(result.leverage_sys_updated_time, Some(1676538056333));
        assert_eq!(result.created_time, 1676538056258);
        assert_eq!(result.updated_time, 1697673600012);
    }
}

#[derive(Clone, Default)]
pub struct LeverageRequest<'a> {
    pub category: Category,
    pub symbol: Cow<'a, str>,
    pub leverage: i8,
}

impl<'a> LeverageRequest<'a> {
    pub fn new(category: Category, symbol: &'a str, leverage: i8) -> Self {
        Self {
            category,
            symbol: Cow::Borrowed(symbol),
            leverage,
        }
    }
    pub fn default() -> LeverageRequest<'a> {
        LeverageRequest::new(Category::Linear, "BTCUSDT", 10)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LeverageResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    pub result: Empty, // Assuming result is an empty struct as per provided JSON
    pub ret_ext_info: Empty, // Assuming retExtInfo is an empty struct as per provided JSON
    pub time: u64,
}

#[derive(Default, Clone)]
pub struct ChangeMarginRequest<'a> {
    pub category: Category,
    pub symbol: Cow<'a, str>,
    pub trade_mode: i8,
    pub leverage: i8,
}

impl<'a> ChangeMarginRequest<'a> {
    pub fn new(category: Category, symbol: &'a str, trade_mode: i8, leverage: i8) -> Self {
        Self {
            category,
            symbol: Cow::Borrowed(symbol),
            trade_mode: match trade_mode {
                1 => 1,
                0 => 0,
                _ => 0,
            },
            leverage,
        }
    }
    pub fn default() -> ChangeMarginRequest<'a> {
        ChangeMarginRequest::new(Category::Linear, "BTCUSDT", 0, 10)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChangeMarginResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    pub result: Empty, // Assuming result is an empty struct as per provided JSON
    pub ret_ext_info: Empty, // Assuming retExtInfo is an empty struct as per provided JSON
    pub time: u64,
}

#[derive(Clone, Default)]
pub struct MarginModeRequest<'a> {
    pub category: Category,
    pub mode: i8,
    pub symbol: Option<Cow<'a, str>>,
    pub coin: Option<Cow<'a, str>>,
}

impl<'a> MarginModeRequest<'a> {
    pub fn new(
        category: Category,
        mode: i8,
        symbol: Option<&'a str>,
        coin: Option<&'a str>,
    ) -> Self {
        Self {
            category,
            mode,
            symbol: symbol.map(Cow::Borrowed),
            coin: coin.map(Cow::Borrowed),
        }
    }
    pub fn default() -> MarginModeRequest<'a> {
        MarginModeRequest::new(Category::Linear, 1, None, None)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MarginModeResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    pub result: Empty, // Assuming result is an empty struct as per provided JSON
    pub ret_ext_info: Empty, // Assuming retExtInfo is an empty struct as per provided JSON
    pub time: u64,
}

#[derive(Clone, Default)]
pub struct SetRiskLimit<'a> {
    pub category: Category,
    pub symbol: Cow<'a, str>,
    pub risk_id: i8,
    pub position_idx: Option<i32>,
}

impl<'a> SetRiskLimit<'a> {
    pub fn new(
        category: Category,
        symbol: &'a str,
        risk_id: i8,
        position_idx: Option<i32>,
    ) -> Self {
        Self {
            category,
            symbol: Cow::Borrowed(symbol),
            risk_id,
            position_idx,
        }
    }
    pub fn default() -> SetRiskLimit<'a> {
        SetRiskLimit::new(Category::Linear, "BTCUSDT", 1, None)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SetRiskLimitResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    pub result: SetRiskLimitResult,
    pub ret_ext_info: Empty, // Assuming retExtInfo is a JSON value as per provided JSON
    pub time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SetRiskLimitResult {
    pub risk_id: i32,
    #[serde(with = "string_to_u64")]
    pub risk_limit_value: u64,
    pub category: String,
}

#[derive(Clone, Default)]
pub struct TradingStopRequest<'a> {
    pub category: Category,
    pub symbol: Cow<'a, str>,
    pub take_profit: Option<f64>,
    pub stop_loss: Option<f64>,
    pub tp_trigger_by: Option<Cow<'a, str>>,
    pub sl_trigger_by: Option<Cow<'a, str>>,
    pub tpsl_mode: Option<Cow<'a, str>>,
    pub tp_order_type: Option<OrderType>,
    pub sl_order_type: Option<OrderType>,
    pub tp_size: Option<f64>,
    pub sl_size: Option<f64>,
    pub tp_limit_price: Option<f64>,
    pub sl_limit_price: Option<f64>,
    pub position_idx: i32,
}

impl<'a> TradingStopRequest<'a> {
    pub fn new(
        category: Category,
        symbol: &'a str,
        take_profit: Option<f64>,
        stop_loss: Option<f64>,
        tp_trigger_by: Option<&'a str>,
        sl_trigger_by: Option<&'a str>,
        tpsl_mode: Option<&'a str>,
        tp_order_type: Option<OrderType>,
        sl_order_type: Option<OrderType>,
        tp_size: Option<f64>,
        sl_size: Option<f64>,
        tp_limit_price: Option<f64>,
        sl_limit_price: Option<f64>,
        position_idx: i32,
    ) -> Self {
        Self {
            category,
            symbol: Cow::Borrowed(symbol),
            take_profit,
            stop_loss,
            tp_trigger_by: tp_trigger_by.map(Cow::Borrowed),
            sl_trigger_by: sl_trigger_by.map(Cow::Borrowed),
            tpsl_mode: tpsl_mode.map(Cow::Borrowed),
            tp_order_type,
            sl_order_type,
            tp_size,
            sl_size,
            tp_limit_price,
            sl_limit_price,
            position_idx,
        }
    }

    pub fn default() -> TradingStopRequest<'a> {
        TradingStopRequest::new(
            Category::Linear,
            "BTCUSDT",
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            1,
        )
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TradingStopResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    pub result: Empty, // Assuming result is an empty struct as per provided JSON
    pub ret_ext_info: Empty, // Assuming retExtInfo is an empty struct as per provided JSON
    pub time: u64,
}

#[derive(Clone, Default)]
pub struct AddMarginRequest<'a> {
    pub category: Category,
    pub symbol: Cow<'a, str>,
    pub auto_add: bool,
    pub position_idx: Option<i32>,
}

impl<'a> AddMarginRequest<'a> {
    pub fn new(
        category: Category,
        symbol: &'a str,
        auto_add: bool,
        position_idx: Option<i32>,
    ) -> Self {
        Self {
            category,
            symbol: Cow::Borrowed(symbol),
            auto_add,
            position_idx,
        }
    }
    pub fn default() -> AddMarginRequest<'a> {
        AddMarginRequest::new(Category::Linear, "BTCUSDT", false, None)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AddMarginResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    pub result: Empty, // Assuming result is an empty struct as per provided JSON
    pub ret_ext_info: Empty, // Assuming retExtInfo is an empty struct as per provided JSON
    pub time: u64,
}

#[derive(Clone, Default)]
pub struct AddReduceMarginRequest<'a> {
    pub category: Category,
    pub symbol: Cow<'a, str>,
    pub margin: f64,
    pub position_idx: Option<i32>,
}

impl<'a> AddReduceMarginRequest<'a> {
    pub fn new(
        category: Category,
        symbol: &'a str,
        margin: f64,
        position_idx: Option<i32>,
    ) -> Self {
        Self {
            category,
            symbol: Cow::Borrowed(symbol),
            margin,
            position_idx,
        }
    }
    pub fn default() -> AddReduceMarginRequest<'a> {
        AddReduceMarginRequest::new(Category::Linear, "BTCUSDT", 1.0, None)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AddReduceMarginResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    pub result: AddReduceMarginResult,
    pub ret_ext_info: Empty, // Assuming retExtInfo is an empty struct as per provided JSON
    pub time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AddReduceMarginResult {
    pub category: Category,

    pub symbol: String,

    pub position_idx: i32,

    pub risk_id: i32,

    #[serde(with = "string_to_float")]
    pub risk_limit_value: f64,

    #[serde(with = "string_to_float")]
    pub size: f64,

    #[serde(with = "string_to_float")]
    pub position_value: f64,

    #[serde(with = "string_to_float")]
    pub avg_price: f64,

    #[serde(with = "string_to_float")]
    pub liq_price: f64,

    #[serde(with = "string_to_float")]
    pub bust_price: f64,

    #[serde(with = "string_to_float")]
    pub mark_price: f64,

    pub leverage: String,
    pub auto_add_margin: i32,
    pub position_status: String,

    #[serde(rename = "positionIM")]
    pub position_im: String,

    #[serde(rename = "positionMM")]
    pub position_mm: String,

    pub unrealised_pnl: String,
    pub cum_realised_pnl: String,

    #[serde(with = "string_to_float_optional")]
    pub stop_loss: Option<f64>,

    #[serde(with = "string_to_float_optional")]
    pub take_profit: Option<f64>,

    pub trailing_stop: String,

    #[serde(with = "string_to_u64")]
    pub created_time: u64,

    #[serde(with = "string_to_u64")]
    pub updated_time: u64,
}

#[derive(Clone, Default)]
pub struct ClosedPnlRequest<'a> {
    pub category: Category,
    pub symbol: Option<Cow<'a, str>>,
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
    pub limit: Option<u64>,
}

impl<'a> ClosedPnlRequest<'a> {
    pub fn new(
        category: Category,
        symbol: Option<&'a str>,
        start_time: Option<u64>,
        end_time: Option<u64>,
        limit: Option<u64>,
    ) -> Self {
        Self {
            category,
            symbol: symbol.map(Cow::Borrowed),
            start_time,
            end_time,
            limit,
        }
    }
    pub fn default() -> ClosedPnlRequest<'a> {
        ClosedPnlRequest::new(Category::Linear, None, None, None, None)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClosedPnlResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    pub result: ClosedPnlResult,
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClosedPnlResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    pub category: String,
    pub list: Vec<ClosedPnlItem>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClosedPnlItem {
    pub symbol: String,
    pub order_type: String,
    pub leverage: String,
    #[serde(with = "string_to_u64")]
    pub updated_time: u64,
    pub side: String,
    pub order_id: String,
    #[serde(with = "string_to_float")]
    pub closed_pnl: f64,
    #[serde(with = "string_to_float")]
    pub avg_entry_price: f64,
    #[serde(with = "string_to_float")]
    pub qty: f64,
    #[serde(with = "string_to_float")]
    pub cum_entry_value: f64,
    #[serde(with = "string_to_float")]
    pub created_time: f64,
    #[serde(with = "string_to_float")]
    pub order_price: f64,
    #[serde(with = "string_to_float")]
    pub closed_size: f64,
    #[serde(with = "string_to_float")]
    pub avg_exit_price: f64,
    pub exec_type: String,
    pub fill_count: String,
    #[serde(with = "string_to_float")]
    pub cum_exit_value: f64,
}

#[derive(Clone, Default, Serialize)]
pub struct MovePositionRequest<'a> {
    pub from_uid: u64,
    pub to_uid: u64,
    pub list: Vec<PositionItem<'a>>,
}

#[derive(Clone, Default, Serialize)]
pub struct PositionItem<'a> {
    pub category: Category,
    pub symbol: Cow<'a, str>,
    pub price: f64,
    pub side: Side,
    pub qty: f64,
}

impl<'a> MovePositionRequest<'a> {
    pub fn new(from_uid: u64, to_uid: u64, list: Vec<PositionItem<'a>>) -> Self {
        Self {
            from_uid,
            to_uid,
            list,
        }
    }
    pub fn default() -> MovePositionRequest<'a> {
        MovePositionRequest::new(0, 0, vec![])
    }
}
impl<'a> PositionItem<'a> {
    pub fn new(category: Category, symbol: &'a str, price: f64, side: Side, qty: f64) -> Self {
        Self {
            category,
            symbol: Cow::Borrowed(symbol),
            price,
            side,
            qty,
        }
    }
    pub fn default() -> PositionItem<'a> {
        PositionItem::new(Category::Linear, "BTCUSDT", 0.0, Side::Buy, 0.0)
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct MovePositionResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    pub result: MovePositionResult,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct MovePositionResult {
    pub block_trade_id: String,
    pub status: String,
    pub reject_party: String,
}

#[derive(Serialize, Clone, Default)]
pub struct MoveHistoryRequest<'a> {
    pub category: Option<Category>,
    pub symbol: Option<Cow<'a, str>>,
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
    pub status: Option<Cow<'a, str>>,
    pub block_trade_id: Option<Cow<'a, str>>,
    pub limit: Option<Cow<'a, str>>,
}

impl<'a> MoveHistoryRequest<'a> {
    pub fn new(
        category: Option<Category>,
        symbol: Option<&'a str>,
        start_time: Option<u64>,
        end_time: Option<u64>,
        status: Option<&'a str>,
        block_trade_id: Option<&'a str>,
        limit: Option<&'a str>,
    ) -> Self {
        Self {
            category,
            symbol: symbol.map(Cow::Borrowed),
            start_time,
            end_time,
            status: status.map(Cow::Borrowed),
            block_trade_id: block_trade_id.map(Cow::Borrowed),
            limit: limit.map(Cow::Borrowed),
        }
    }
    pub fn default() -> MoveHistoryRequest<'a> {
        MoveHistoryRequest::new(None, None, None, None, None, None, None)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MoveHistoryResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    pub result: MoveHistoryResult,
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MoveHistoryResult {
    pub list: Vec<MoveHistoryEntry>,
    #[serde(rename = "nextPageCursor")]
    pub next_page_cursor: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MoveHistoryEntry {
    pub block_trade_id: String,
    pub category: String,
    pub order_id: String,
    #[serde(rename = "userId")]
    pub user_id: u64,
    pub symbol: String,
    pub side: String,
    #[serde(with = "string_to_float")]
    pub price: f64,
    #[serde(with = "string_to_float")]
    pub qty: f64,
    #[serde(with = "string_to_float")]
    pub exec_fee: f64,
    pub status: String,
    pub exec_id: String,
    pub result_code: i16,
    pub result_message: String,
    pub created_at: u64,
    pub updated_at: u64,
    pub reject_party: String,
}

// = = = = = = = = =  ==  = = == = = == =  = = = = = = = = = = = = = = = =
//
//  ACCOUNT STRUCTS AND RESPONSES
//
// = = = = = = = = = = = = = = = = = = ==  = = = = ==  = = == = =  = = = =

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WalletResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    pub result: WalletList,
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct WalletList {
    pub list: Vec<WalletData>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UTAResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    pub result: UTAUpdateStatus,
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UTAUpdateStatus {
    pub unified_update_status: String,
    pub unified_update_msg: UnifiedUpdateMsg,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct UnifiedUpdateMsg {
    pub msg: Vec<String>,
}

#[derive(Clone, Debug, Default)]
pub struct BorrowHistoryRequest<'a> {
    pub coin: Option<Cow<'a, str>>,
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
    pub limit: Option<Cow<'a, str>>,
}

impl<'a> BorrowHistoryRequest<'a> {
    pub fn new(
        coin: Option<&'a str>,
        start_time: Option<u64>,
        end_time: Option<u64>,
        limit: Option<&'a str>,
    ) -> Self {
        Self {
            coin: coin.map(Cow::Borrowed),
            start_time,
            end_time,
            limit: limit.map(Cow::Borrowed),
        }
    }
    pub fn default() -> BorrowHistoryRequest<'a> {
        BorrowHistoryRequest::new(None, None, None, None)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BorrowHistoryResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    pub result: BorrowHistory,
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BorrowHistory {
    pub next_page_cursor: String,
    pub rows: Vec<BorrowHistoryEntry>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BorrowHistoryEntry {
    #[serde(with = "string_to_float")]
    pub borrow_amount: f64,
    pub cost_exemption: String,
    #[serde(with = "string_to_float")]
    pub free_borrowed_amount: f64,
    pub created_time: u64,
    #[serde(with = "string_to_float")]
    pub interest_bearing_borrow_size: f64,
    pub currency: String,
    #[serde(with = "string_to_float")]
    pub unrealised_loss: f64,
    #[serde(with = "string_to_float")]
    pub hourly_borrow_rate: f64,
    #[serde(with = "string_to_float")]
    pub borrow_cost: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RepayLiabilityResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    pub result: LiabilityQty,
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LiabilityQty {
    pub list: Vec<LiabilityQtyData>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LiabilityQtyData {
    pub coin: String,
    #[serde(with = "string_to_float")]
    pub repayment_qty: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetCollateralCoinResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    pub result: Empty,
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BatchSetCollateralCoinResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    pub result: SwitchList,
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SwitchList {
    pub list: Vec<SwitchListData>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SwitchListData {
    pub coin: String,
    pub collateral_switch: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CollateralInfoResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    pub result: CollateralInfoList,
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CollateralInfoList {
    pub list: Vec<CollateralInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CollateralInfo {
    pub available_to_borrow: String,
    #[serde(with = "string_to_float")]
    pub free_borrowing_amount: f64,
    #[serde(with = "string_to_float")]
    pub free_borrow_amount: f64,
    #[serde(with = "string_to_float")]
    pub max_borrowing_amount: f64,
    #[serde(with = "string_to_float")]
    pub hourly_borrow_rate: f64,
    #[serde(with = "string_to_float")]
    pub borrow_usage_rate: f64,
    pub collateral_switch: bool,
    #[serde(with = "string_to_float")]
    pub borrow_amount: f64,
    pub borrowable: bool,
    pub currency: String,
    pub margin_collateral: bool,
    pub free_borrowing_limit: String,
    pub collateral_ratio: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FeeRateResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    pub result: FeeRateList,
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FeeRateList {
    pub list: Vec<FeeRate>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FeeRate {
    pub symbol: String,
    pub maker_fee_rate: String,
    pub taker_fee_rate: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountInfoResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    pub result: AccountInfo,
    #[serde(default)]
    pub ret_ext_info: Empty,
    #[serde(default)]
    pub time: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountInfo {
    pub margin_mode: String,
    #[serde(with = "string_to_u64")]
    pub updated_time: u64,
    pub unified_margin_status: i8,
    pub dcp_status: String,
    pub time_window: i32,
    pub smp_group: i8,
    pub is_master_trader: bool,
    pub spot_hedging_status: String,
}

#[derive(Clone, Default)]
pub struct TransactionLogRequest<'a> {
    pub account_type: Option<Cow<'a, str>>,
    pub category: Option<Category>,
    pub currency: Option<Cow<'a, str>>,
    pub base_coin: Option<Cow<'a, str>>,
    pub log_type: Option<Cow<'a, str>>,
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
    pub limit: Option<u32>,
}

impl<'a> TransactionLogRequest<'a> {
    pub fn new(
        account_type: Option<&'a str>,
        category: Option<Category>,
        currency: Option<&'a str>,
        base_coin: Option<&'a str>,
        log_type: Option<&'a str>,
        start_time: Option<u64>,
        end_time: Option<u64>,
        limit: Option<u32>,
    ) -> Self {
        Self {
            account_type: account_type.map(Cow::Borrowed),
            category,
            currency: currency.map(Cow::Borrowed),
            base_coin: base_coin.map(Cow::Borrowed),
            log_type: log_type.map(Cow::Borrowed),
            start_time,
            end_time,
            limit,
        }
    }
    pub fn default() -> Self {
        Self::new(None, None, None, None, None, None, None, None)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TransactionLogEntry {
    pub id: String,
    pub symbol: String,
    pub side: String,
    pub funding: Option<String>,
    pub order_link_id: Option<String>,
    pub order_id: String,
    #[serde(with = "string_to_float")]
    pub fee: f64,
    pub change: String,
    #[serde(with = "string_to_float")]
    pub cash_flow: f64,
    pub transaction_time: String,
    pub type_field: String,
    pub fee_rate: String,
    pub bonus_change: Option<String>,
    #[serde(with = "string_to_float")]
    pub size: f64,
    #[serde(with = "string_to_float")]
    pub qty: f64,
    #[serde(with = "string_to_float")]
    pub cash_balance: f64,
    pub currency: String,
    pub category: String,
    pub trade_price: String,
    pub trade_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TransactionLogResult {
    pub next_page_cursor: String,
    pub list: Vec<TransactionLogEntry>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TransactionLogResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    pub result: TransactionLogResult,
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SmpResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    pub result: SmpResult,
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SmpResult {
    pub smp_group: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetMarginModeResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    pub result: MarginModeResult,
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MarginModeResult {
    pub reason: Vec<ReasonObject>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReasonObject {
    pub reason_code: String,
    pub reason_msg: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SpotHedgingResponse {
    pub ret_code: i32,
    pub ret_msg: String,
}

// = = = = = = = = = = = = ==  = == = =  =  = = = = ==
// HEADER STRUCT FOR TRADESTREM RESPONSE
// = = = = = = = = = = = = ==  = == = =  =  = = = = ==

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Header {
    #[serde(rename = "X-Bapi-Limit")]
    pub x_bapi_limit: String,
    #[serde(rename = "X-Bapi-Limit-Status")]
    pub x_bapi_limit_status: String,
    #[serde(rename = "X-Bapi-Limit-Reset-Timestamp")]
    pub x_bapi_limit_reset_timestamp: String,
    #[serde(rename = "Traceid")]
    pub traceid: String,
    #[serde(rename = "Timenow")]
    pub timenow: String,
}

// = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = =
//
// WEBSOCKET STRUCTS AND RESPONSES
//
// = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = =

#[derive(Clone, Debug, Default)]
pub struct Subscription<'a> {
    pub op: &'a str,
    pub args: Vec<&'a str>,
}

impl<'a> Subscription<'a> {
    pub fn new(op: &'a str, args: Vec<&'a str>) -> Self {
        Self { op, args }
    }
    pub fn default() -> Subscription<'a> {
        Subscription::new("subscribe", vec![])
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum WebsocketEvents {
    OrderBookEvent(OrderBookUpdate),
    TradeEvent(TradeUpdate),
    TickerEvent(WsTicker),
    LiquidationEvent(Liquidation),
    KlineEvent(WsKline),
    PositionEvent(PositionEvent),
    ExecutionEvent(Execution),
    OrderEvent(OrderEvent),
    Wallet(WalletEvent),
    TradeStream(TradeStreamEvent),
    FastExecEvent(FastExecution),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Tickers {
    Linear(LinearTickerData),
    Spot(SpotTickerData),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum PongResponse {
    PublicPong(PongData),
    PrivatePong(PongData),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PongData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    pub ret_msg: String,
    pub conn_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub req_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<String>>,
    pub op: String,
}

unsafe impl Send for PongData {}
unsafe impl Sync for PongData {}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TradeStreamEvent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub req_id: Option<String>,
    pub ret_code: i32,
    pub ret_msg: String,
    pub op: String,
    pub data: OrderStatus,
    pub header: Header,
    pub conn_id: String,
}

unsafe impl Send for TradeStreamEvent {}
unsafe impl Sync for TradeStreamEvent {}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderBookUpdate {
    #[serde(rename = "topic")]
    pub topic: String,
    #[serde(rename = "type")]
    pub event_type: String,
    #[serde(rename = "ts")]
    pub timestamp: u64,
    pub data: WsOrderBook,
    pub cts: u64,
}

unsafe impl Send for OrderBookUpdate {}
unsafe impl Sync for OrderBookUpdate {}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WsOrderBook {
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "a")]
    pub asks: Vec<Ask>,
    #[serde(rename = "b")]
    pub bids: Vec<Bid>,
    #[serde(rename = "u")]
    pub update_id: u64,
    pub seq: u64,
}

unsafe impl Send for WsOrderBook {}
unsafe impl Sync for WsOrderBook {}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TradeUpdate {
    pub topic: String,
    #[serde(rename = "type")]
    pub event_type: String,
    #[serde(rename = "ts")]
    pub timestamp: u64,
    pub data: Vec<WsTrade>,
}

unsafe impl Send for TradeUpdate {}
unsafe impl Sync for TradeUpdate {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WsTrade {
    #[serde(rename = "T")]
    pub timestamp: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "S")]
    pub side: String,
    #[serde(rename = "v", with = "string_to_float")]
    pub volume: f64,
    #[serde(rename = "p", with = "string_to_float")]
    pub price: f64,
    #[serde(rename = "L")]
    pub tick_direction: String,
    #[serde(rename = "i")]
    pub id: String,
    #[serde(rename = "BT")]
    pub buyer_is_maker: bool,
}

unsafe impl Send for WsTrade {}
unsafe impl Sync for WsTrade {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WsTicker {
    pub topic: String,
    #[serde(rename = "type")]
    pub event_type: String,
    pub data: Tickers,
    pub cs: u64,
    pub ts: u64,
}

unsafe impl Send for WsTicker {}
unsafe impl Sync for WsTicker {}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LinearTickerData {
    pub symbol: String,
    #[serde(rename = "tickDirection")]
    pub tick_direction: String,
    #[serde(rename = "price24hPcnt", with = "string_to_float")]
    pub price_24h_pcnt: f64,
    #[serde(with = "string_to_float")]
    pub last_price: f64,
    #[serde(rename = "prevPrice24h", with = "string_to_float")]
    pub prev_price_24h: f64,
    #[serde(rename = "highPrice24h", with = "string_to_float")]
    pub high_price_24h: f64,
    #[serde(rename = "lowPrice24h", with = "string_to_float")]
    pub low_price_24h: f64,
    #[serde(rename = "prevPrice1h", with = "string_to_float")]
    pub prev_price_1h: f64,
    #[serde(with = "string_to_float")]
    pub mark_price: f64,
    #[serde(with = "string_to_float")]
    pub index_price: f64,
    #[serde(with = "string_to_float")]
    pub open_interest: f64,
    #[serde(with = "string_to_float")]
    pub open_interest_value: f64,
    #[serde(rename = "turnover24h", with = "string_to_float")]
    pub turnover_24h: f64,
    #[serde(rename = "volume24h", with = "string_to_float")]
    pub volume_24h: f64,
    #[serde(with = "string_to_u64")]
    pub next_funding_time: u64,
    #[serde(with = "string_to_float")]
    pub funding_rate: f64,
    #[serde(rename = "bid1Price", with = "string_to_float")]
    pub bid_price: f64,
    #[serde(rename = "bid1Size", with = "string_to_float")]
    pub bid_size: f64,
    #[serde(rename = "ask1Price", with = "string_to_float")]
    pub ask_price: f64,
    #[serde(rename = "ask1Size", with = "string_to_float")]
    pub ask_size: f64,
}

unsafe impl Send for LinearTickerData {}
unsafe impl Sync for LinearTickerData {}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SpotTickerData {
    pub symbol: String,
    #[serde(with = "string_to_float")]
    pub last_price: f64,
    #[serde(with = "string_to_float")]
    pub high_price_24h: f64,
    #[serde(with = "string_to_float")]
    pub low_price_24h: f64,
    #[serde(with = "string_to_float")]
    pub prev_price_24h: f64,
    #[serde(with = "string_to_float")]
    pub volume_24h: f64,
    #[serde(with = "string_to_float")]
    pub turnover_24h: f64,
    #[serde(with = "string_to_float")]
    pub price_24h_pcnt: f64,
    #[serde(with = "string_to_float")]
    pub usd_index_price: f64,
}

unsafe impl Send for SpotTickerData {}
unsafe impl Sync for SpotTickerData {}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Liquidation {
    #[serde(rename = "topic")]
    pub topic: String,
    #[serde(rename = "type")]
    pub event_type: String,
    #[serde(rename = "ts")]
    pub ts: u64,
    #[serde(rename = "data")]
    pub data: LiquidationData,
}

unsafe impl Send for Liquidation {}
unsafe impl Sync for Liquidation {}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LiquidationData {
    pub updated_time: u64,
    pub symbol: String,
    pub side: String,
    #[serde(with = "string_to_float")]
    pub size: f64,
    #[serde(with = "string_to_float")]
    pub price: f64,
}

unsafe impl Send for LiquidationData {}
unsafe impl Sync for LiquidationData {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WsKline {
    pub topic: String,
    pub data: Vec<KlineData>,
    #[serde(rename = "ts")]
    pub timestamp: u64,
    #[serde(rename = "type")]
    pub event_type: String,
}

unsafe impl Send for WsKline {}
unsafe impl Sync for WsKline {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KlineData {
    pub start: u64,
    pub end: u64,
    pub interval: String,
    #[serde(with = "string_to_float")]
    pub open: f64,
    #[serde(with = "string_to_float")]
    pub close: f64,
    #[serde(with = "string_to_float")]
    pub high: f64,
    #[serde(with = "string_to_float")]
    pub low: f64,
    #[serde(with = "string_to_float")]
    pub volume: f64,
    #[serde(with = "string_to_float")]
    pub turnover: f64,
    pub confirm: bool,
    pub timestamp: u64,
}

unsafe impl Send for KlineData {}
unsafe impl Sync for KlineData {}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PositionEvent {
    pub id: String,
    pub topic: String,
    pub creation_time: u64,
    pub data: Vec<PositionData>,
}

unsafe impl Send for PositionEvent {}
unsafe impl Sync for PositionEvent {}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PositionData {
    pub position_idx: u8,

    pub trade_mode: u8,

    pub risk_id: u8,

    pub risk_limit_value: String,

    pub symbol: String,

    pub side: Side,

    #[serde(with = "string_to_float")]
    pub size: f64,

    #[serde(with = "string_to_float")]
    pub entry_price: f64,

    pub leverage: String,

    #[serde(with = "string_to_float")]
    pub position_value: f64,

    #[serde(with = "string_to_float")]
    pub position_balance: f64,

    #[serde(with = "string_to_float")]
    pub mark_price: f64,

    #[serde(rename = "positionIM")]
    pub position_im: String,

    #[serde(rename = "positionMM")]
    pub position_mm: String,

    #[serde(with = "string_to_float")]
    pub take_profit: f64,

    #[serde(with = "string_to_float")]
    pub stop_loss: f64,

    #[serde(with = "string_to_float_optional")]
    pub trailing_stop: Option<f64>,

    #[serde(rename = "unrealisedPnl")]
    pub unrealised_pnl: String,

    #[serde(rename = "cumRealisedPnl")]
    pub cum_realised_pnl: String,

    #[serde(with = "string_to_u64")]
    pub created_time: u64,

    #[serde(with = "string_to_u64")]
    pub updated_time: u64,

    #[serde(rename = "tpslMode")]
    pub tpsl_mode: String,

    #[serde(with = "string_to_float")]
    pub liq_price: f64,

    #[serde(with = "string_to_float_optional")]
    pub bust_price: Option<f64>,

    pub category: Category,

    pub position_status: String,

    pub adl_rank_indicator: u8,

    pub auto_add_margin: u8,

    #[serde(with = "string_to_u64_optional")]
    pub mmr_sys_updated_time: Option<u64>,

    #[serde(with = "string_to_u64_optional")]
    pub leverage_sys_updated_time: Option<u64>,

    pub seq: u64,

    pub is_reduce_only: bool,
}

unsafe impl Send for PositionData {}
unsafe impl Sync for PositionData {}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Execution {
    pub id: String,
    pub topic: String,
    pub creation_time: u64,
    pub data: Vec<ExecutionData>,
}

unsafe impl Send for Execution {}
unsafe impl Sync for Execution {}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionData {
    pub category: String,
    pub symbol: String,
    #[serde(with = "string_to_float")]
    pub exec_fee: f64,
    pub exec_id: String,
    #[serde(with = "string_to_float")]
    pub exec_price: f64,
    #[serde(with = "string_to_float")]
    pub exec_qty: f64,
    pub exec_type: String,
    #[serde(with = "string_to_float")]
    pub exec_value: f64,
    pub is_maker: bool,
    #[serde(rename = "feeRate", with = "string_to_float")]
    pub fee_rate: f64,
    #[serde(rename = "tradeIv")]
    pub trade_iv: String,
    #[serde(rename = "markIv")]
    pub mark_iv: String,
    #[serde(rename = "blockTradeId")]
    pub block_trade_id: String,
    #[serde(with = "string_to_float")]
    pub mark_price: f64,
    #[serde(with = "string_to_float")]
    pub index_price: f64,
    #[serde(with = "string_to_float")]
    pub underlying_price: f64,
    #[serde(with = "string_to_float")]
    pub leaves_qty: f64,
    pub order_id: String,
    pub order_link_id: String,
    #[serde(with = "string_to_float")]
    pub order_price: f64,
    #[serde(with = "string_to_float")]
    pub order_qty: f64,
    pub order_type: String,
    #[serde(rename = "stopOrderType")]
    pub stop_order_type: String,
    pub side: String,
    #[serde(with = "string_to_u64")]
    pub exec_time: u64,
    pub is_leverage: String,
    pub closed_size: String,
    pub seq: u64,
}

unsafe impl Send for ExecutionData {}
unsafe impl Sync for ExecutionData {}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FastExecution {
    pub topic: String,
    pub creation_time: u64,
    pub data: Vec<FastExecData>,
}

unsafe impl Send for FastExecution {}
unsafe impl Sync for FastExecution {}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FastExecData {
    pub category: String,
    pub symbol: String,
    pub exec_id: String,
    #[serde(with = "string_to_float")]
    pub exec_price: f64,
    #[serde(with = "string_to_float")]
    pub exec_qty: f64,
    pub order_id: String,
    pub order_link_id: String,
    pub side: String,
    #[serde(with = "string_to_u64")]
    pub exec_time: u64,
    pub seq: u64,
}

unsafe impl Send for FastExecData {}
unsafe impl Sync for FastExecData {}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderData {
    pub symbol: String,
    pub order_id: String,
    pub side: String,
    pub order_type: String,
    pub cancel_type: String,
    #[serde(with = "string_to_float")]
    pub price: f64,
    #[serde(with = "string_to_float")]
    pub qty: f64,
    pub order_iv: String,
    pub time_in_force: String,
    pub order_status: String,
    pub order_link_id: String,
    #[serde(with = "string_to_float")]
    pub last_price_on_created: f64,
    pub reduce_only: bool,
    #[serde(with = "string_to_float")]
    pub leaves_qty: f64,
    #[serde(with = "string_to_float")]
    pub leaves_value: f64,
    #[serde(with = "string_to_float")]
    pub cum_exec_qty: f64,
    #[serde(with = "string_to_float")]
    pub cum_exec_value: f64,
    #[serde(with = "string_to_float")]
    pub avg_price: f64,
    pub block_trade_id: String,
    #[serde(rename = "positionIdx")]
    pub position_idx: u8,
    #[serde(with = "string_to_float")]
    pub cum_exec_fee: f64,
    #[serde(with = "string_to_u64")]
    pub created_time: u64,
    #[serde(with = "string_to_u64")]
    pub updated_time: u64,
    pub reject_reason: String,
    #[serde(rename = "stopOrderType")]
    pub stop_order_type: String,
    #[serde(rename = "tpslMode")]
    pub tpsl_mode: String,
    #[serde(with = "string_to_float")]
    pub trigger_price: f64,
    #[serde(with = "string_to_float")]
    pub take_profit: f64,
    #[serde(with = "string_to_float")]
    pub stop_loss: f64,
    #[serde(rename = "tpTriggerBy")]
    pub tp_trigger_by: String,
    #[serde(rename = "slTriggerBy")]
    pub sl_trigger_by: String,
    #[serde(rename = "tpLimitPrice", with = "string_to_float")]
    pub tp_limit_price: f64,
    #[serde(rename = "slLimitPrice", with = "string_to_float")]
    pub sl_limit_price: f64,
    pub trigger_direction: u8,
    pub trigger_by: String,
    pub close_on_trigger: bool,
    pub category: String,
    pub place_type: String,
    pub smp_type: String,
    pub smp_group: u8,
    pub smp_order_id: String,
    pub fee_currency: String,
}

unsafe impl Send for OrderData {}
unsafe impl Sync for OrderData {}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderEvent {
    pub id: String,
    pub topic: String,
    pub creation_time: u64,
    pub data: Vec<OrderData>,
}

unsafe impl Send for OrderEvent {}
unsafe impl Sync for OrderEvent {}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WalletEvent {
    pub id: String,
    pub topic: String,
    pub creation_time: u64,
    pub data: Vec<WalletData>,
}
unsafe impl Send for WalletEvent {}
unsafe impl Sync for WalletEvent {}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WalletData {
    #[serde(rename = "accountIMRate", with = "string_to_float_optional")]
    pub account_im_rate: Option<f64>,

    #[serde(rename = "accountMMRate", with = "string_to_float_optional")]
    pub account_mm_rate: Option<f64>,

    #[serde(with = "string_to_float")]
    pub total_equity: f64,

    #[serde(with = "string_to_float")]
    pub total_wallet_balance: f64,

    #[serde(with = "string_to_float_optional")]
    pub total_margin_balance: Option<f64>,

    #[serde(with = "string_to_float_optional")]
    pub total_available_balance: Option<f64>,

    #[serde(rename = "totalPerpUPL", with = "string_to_float")]
    pub total_perp_upl: f64,

    #[serde(with = "string_to_float_optional")]
    pub total_initial_margin: Option<f64>,

    #[serde(with = "string_to_float_optional")]
    pub total_maintenance_margin: Option<f64>,

    pub coin: Vec<CoinData>,

    #[serde(rename = "accountLTV", with = "string_to_float_optional")]
    pub account_ltv: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
}

unsafe impl Send for WalletData {}
unsafe impl Sync for WalletData {}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CoinData {
    pub coin: String,

    #[serde(with = "string_to_float")]
    pub equity: f64,

    #[serde(with = "string_to_float")]
    pub usd_value: f64,

    #[serde(with = "string_to_float")]
    pub wallet_balance: f64,

    #[serde(with = "string_to_float_optional")]
    pub available_to_withdraw: Option<f64>,

    #[serde(with = "string_to_float_optional")]
    pub available_to_borrow: Option<f64>,

    #[serde(with = "string_to_float")]
    pub borrow_amount: f64,

    #[serde(with = "string_to_float")]
    pub accrued_interest: f64,

    #[serde(rename = "totalOrderIM", with = "string_to_float")]
    pub total_order_im: f64,

    #[serde(rename = "totalPositionIM", with = "string_to_float")]
    pub total_position_im: f64,

    #[serde(rename = "totalPositionMM", with = "string_to_float")]
    pub total_position_mm: f64,

    #[serde(with = "string_to_float")]
    pub unrealised_pnl: f64,

    #[serde(with = "string_to_float")]
    pub cum_realised_pnl: f64,

    #[serde(with = "string_to_float")]
    pub bonus: f64,

    pub collateral_switch: bool,

    pub margin_collateral: bool,

    #[serde(with = "string_to_float")]
    pub locked: f64,

    #[serde(with = "string_to_float")]
    pub spot_hedging_qty: f64,
}

unsafe impl Send for CoinData {}
unsafe impl Sync for CoinData {}

mod string_to_u64 {
    use std::default;

    use serde::{self, Deserialize, Deserializer, Serializer};

    // Serialize a u64 as a string.
    pub fn serialize<S>(value: &u64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = value.to_string();
        serializer.serialize_str(&s)
    }

    // Deserialize a string to a u64.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<u64, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse::<u64>().map_err(serde::de::Error::custom)
    }
}

pub mod string_to_float {
    use serde::{self, Deserialize, Deserializer, Serializer};

    // Serialize a u64 as a string.
    pub fn serialize<S>(value: &f64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = value.to_string();
        serializer.serialize_str(&s)
    }

    // Deserialize a string as an f64.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<f64, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse::<f64>().map_err(serde::de::Error::custom)
    }
}

mod string_to_u64_optional {
    use serde::{Deserialize, Deserializer, Serializer};
    use std::str::FromStr;

    // Serialization: Convert Option<u64> to string
    pub fn serialize<S>(value: &Option<u64>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(v) => serializer.serialize_str(&v.to_string()),
            None => serializer.serialize_str(""),
        }
    }

    // Deserialization: Parse string to Option<u64>, return None for empty string
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s.is_empty() {
            Ok(None) // Return None for empty string
        } else {
            u64::from_str(&s)
                .map(Some)
                .map_err(serde::de::Error::custom)
        }
    }
}
