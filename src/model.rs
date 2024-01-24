use crate::errors::{Error, ErrorKind, Result};
use serde::de::{self, Deserializer, SeqAccess, Visitor};
use serde::{Deserialize, Serialize};
use serde_json::{from_value, Value};
use std::convert::TryFrom;
use std::fmt::{self, format};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Empty {}

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
    #[serde(with = "string")]
    pub time_second: u64,
    #[serde(with = "string")]
    pub time_nano: u64,
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
    pub start_time: String,
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
    pub start_time: String,
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
    pub start_time: String,
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
    pub result: InstrumentsInfo,
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Instrument {
    Linear(LinearInstrument),
    Spot(SpotInstrument),
    Option(OptionInstrument),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InstrumentsInfo {
    pub category: String,
    pub list: Vec<Instrument>,
    pub next_page_cursor: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LinearInstrument {
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

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SpotInstrument {
    symbol: String,
    base_coin: String,
    quote_coin: String,
    status: String,
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

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OptionInstrument {
    symbol: String,
    status: String,
    base_coin: String,
    quote_coin: String,
    settle_coin: String,
    launch_time: u64,
    delivery_time: u64,
    delivery_fee_rate: String,
    price_filter: PriceFilter,
    lot_size_filter: LotSizeFilter,
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
    pub time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Tickers {
    pub category: String,
    pub list: Vec<Ticker>,
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
    pub basis_rate: String,
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
    pub time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RecentTradeList {
    pub category: String,
    pub list: Vec<RecentTrade>,
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
    pub time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OpenInterestSummary {
    pub symbol: String,
    pub list: Vec<OpenInterest>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OpenInterest {
    pub open_interest: String,
    pub timestamp: u64,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalVolatilityResponse {
    pub ret_code: i16,
    pub ret_msg: String,
    pub category: String,
    pub result: HistoricalVolatilitySummary,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalVolatilitySummary {
    pub list: Vec<HistoricalVolatility>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalVolatility {
    pub period: String,
    pub value: String,
    pub timestamp: u64,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InsuranceResponse {
    pub ret_code: i16,
    pub ret_msg: String,
    pub result: InsuranceSummary,
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InsuranceSummary {
    pub updated_time: u64,
    pub list: Vec<Insurance>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Insurance {
    pub coin: String,
    pub balance: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RiskLimitResponse {
    pub ret_code: i16,
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
    pub id: String,
    pub symbol: String,
    pub risk_limit_value: String,
    pub maintainence_margin: String,
    pub initial_margin: String,
    pub is_lowest_risk: u8,
    pub max_leverage: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryPriceResponse {
    pub ret_code: i16,
    pub ret_msg: String,
    pub result: DeliveryPrice,
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryPriceSummary {
    pub category: String,
    pub list: Vec<DeliveryPrice>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryPrice {
    pub symbol: String,
    pub delivery_price: String,
    pub delivery_time: u64,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LongShortRatioResponse {
    pub ret_code: i16,
    pub ret_msg: String,
    pub result: LongShortRatioSummary,
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LongShortRatioSummary {
    pub category: String,
    pub list: Vec<LongShortRatio>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LongShortRatio {
    pub symbol: String,
    pub buy_ratio: String,
    pub sell_ratio: String,
    pub timestamp: u64,
}

mod string {
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(value: &u64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&value.to_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<u64, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse::<u64>().map_err(serde::de::Error::custom)
    }
}
