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
    Trade(Trade),
    Position(Position),
    Account(Account),
    Asset(Asset),
    SpotLeverage(SpotLeverage),
    SpotMargin(SpotMargin),
}

pub enum Linear {
    Market(Market),
    Trade(Trade),
    Position(Position),
    Account(Account),
    Asset(Asset),
}
pub enum Inverse {
    Market(Market),
    Trade(Trade),
    Position(Position),
    Account(Account),
    Asset(Asset),
}
pub enum Option {
    Market(Market),
    Trade(Trade),
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

enum Trade {
    Place,
    Amend,
    Cancel,
    OpenOrders,
    CancelAll,
    History,
    BatchPlace,
    BatchAmend,
    BatchCancel,
    SpotBorrowCheck,
}

enum Position {
    Information,
    SetLeverage,
    SetRiskLimit,
    SetTradingStop,
    SwitchIsolated,
    SwitchMode,
    SetAutoaddMargin,
    AddorReduceMargin,
    ClosedPnl,
    History,
}

enum Account {
    Balance,
    UpgradetoUTA,
    BorrowHistory,
    CollateralInfo,
    CoinGreeks,
    Information,
    TransactionLog,
    SetMarginMode
}

enum Asset {
    DeliveryRecord,
    SettlementRecord,
    Intertransfer,
    QueryTransferList,
    SaveTransferSubmember,
    UniversalTransfer,
    QueryUniversalTransferList,
    QueryTransferCoinList,
    QueryTransferSubmemberList,
    QueryAccountCoinBalance,
    QueryAssetInfo,
    QueryAllowedList,
    QueryRecord,
    QueryInfo,
    QueryAsset,
    Withdraw,
    CanceWithdraw,
    Deposit,
    QuerySubmemberAddress,
    OrderRecord,
}

enum SpotLeverage {
    Info,
    Reference,
    Purchase,
    Redeem,
    OrderRecord
}

enum SpotMargin {
    SwitchMode,
    SetLeverage,
    SetPledgeToken
}

impl From<API> for String {
    fn from(item: API) -> Self {
        String::from(match item {
            API::Spot(Segment) => match Segment {
                Spot::Market(route) => match route {
                    Market::Time => "/v5/market/time?",
                    Market::Insurance => "/v5/market/insurance?",
                    Market::Kline => "/v5/market/kline?catergory=spot&",
                    Market::InstrumentsInfo => "/v5/market/instruments-info?category=spot&",
                    Market::OrderBook => "/v5/market/orderbook?category=spot&",
                    Market::Tickers => "/v5/market/tickers?category=spot&",
                    Market::RecentTrades => "/v5/market/trades?category=spot&",
                    _ => Err("Endpoint does not exist".to_string()),
                },
                Spot::Order(route) => match route {
                    Trade::Place => "/v5/order/create?",
                    Trade::Amend => "/v5/order/amend?",
                    Trade::Cancel => "/v5/order/cancel?",
                    Trade::OpenOrders => "/v5/order/realtime?category=spot&",
                    Trade::CancelAll => "/v5/order/cancel-all?",
                    Trade::History => "/v5/order/history?category=spot&",
                },
                Spot::Position(route) => match route {
                    Position::History => "/v5/execution/list?category=spot&",
                },
                Spot::Account(route) => match route {},
                Spot::Asset(route) => match route {},
                Spot::SpotLeverage(route) => match route {},
                Spot::SpotMargin(route) => match route {},
            },
            API::Linear(Segment) => match Segment {
                Linear::Market(route) => match route {
                    Market::Time => "/v5/market/time?",
                    Market::Insurance => "/v5/market/insurance?",
                    Market::Kline => "/v5/market/kline?category=linear&",
                    Market::MarkPriceKline => "/v5/market/mark-price-kline?category=linear&",
                    Market::IndexPriceKline => "/v5/market/index-price-kline?category=linear&",
                    Market::PremiumIndexPriceKline => {
                        "/v5/market/premium-index-price-kline?category=linear&"
                    }
                    Market::InstrumentsInfo => "/v5/market/instruments-info?category=linear&",
                    Market::OrderBook => "/v5/market/orderbook?category=linear&",
                    Market::Tickers => "/v5/market/tickers?category=linear&",
                    Market::FundingRate => "/v5/market/funding/history?category=linear&",
                    Market::RecentTrades => "/v5/market/recent-trades?category=linear&",
                    Market::OpenInterest => "/v5/market/open-interest?category=linear&",
                    Market::RiskLimit => "/v5/market/risk-limit?category=linear&",
                    Market::DeliveryPrice => "/v5/market/delivery-price?category=linear&",
                    Market::LongShortRatio => "/v5/market/account-ratio?category=linear&",
                    _ => Err("Endpoint does not exist".to_string()),
                },
                Linear::Order(route) => match route {
                    Trade::Place => "/v5/order/create?",
                    Trade::Amend => "/v5/order/amend?",
                    Trade::Cancel => "/v5/order/cancel?",
                    Trade::OpenOrders => "/v5/order/realtime?category=linear&",
                    Trade::CancelAll => "/v5/order/cancel-all?",
                    Trade::History => "/v5/order/history?category=linear&",
                    Trade::BatchPlace => "/v5/order/create-batch?",
                    Trade::BatchAmend => "/v5/order/amend-batch?",
                    Trade::BatchCancel => "/v5/order/cancel-batch?",
                },
                Linear::Position(route) => match route {
                    Position::History => "/v5/execution/list?category=linear&",
                },
                Linear::Account(route) => match route {},
                Linear::Asset(route) => match route {},
            },
            API::Inverse(Segment) => match Segment {
                Inverse::Market(route) => match route {
                    Market::Time => "/v5/market/time?",
                    Market::Insurance => "/v5/market/insurance?",
                    Market::Kline => "/v5/market/kline?category=inverse&",
                    Market::MarkPriceKline => "/v5/market/mark-price-kline?category=inverse&",
                    Market::IndexPriceKline => "/v5/market/index-price-kline?category=inverse&",
                    Market::InstrumentsInfo => "/v5/market/instruments-info?category=inverse&",
                    Market::OrderBook => "/v5/market/orderbook?category=inverse&",
                    Market::Tickers => "/v5/market/tickers?category=inverse&",
                    Market::FundingRate => "/v5/market/funding/history?category=inverse&",
                    Market::RecentTrades => "/v5/market/recent-trades?category=inverse&",
                    Market::OpenInterest => "/v5/market/open-interest?category=inverse&",
                    Market::RiskLimit => "/v5/market/risk-limit?category=inverse&",
                    Market::DeliveryPrice => "/v5/market/delivery-price?category=inverse&",
                    Market::LongShortRatio => "/v5/market/account-ratio?category=inverse&",
                    _ => Err("Endpoint does not exist".to_string()),
                },
                Inverse::Order(route) => match route {
                    Trade::Place => "/v5/order/create?",
                    Trade::Amend => "/v5/order/amend?",
                    Trade::Cancel => "/v5/order/cancel?",
                    Trade::OpenOrders => "/v5/order/realtime?category=inverse&",
                    Trade::CancelAll => "/v5/order/cancel-all?",
                    Trade::History => "/v5/order/history?category=inverse&",
                },
                Inverse::Position(route) => match route {
                    Position::History => "/v5/execution/list?category=inverse&",
                },
                Inverse::Account(route) => match route {},
                Inverse::Asset(route) => match route {},
            },
            API::Option(Segment) => match Segment {
                Option::Market(route) => match route {
                    Market::Time => "/v5/market/time?",
                    Market::Insurance => "/v5/market/insurance?",
                    Market::InstrumentsInfo => "/v5/market/instruments-info?category=option&",
                    Market::OrderBook => "/v5/market/orderbook?category=option&",
                    Market::Tickers => "/v5/market/tickers?category=option&",
                    Market::RecentTrades => "/v5/market/trades?category=option&",
                    Market::HistoricalVolatility => {
                        "/v5/market/historical-volatility?category=option&"
                    }
                    Market::DeliveryPrice => "/v5/market/delivery-price?category=option&",
                    _ => Err("Endpoint does not exist".to_string()),
                },
                Option::Order(route) => match route {
                    Trade::Place => "/v5/order/create?",
                    Trade::Amend => "/v5/order/amend?",
                    Trade::Cancel => "/v5/order/cancel?",
                    Trade::OpenOrders => "/v5/order/realtime?category=option&",
                    Trade::CancelAll => "/v5/order/cancel-all?",
                    Trade::History => "/v5/order/history?category=option&",
                    Trade::BatchPlace => "/v5/order/create-batch?",
                    Trade::BatchAmend => "/v5/order/amend-batch?",
                    Trade::BatchCancel => "/v5/order/cancel-batch?",
                },
                Option::Position(route) => match route {
                    Position::History  => "/v5/execution/list?category=option&",
                },
                Option::Account(route) => match route {},
                Option::Asset(route) => match route {},
            },
        })
    }
}
