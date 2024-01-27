use crate::errors::{Error, ErrorKind, Result};
use serde::{Deserialize, Serialize};
use serde_json::{from_value, Value};
use std::borrow::Cow;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Empty {}

/// ----------------------------------------
///  RESPONSE STRUCTS FOR MARKET REQUESTS
/// ----------------------------------------

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ServerTimeResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i16,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: ServerTime,
    #[serde(rename = "retExtInfo")]
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

#[derive(Clone)]
pub struct KlineRequest<'a> {
    pub category: Option<Category>,
    pub symbol: Cow<'a, str>,
    pub interval: Cow<'a, str>,
    pub start: Option<Cow<'a, str>>,
    pub end: Option<Cow<'a, str>>,
    pub limit: Option<u64>,
}

impl<'a> KlineRequest<'a> {
    pub fn new(
        category: Option<Category>,
        symbol: &'a str,
        interval: &'a str,
        start: Option<&'a str>,
        end: Option<&'a str>,
        limit: Option<u64>,
    ) -> KlineRequest<'a> {
        KlineRequest {
            category: category,
            symbol: Cow::Borrowed(symbol),
            interval: Cow::Borrowed(interval),
            start: start.map(|s| Cow::Borrowed(s)),
            end: end.map(|s| Cow::Borrowed(s)),
            limit,
        }
    }
}
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct KlineResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i16,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: KlineSummary,
    #[serde(rename = "retExtInfo")]
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

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MarkPriceKlineResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i16,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: MarkPriceKlineSummary,
    #[serde(rename = "retExtInfo")]
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

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IndexPriceKlineResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i16,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: IndexPriceKlineSummary,
    #[serde(rename = "retExtInfo")]
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

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PremiumIndexPriceKlineResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i16,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: PremiumIndexPriceKlineSummary,
    #[serde(rename = "retExtInfo")]
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

#[derive(Clone)]
pub struct InstrumentRequest<'a> {
    pub category: Category,
    pub symbol: Option<Cow<'a, str>>,
    pub status: Option<bool>,
    pub base_coin: Option<Cow<'a, str>>,
    pub limit: Option<u64>,
}
impl<'a> InstrumentRequest<'a> {
    pub fn new(
        category: Category,
        symbol: Option<&'a str>,
        status: Option<bool>,
        base_coin: Option<&'a str>,
        limit: Option<u64>,
    ) -> InstrumentRequest<'a> {
        InstrumentRequest {
            category: category,
            symbol: symbol.map(|s| Cow::Borrowed(s)),
            status: status,
            base_coin: base_coin.map(|s| Cow::Borrowed(s)),
            limit,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FuturesInstrumentsInfoResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i16,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: FuturesInstrumentsInfo,
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FuturesInstrumentsInfo {
    pub category: String,
    pub list: Vec<FuturesInstrument>,
    #[serde(rename = "nextPageCursor")]
    pub next_page_cursor: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FuturesInstrument {
    pub symbol: String,
    #[serde(rename = "contractType")]
    pub contract_type: String,
    pub status: String,
    #[serde(rename = "baseCoin")]
    pub base_coin: String,
    #[serde(rename = "quoteCoin")]
    pub quote_coin: String,
    #[serde(rename = "launchTime", with = "string_to_u64")]
    pub launch_time: u64,
    #[serde(rename = "deliveryTime", with = "string_to_u64")]
    pub delivery_time: u64,
    #[serde(rename = "deliveryFeeRate")]
    pub delivery_fee_rate: String,
    #[serde(rename = "priceScale")]
    pub price_scale: String,
    #[serde(rename = "leverageFilter")]
    pub leverage_filter: LeverageFilter,
    #[serde(rename = "priceFilter")]
    pub price_filter: PriceFilter,
    #[serde(rename = "lotSizeFilter")]
    pub lot_size_filter: LotSizeFilter,
    #[serde(rename = "unifiedMarginTrade")]
    pub unified_margin_trade: bool,
    #[serde(rename = "fundingInterval")]
    pub funding_interval: u64,
    #[serde(rename = "settleCoin")]
    pub settle_coin: String,
    #[serde(rename = "copyTrading")]
    pub copy_trading: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SpotInstrumentsInfoResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i16,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: SpotInstrumentsInfo,
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SpotInstrumentsInfo {
    pub category: String,
    pub list: Vec<SpotInstrument>,
    #[serde(rename = "nextPageCursor")]
    pub next_page_cursor: Option<String>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SpotInstrument {
    pub symbol: String,
    #[serde(rename = "baseCoin")]
    pub base_coin: String,
    #[serde(rename = "quoteCoin")]
    pub quote_coin: String,
    pub innovation: String,
    pub status: String,
    #[serde(rename = "marginTrading")]
    pub margin_trading: String,
    #[serde(rename = "lotSizeFilter")]
    pub lot_size_filter: LotSizeFilter,
    #[serde(rename = "priceFilter")]
    pub price_filter: PriceFilter,
    #[serde(rename = "riskParameters")]
    pub risk_parameters: RiskParameters,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OptionsInstrument {
    pub symbol: String,
    pub status: String,
    #[serde(rename = "baseCoin")]
    pub base_coin: String,
    #[serde(rename = "quoteCoin")]
    pub quote_coin: String,
    #[serde(rename = "settleCoin")]
    pub settle_coin: String,
    #[serde(rename = "optionType")]
    pub option_type: String,
    #[serde(rename = "launchTime", with = "string_to_u64")]
    pub launch_time: u64,
    #[serde(rename = "deliveryTime", with = "string_to_u64")]
    pub delivery_time: u64,
    #[serde(rename = "deliveryFeeRate")]
    pub delivery_fee_rate: String,
    #[serde(rename = "priceFilter")]
    pub price_filter: PriceFilter,
    #[serde(rename = "lotSizeFilter")]
    pub lot_size_filter: LotSizeFilter,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RiskParameters {
    #[serde(rename = "limitParameter")]
    pub limit_parameter: String,
    #[serde(rename = "marketParameter")]
    pub market_parameter: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LeverageFilter {
    #[serde(rename = "minLeverage")]
    pub min_leverage: String,
    #[serde(rename = "maxLeverage")]
    pub max_leverage: String,
    #[serde(rename = "leverageStep")]
    pub leverage_step: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PriceFilter {
    #[serde(rename = "minPrice")]
    pub min_price: Option<String>,
    #[serde(rename = "maxPrice")]
    pub max_price: Option<String>,
    #[serde(rename = "tickSize", with = "string_to_float")]
    pub tick_size: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LotSizeFilter {
    #[serde(rename = "basePrecision", skip_serializing_if = "Option::is_none")]
    pub base_precision: Option<String>,
    #[serde(rename = "quotePrecision", skip_serializing_if = "Option::is_none")]
    pub quote_precision: Option<String>,
    #[serde(rename = "minOrderQty", with = "string_to_float")]
    pub min_order_qty: f64,
    #[serde(rename = "maxOrderQty", with = "string_to_float")]
    pub max_order_qty: f64,
    #[serde(rename = "minOrderAmt", skip_serializing_if = "Option::is_none")]
    pub min_order_amt: Option<String>,
    #[serde(rename = "maxOrderAmt", skip_serializing_if = "Option::is_none")]
    pub max_order_amt: Option<String>,
    #[serde(rename = "qtyStep", skip_serializing_if = "Option::is_none")]
    pub qty_step: Option<String>,
    #[serde(
        rename = "postOnlyMaxOrderQty",
        skip_serializing_if = "Option::is_none"
    )]
    pub post_only_max_order_qty: Option<String>,
}

#[derive(Clone)]
pub struct OrderbookRequest<'a> {
    pub symbol: Cow<'a, str>,
    pub category: Category,
    pub limit: Option<u64>,
}

impl<'a> OrderbookRequest<'a> {
    pub fn new(symbol: &'a str, category: Category, limit: Option<u64>) -> OrderbookRequest<'a> {
        OrderbookRequest {
            symbol: Cow::Borrowed(symbol),
            category,
            limit,
        }
    }
}
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderBookResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i16,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: OrderBook,
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: Empty,
    pub time: u64,
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

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FuturesTickersResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i16,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: FuturesTickers,
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: Empty,
    pub time: u64,
}
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SpotTickersResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i16,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: SpotTickers,
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FuturesTickers {
    pub category: String,
    pub list: Vec<FuturesTicker>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SpotTickers {
    pub category: String,
    pub list: Vec<SpotTicker>,
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
    #[serde(rename = "prevPrice24h", with = "string_to_float")]
    pub prev_price_24h: f64,
    #[serde(rename = "price24hPcnt", with = "string_to_float")]
    pub daily_change_percentage: f64,
    #[serde(rename = "highPrice24h", with = "string_to_float")]
    pub high_24h: f64,
    #[serde(rename = "lowPrice24h", with = "string_to_float")]
    pub low_24h: f64,
    #[serde(rename = "prevPrice1h", with = "string_to_float")]
    pub prev_price_1h: f64,
    #[serde(with = "string_to_float")]
    pub open_interest: f64,
    #[serde(with = "string_to_float")]
    pub open_interest_value: f64,
    #[serde(rename = "turnover24h")]
    pub turnover_24h: String,
    #[serde(rename = "volume24h")]
    pub volume_24h: String,
    pub funding_rate: String,
    #[serde(rename = "nextFundingTime", with = "string_to_u64")]
    pub next_funding_time: u64,
    pub predicted_delivery_price: String,
    pub basis_rate: String,
    pub delivery_fee_rate: String,
    #[serde(rename = "deliveryTime", with = "string_to_u64")]
    pub delivery_time: u64,
    #[serde(rename = "ask1Size", with = "string_to_float")]
    pub ask_size: f64,
    #[serde(rename = "bid1Price", with = "string_to_float")]
    pub bid_price: f64,
    #[serde(rename = "ask1Price", with = "string_to_float")]
    pub ask_price: f64,
    #[serde(rename = "bid1Size", with = "string_to_float")]
    pub bid_size: f64,
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
    #[serde(rename = "turnover24h")]
    pub turnover_24h: String,
    #[serde(rename = "volume24h")]
    pub volume_24h: String,
    #[serde(rename = "usdIndexPrice")]
    pub usd_index_price: String,
}

#[derive(Clone)]
pub struct FundingHistoryRequest<'a> {
    pub category: Category,
    pub symbol: Cow<'a, str>,
    pub start_time: Option<Cow<'a, str>>,
    pub end_time: Option<Cow<'a, str>>,
    pub limit: Option<u64>,
}
impl<'a> FundingHistoryRequest<'a> {
    pub fn new(
        category: Category,
        symbol: &'a str,
        start_time: Option<&'a str>,
        end_time: Option<&'a str>,
        limit: Option<u64>,
    ) -> FundingHistoryRequest<'a> {
        FundingHistoryRequest {
            category,
            symbol: Cow::Borrowed(symbol),
            start_time: start_time.map(|s| Cow::Borrowed(s)),
            end_time: end_time.map(|s| Cow::Borrowed(s)),
            limit,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FundingRateResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i16,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: FundingRateSummary,
    #[serde(rename = "retExtInfo")]
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
    #[serde(rename = "fundingRate", with = "string_to_float")]
    pub funding_rate: f64,
    #[serde(rename = "fundingRateTimestamp", with = "string_to_u64")]
    pub funding_rate_timestamp: u64,
}

#[derive(Clone)]
pub struct RecentTradesRequest<'a> {
    pub category: Category,
    pub symbol: Option<Cow<'a, str>>,
    pub base_coin: Option<Cow<'a, str>>,
    pub limit: Option<u64>,
}
impl<'a> RecentTradesRequest<'a> {
    pub fn new(
        category: Category,
        symbol: Option<&'a str>,
        base_coin: Option<&'a str>,
        limit: Option<u64>,
    ) -> RecentTradesRequest<'a> {
        RecentTradesRequest {
            category,
            symbol: symbol.map(|s| Cow::Borrowed(s)),
            base_coin: base_coin.map(|s| Cow::Borrowed(s)),
            limit,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RecentTradesResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i16,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: RecentTrades,
    #[serde(rename = "retExtInfo")]
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
    #[serde(rename = "execId")]
    pub exec_id: String,
    pub symbol: String,
    #[serde(with = "string_to_float")]
    pub price: f64,
    #[serde(rename = "size", with = "string_to_float")]
    pub qty: f64,
    pub side: String,
    #[serde(rename = "time")]
    pub timestamp: String,
    #[serde(rename = "isBlockTrade")]
    pub is_block_trade: bool,
}

#[derive(Clone)]
pub struct OpenInterestRequest<'a> {
    pub category: Category,
    pub symbol: Cow<'a, str>,
    pub interval: Cow<'a, str>,
    pub start: Option<Cow<'a, str>>,
    pub end: Option<Cow<'a, str>>,
    pub limit: Option<u64>,
}

impl<'a> OpenInterestRequest<'a> {
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
            start: start.map(|s| Cow::Borrowed(s)),
            end: end.map(|s| Cow::Borrowed(s)),
            limit,
        }
    }
}
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OpeninterestResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i16,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: OpenInterestSummary,
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OpenInterestSummary {
    pub symbol: String,
    pub category: String,
    pub list: Vec<OpenInterest>,
    #[serde(rename = "nextPageCursor")]
    pub next_page_cursor: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OpenInterest {
    #[serde(rename = "openInterest", with = "string_to_float")]
    pub open_interest: f64,
    #[serde(with = "string_to_u64")]
    pub timestamp: u64,
}

#[derive(Clone)]
pub struct HistoricalVolatilityRequest<'a> {
    pub base_coin: Option<Cow<'a, str>>,
    pub period: Option<Cow<'a, str>>,
    pub start: Option<Cow<'a, str>>,
    pub end: Option<Cow<'a, str>>,
}

impl<'a> HistoricalVolatilityRequest<'a> {
    pub fn new(
        base_coin: Option<&'a str>,
        period: Option<&'a str>,
        start: Option<&'a str>,
        end: Option<&'a str>,
    ) -> HistoricalVolatilityRequest<'a> {
        HistoricalVolatilityRequest {
            base_coin: base_coin.map(|s| Cow::Borrowed(s)),
            period: period.map(|s| Cow::Borrowed(s)),
            start: start.map(|s| Cow::Borrowed(s)),
            end: end.map(|s| Cow::Borrowed(s)),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalVolatilityResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i16,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub category: String,
    #[serde(rename = "result")]
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

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InsuranceResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i16,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: InsuranceSummary,
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InsuranceSummary {
    #[serde(rename = "updatedTime", with = "string_to_u64")]
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

#[derive(Clone)]
pub struct RiskLimitRequest<'a> {
    pub category: Category,
    pub symbol: Option<Cow<'a, str>>,
}

impl<'a> RiskLimitRequest<'a> {
    pub fn new(category: Category, symbol: Option<&'a str>) -> RiskLimitRequest<'a> {
        RiskLimitRequest {
            category,
            symbol: symbol.map(|s| Cow::Borrowed(s)),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RiskLimitResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i16,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: RiskLimitSummary,
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RiskLimitSummary {
    pub category: String,
    pub list: Vec<RiskLimit>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RiskLimit {
    pub id: u64,
    pub symbol: String,
    #[serde(rename = "riskLimitValue", with = "string_to_float")]
    pub risk_limit_value: f64,
    #[serde(rename = "maintenanceMargin", with = "string_to_float")]
    pub maintainence_margin: f64,
    #[serde(rename = "initialMargin", with = "string_to_float")]
    pub initial_margin: f64,
    #[serde(rename = "isLowestRisk")]
    pub is_lowest_risk: u8,
    #[serde(rename = "maxLeverage")]
    pub max_leverage: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryPriceResponse {
    pub ret_code: i16,
    pub ret_msg: String,
    pub result: DeliveryPriceSummary,
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryPriceSummary {
    pub category: String,
    #[serde(rename = "nextPageCursor", skip_serializing_if = "Option::is_none")]
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

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LongShortRatioResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i16,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: LongShortRatioSummary,
    #[serde(rename = "retExtInfo")]
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
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "buyRatio", with = "string_to_float")]
    pub buy_ratio: f64,
    #[serde(rename = "sellRatio", with = "string_to_float")]
    pub sell_ratio: f64,
    #[serde(rename = "timestamp", with = "string_to_u64")]
    pub timestamp: u64,
}

/// --------------------------------------------------
///  REQUEST & RESPONSE STRUCTS FOR TRADE
/// --------------------------------------------------
#[derive(Clone)]
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum OrderType {
    Limit,
    Market,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Order {}

pub struct OrderHistoryRequest<'a> {
    pub category: Category,
    pub symbol: Option<Cow<'a, str>>,
    pub base_coin: Option<Cow<'a, str>>,
    pub settle_coin: Option<Cow<'a, str>>,
    pub order_id: Option<Cow<'a, str>>,
    pub order_link_id: Option<Cow<'a, str>>,
    pub order_filter: Option<Cow<'a, str>>,
    pub order_status: Option<Cow<'a, str>>,
    pub start_time: Option<Cow<'a, str>>,
    pub end_time: Option<Cow<'a, str>>,
    pub limit: Option<u64>,
}

impl<'a> OrderHistoryRequest<'a> {
    pub fn new(
        category: Category,
        symbol: Option<&'a str>,
        base_coin: Option<&'a str>,
        settle_coin: Option<&'a str>,
        order_id: Option<&'a str>,
        order_link_id: Option<&'a str>,
        order_filter: Option<&'a str>,
        order_status: Option<&'a str>,
        start_time: Option<&'a str>,
        end_time: Option<&'a str>,
        limit: Option<u64>,
    ) -> OrderHistoryRequest<'a> {
        OrderHistoryRequest {
            category,
            symbol: symbol.map(|s| Cow::Borrowed(s)),
            base_coin: base_coin.map(|s| Cow::Borrowed(s)),
            settle_coin: settle_coin.map(|s| Cow::Borrowed(s)),
            order_id: order_id.map(|s| Cow::Borrowed(s)),
            order_link_id: order_link_id.map(|s| Cow::Borrowed(s)),
            order_filter: order_filter.map(|s| Cow::Borrowed(s)),
            order_status: order_status.map(|s| Cow::Borrowed(s)),
            start_time: start_time.map(|s| Cow::Borrowed(s)),
            end_time: end_time.map(|s| Cow::Borrowed(s)),
            limit,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderHistoryResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i16,
    #[serde(rename = "retMsg")]
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
    pub list: Vec<Orders>,
    #[serde(rename = "nextPageCursor", skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Orders {
    #[serde(rename = "orderId")]
    pub order_id: String,
    #[serde(rename = "orderLinkId")]
    pub order_link_id: String,
    #[serde(rename = "blockTradeId")]
    pub block_trade_id: String,
    pub symbol: String,
    #[serde(with = "string_to_float")]
    pub price: f64,
    #[serde(with = "string_to_float")]
    pub qty: f64,
    pub side: Side,
    #[serde(rename = "isLeverage")]
    pub is_leverage: String,
    #[serde(rename = "positionIdx")]
    pub position_idx: i32,
    #[serde(rename = "orderStatus")]
    pub order_status: String,
    #[serde(rename = "cancelType")]
    pub cancel_type: String,
    #[serde(rename = "rejectReason")]
    pub reject_reason: String,
    #[serde(rename = "avgPrice", with = "string_to_float")]
    pub avg_price: f64,
    #[serde(rename = "leavesQty", with = "string_to_float")]
    pub leaves_qty: f64,
    #[serde(rename = "leavesValue", with = "string_to_float")]
    pub leaves_value: f64,
    #[serde(rename = "cumExecQty", with = "string_to_float")]
    pub cum_exec_qty: f64,
    #[serde(rename = "cumExecValue", with = "string_to_float")]
    pub cum_exec_value: f64,
    #[serde(rename = "cumExecFee", with = "string_to_float")]
    pub cum_exec_fee: f64,
    #[serde(rename = "timeInForce")]
    pub time_in_force: String,
    #[serde(rename = "orderType")]
    pub order_type: OrderType,
    #[serde(rename = "stopOrderType")]
    pub stop_order_type: String,
    #[serde(rename = "orderIv")]
    pub order_iv: String,
    #[serde(rename = "triggerPrice", with = "string_to_float")]
    pub trigger_price: f64,
    #[serde(rename = "takeProfit", with = "string_to_float")]
    pub take_profit: f64,
    #[serde(rename = "stopLoss", with = "string_to_float")]
    pub stop_loss: f64,
    #[serde(rename = "tpTriggerBy")]
    pub tp_trigger_by: String,
    #[serde(rename = "slTriggerBy")]
    pub sl_trigger_by: String,
    #[serde(rename = "triggerDirection")]
    pub trigger_direction: i32,
    #[serde(rename = "triggerBy")]
    pub trigger_by: String,
    #[serde(rename = "lastPriceOnCreated", with = "string_to_float")]
    pub last_price_on_created: f64,
    #[serde(rename = "reduceOnly")]
    pub reduce_only: bool,
    #[serde(rename = "closeOnTrigger")]
    pub close_on_trigger: bool,
    #[serde(rename = "smpType")]
    pub smp_type: String,
    #[serde(rename = "smpGroup")]
    pub smp_group: i32,
    #[serde(rename = "smpOrderId")]
    pub smp_order_id: String,
    #[serde(rename = "tpslMode")]
    pub tpsl_mode: String,
    #[serde(rename = "tpLimitPrice", with = "string_to_float")]
    pub tp_limit_price: f64,
    #[serde(rename = "slLimitPrice", with = "string_to_float")]
    pub sl_limit_price: f64,
    #[serde(rename = "placeType")]
    pub place_type: String,
    #[serde(with = "string_to_u64")]
    pub created_time: u64,
    #[serde(with = "string_to_u64")]
    pub updated_time: u64,
}

mod string_to_u64 {
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

mod string_to_float {
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
