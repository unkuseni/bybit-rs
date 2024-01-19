use serde::{Deserialize, Serialize};
use serde_json::{from_value, Value};
use std::convert::TryFrom;

#[derive(Deserialize, Clone)]
pub struct Empty {}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeInformation {
    pub timezone: String,
    pub server_time: ServerTime,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ServerTime {
    pub result: ServerTimeResult,
    pub time: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServerTimeResult {
    pub time_second: String,
    pub time_nano: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Kline {
    pub start_time: i64,
    pub open: String,
    pub high: String,
    pub low: String,
    pub close: String,
    pub volume: String,
    pub quote_asset_volume: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KlineSummary {
    pub symbol: String,
    pub category: String,
    pub list: Vec<Kline>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KlineResponse {
    pub result: KlineSummary,
    pub time: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MarkPriceKline {
    pub start_time: i64,
    pub open: String,
    pub high: String,
    pub low: String,
    pub close: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MarkPriceKlineSummary {
    pub symbol: String,
    pub category: String,
    pub list: Vec<MarkPriceKline>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MarkPriceKlineResponse {
    pub result: MarkPriceKlineSummary,
    pub time: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IndexPriceKline {
    pub start_time: i64,
    pub open: String,
    pub high: String,
    pub low: String,
    pub close: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IndexPriceKlineSummary {
    pub symbol: String,
    pub category: String,
    pub list: Vec<IndexPriceKline>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IndexPriceKlineResponse {
    pub result: IndexPriceKlineSummary,
    pub time: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PremiumIndexPriceKline {
    pub start_time: i64,
    pub open: String,
    pub high: String,
    pub low: String,
    pub close: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PremiumIndexPriceKlineSummary {
    pub symbol: String,
    pub category: String,
    pub list: Vec<PremiumIndexPriceKline>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PremiumIndexPriceKlineResponse {
    pub result: PremiumIndexPriceKlineSummary,
    pub time: i64,
}

#[derive(Debug,Serialize, Deserialize, Clone)]
pub struct SymbolInformation {
    pub result: SymbolResult,
    pub time: i64,
}

#[derive(Debug,Serialize, Deserialize, Clone)]
#[serde(tag = "category", rename_all = "lowercase")]
pub enum Symbol {
    Linear(LinearSymbol),
    Spot(SpotSymbol),
    Option(OptionSymbol),
}

#[derive(Debug,Serialize, Deserialize, Clone)]
pub struct SymbolResult {
    category: String,
    list: Vec<Ticker>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LinearSymbol {
    pub symbol: String,
    pub contract_type: String,
    pub status: String,
    pub base_coin: String,
    pub quote_coin: String,
    pub launch_time: String,
    pub delivery_time: String,
    pub delivery_fee_rate: String,
    pub price_scale: String,
    pub leverage_filter: LeverageFilter,
    pub price_filter: PriceFilter,
    pub lot_size_filter: LotSizeFilter,
    pub unified_margin_trade: bool,
    pub funding_interval: i64,
    pub settle_coin: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OptionSymbol {
    pub symbol: String,
    pub status: String,
    pub base_coin: String,
    pub settle_coin: String,
    pub launch_time: String,
    pub delivery_time: String,
    pub delivery_fee_rate: String,
    pub price_filter: PriceFilter,
    pub lot_size_filter: LotSizeFilter,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SpotSymbol {
    pub symbol: String,
    pub base_coin: String,
    pub quote_coin: String,
    pub innovation: String,
    pub status: String,
    pub margin_trading: String,
    pub lot_size_filter: LotSizeFilter,
    pub price_filter: PriceFilter,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LeverageFilter {
    pub min_leverage: String,
    pub max_leverage: String,
    pub leverage_step: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PriceFilter {
    pub min_price: String,
    pub max_price: String,
    pub tick_size: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LotSizeFilter {
    pub max_order_qty: String,
    pub min_order_qty: String,
    pub qty_step: String,
    pub post_only_max_order_qty: String,
    pub base_precision: String,
    pub quote_precision: String,
    pub min_order_amt: String,
    pub max_order_amt: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct OrderBookResponse {
    pub result: OrderBook,
    pub time: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderBook {
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "a")]
    pub asks: Vec<Asks>,
    #[serde(rename = "b")]
    pub bids: Vec<Bids>,
    #[serde(rename = "ts")]
    pub timestamp: i64,
    #[serde(rename = "u")]
    pub last_update_id: u64,
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct Bids {
    #[serde(with = "string_or_float")]
    pub price: f64,
    #[serde(with = "string_or_float")]
    pub qty: f64,
}

impl Bids {
    pub fn new(price: f64, qty: f64) -> Bids {
        Bids { price, qty }
    }
}

#[derive(PartialEq,Debug, Serialize, Deserialize, Clone)]
pub struct Asks {
    #[serde(with = "string_or_float")]
    pub price: f64,
    #[serde(with = "string_or_float")]
    pub qty: f64,
}
impl Asks {
    pub fn new(price: f64, qty: f64) -> Asks {
        Asks { price, qty }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "category", rename_all = "lowercase")]
pub enum Ticker {
    Linear(Vec<LinearCategoryItem>),
    Spot(Vec<SpotCategoryItem>),
    Option(Vec<OptionCategoryItem>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TickerResult {
    category: String,
    list: Vec<Ticker>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TickerResponse {
    result: TickerResult,
    time: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LinearCategoryItem {
    pub symbol: String,
    pub last_price: String,
    pub index_price: String,
    pub mark_price: String,
    pub prev_price_24h: String,
    pub price_24h_pcnt: String,
    pub high_price_24h: String,
    pub low_price_24h: String,
    pub prev_price_1h: String,
    pub open_interest: String,
    pub open_interest_value: String,
    pub turnover_24h: String,
    pub volume_24h: String,
    pub funding_rate: String,
    pub next_funding_time: String,
    pub predicted_delivery_price: String,
    pub basis_rate: String,
    pub delivery_fee_rate: String,
    pub delivery_time: String,
    pub ask1_size: String,
    pub bid1_price: String,
    pub ask1_price: String,
    pub bid1_size: String,
    pub basis: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SpotCategoryItem {
    pub symbol: String,
    pub bid1_price: String,
    pub bid1_size: String,
    pub ask1_price: String,
    pub ask1_size: String,
    pub last_price: String,
    pub prev_price_24h: String,
    pub price_24h_pcnt: String,
    pub high_price_24h: String,
    pub low_price_24h: String,
    pub turnover_24h: String,
    pub volume_24h: String,
    pub usd_index_price: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OptionCategoryItem {
    symbol: String,
    bid1_price: String,
    bid1_size: String,
    bid1_iv: String,
    ask1_price: String,
    ask1_size: String,
    ask1_iv: String,
    last_price: String,
    high_price_24h: String,
    low_price_24h: String,
    mark_price: String,
    index_price: String,
    mark_iv: String,
    underlying_price: String,
    open_interest: String,
    turnover_24h: String,
    volume_24h: String,
    total_volume: String,
    total_turnover: String,
    delta: String,
    gamma: String,
    vega: String,
    theta: String,
    predicted_delivery_price: String,
    change_24h: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FundingResponse {
    pub result: Funding,
    pub time: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Funding {
    pub symbol: String,
    pub funding_time: String,
    pub funding_rate_timestamp: String,
} 

#[derive(Debug,Serialize, Deserialize, Clone)]
pub struct RecentTrades {
    pub result: Vec<Trades>,
    pub time: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Trades {
    pub category: String,
    pub trades: Vec<Trade>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Trade {
    Id(String),
    Symbol(String),
    Price(String),
    Qty(String),
    Side(String),
    Time(String),
    IsBlockTrade(bool),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenInterest {
    pub result: OpenInterestResult,
    pub time: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenInterestResult {
    pub symbol: String,
    pub category: String,
    pub list: Vec<OpenInterestItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenInterestItem {
    pub open_interest: String,
    pub timestamp: String,
}

#[derive(Debug,Serialize, Deserialize, Clone)]
pub struct HistoricalVolatility {
    pub category: String,
    pub result: Vec<HistoricalVolatilityItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HistoricalVolatilityItem {
    pub period: i32,
    pub value: String,
    pub time: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InsuranceResponse {
    pub result: InsuranceResult,
    pub time: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InsuranceResult {
    pub updated_time: String,
    pub list: Vec<InsuranceItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InsuranceItem {
    pub coin: String,
    pub balance: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RiskLimit {
    pub category: String,
    pub list: Vec<RiskLimitItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RiskLimitItem {
    pub id: i32,
    pub symbol: String,
    pub risk_limit_value: String,
    pub maintenance_margin: String,
    pub initial_margin: String,
    #[serde(with = "string_or_bool")]
    pub is_lowest_risk: bool,
    pub max_leverage: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RiskLimitResponse {
    pub result: RiskLimit,
    pub time: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeliveryPriceResponse {
    ret_code: i32,
    ret_msg: String,
    result: DeliveryPriceResult,
    ret_ext_info: serde_json::Value,
    time: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeliveryPriceResult {
    category: String,
    next_page_cursor: String,
    list: Vec<DeliveryPriceItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeliveryPriceItem {
    symbol: String,
    #[serde(with = "string_or_float_opt")]
    delivery_price: Option<f64>,
    delivery_time: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LongShortRatioResponse {
    ret_code: i32,
    ret_msg: String,
    result: LongShortRatioResult,
    ret_ext_info: serde_json::Value,
    time: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LongShortRatioResult {
    list: Vec<LongShortRatioItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LongShortRatioItem {
    symbol: String,
    #[serde(with = "string_or_float")]
    buy_ratio: f64,
    #[serde(with = "string_or_float")]
    sell_ratio: f64,
    #[serde(with = "string_or_i64")]
    timestamp: i64,
}



pub(crate) mod string_or_float {
    use std::fmt;

    use serde::{de, Deserialize, Deserializer, Serializer};

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: fmt::Display,
        S: Serializer,
    {
        serializer.collect_str(value)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<f64, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringOrFloat {
            String(String),
            Float(f64),
        }

        match StringOrFloat::deserialize(deserializer)? {
            StringOrFloat::String(s) => {
                if s == "INF" {
                    Ok(f64::INFINITY)
                } else {
                    s.parse().map_err(de::Error::custom)
                }
            }
            StringOrFloat::Float(i) => Ok(i),
        }
    }
}

pub(crate) mod string_or_float_opt {
    use std::fmt;

    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<T, S>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: fmt::Display,
        S: Serializer,
    {
        match value {
            Some(v) => crate::model::string_or_float::serialize(v, serializer),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringOrFloat {
            String(String),
            Float(f64),
        }

        Ok(Some(crate::model::string_or_float::deserialize(
            deserializer,
        )?))
    }
}

pub(crate) mod string_or_bool {
    use std::fmt;

    use serde::{de, Deserialize, Deserializer, Serializer};

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: fmt::Display,
        S: Serializer,
    {
        serializer.collect_str(value)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringOrFloat {
            String(String),
            Bool(bool),
        }

        match StringOrFloat::deserialize(deserializer)? {
            StringOrFloat::String(s) => s.parse().map_err(de::Error::custom),
            StringOrFloat::Bool(i) => Ok(i),
        }
    }
}
