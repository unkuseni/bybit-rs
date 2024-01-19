use crate::client::Client;
use crate::config::Config;

pub enum API {
    Spot(Spot),
    Linear(Linear),
    Inverse(Inverse),
    Option(Option),
}
/// Bybit Endpoints

pub enum Spot {
    Market(Market),
    Order(Order),
    Position(Position),
    Account(Account),
    Asset(Asset),
    SpotLeverage(SpotLeverage),
    SpotMargin(SpotMargin),
}

pub enum Linear {
    Market(Market),
    Order(Order),
    Position(Position),
    Account(Account),
    Asset(Asset),
}
pub enum Inverse {
    Market(Market),
    Order(Order),
    Position(Position),
    Account(Account),
    Asset(Asset),
    
}
pub enum Option {
    Market(Market),
    Order(Order),
    Position(Position),
    Account(Account),
    Asset(Asset),
}

enum Market {
    Time,
    Kline,
    MarkPriceKline,
    IndexPriceKline,
    PremiumIndexPriceKline,
    InstrumentsInfo,
    OrderBook,
    Tickers,
    Trades,
    FundingRate,
    RecentTrades,
    OpenInterest,
    HistoricalVolatility,
    Insurance,
    RiskLimit,
    DeliveryPrice,
    LongShortRatio,
}

enum Order {}

enum Position {}

enum Account {}

enum Asset {}

enum SpotLeverage {}

enum SpotMargin {}


impl From<API> for String {
    fn from(item: API) -> Self {
        String::from(match item {
            API::Spot(Segment) => match Segment {
                Spot::Market(route) => match route {
                    Market::Time => "/v5/market/time?",
                    Market::Kline => "/v5/market/kline?catergory=spot&",
                },
                Spot::Order(route) => match route {},
                Spot::Position(route) => match route {},
                Spot::Account(route) => match route {},
                Spot::Asset(route) => match route {},
                Spot::SpotLeverage => match route {},
                Spot::SpotMargin => match route {},
            },
            API::Linear(Segment) => match Segment {
                Linear::Market(route) => match route {
                    Market::Time => "/v5/market/time?",
                    Market::Kline => "/v5/market/kline?category=linear&"
                    Market::MarkPriceKline => "/v5/market/mark-price-kline?category=linear&",


                },
                Linear::Order(route) => match route {},
                Linear::Position(route) => match route {},
                Linear::Account(route) => match route {},
                Linear::Asset(route) => match route {},

            },
            API::Option(Segment) => match Segment {
                Option::Market(route) => match route {},
                Option::Order(route) => match route {},
                Option::Position(route) => match route {},
                Option::Account(route) => match route {},
                Option::Asset(route) => match route {},
            },
        })
    }
}
