use bybit_rs::api::*;
use bybit_rs::config::*;
use bybit_rs::general::General;
use bybit_rs::market::*;
use bybit_rs::model::{Category, InstrumentRequest, KlineRequest, OrderbookRequest};
use tokio;
use tokio::time::{Duration, Instant};

#[cfg(test)]
mod tests {

    use super::*;
    use bybit_rs::{
        model::{
            FundingHistoryRequest, HistoricalVolatilityRequest, OpenInterestRequest,
            RecentTradesRequest, RiskLimitRequest,
        },
        util::get_timestamp,
    };

    #[tokio::test]
    async fn test_time() {
        let market: General = Bybit::new(None, None);
        let timestamp = get_timestamp();
        let time = market.get_server_time().await;
        println!("{:#?} , {:?}", time, timestamp);
    }
    #[tokio::test]
    async fn test_kline() {
        let market: MarketData = Bybit::new(None, None);
        let request = KlineRequest::new(
            Some(Category::Linear),
            "MATICUSDT",
            "60",
            Some("010124"),
            Some("050224"),
            None,
        );
        let premium = market.get_klines(request).await;
        if let Ok(data) = premium {
            println!("{:#?}", data.list);
        }
    }

    #[tokio::test]
    async fn test_instrument() {
        let market: MarketData = Bybit::new(None, None);
        let request = InstrumentRequest::new(Category::Linear, Some("MATICUSDT"), None, None, None);
        let instrument = market.get_futures_instrument_info(request.clone()).await;
        if let Ok(data) = instrument {
            println!("{:#?}", data[0]);
        }
        let spot_instrument = market.get_spot_instrument_info(request).await;
        if let Ok(data) = spot_instrument {
            println!("{:#?}", data[0]);
        }
    }

    #[tokio::test]
    async fn test_market() {
        let market: MarketData = Bybit::new(None, None);
        let five_minutes = Duration::from_secs(5 * 60);
        let request = OrderbookRequest::new("GALUSDT", Category::Linear, Some(1));
        let start = Instant::now();
        while Instant::now() - start < five_minutes {
            let order_book = market.get_depth(request.clone()).await;
            if let Ok(order_book) = order_book {
                let mid_price = (order_book.asks[0].price + order_book.bids[0].price) / 2.0;
                let imbalance = (order_book.bids[0].qty - order_book.asks[0].qty) / (order_book.asks[0].qty + order_book.bids[0].qty);
                let fees = fee_percent(mid_price, 0.04);
                let spread = order_book.asks[0].price - order_book.bids[0].price;
                let arb = spread - fees;
                println!(
                    "{:#?} , Spread: {:.5} Arb: {} Imb: {:.4}",
                    order_book,
                    spread,
                    if arb > fee_percent(mid_price, 0.02) {
                        arb
                    } else {
                        0.0
                    },
                    imbalance
                );
            }
        }
    }


    fn fee_percent(value: f64, percent: f64) -> f64 {
        (percent / 100.0) * value
    }

    #[tokio::test]
    async fn test_ticker() {
        let market: MarketData = Bybit::new(None, None);
        let symbol = "MATICUSDT";
        let ticker = market.get_futures_tickers(Some(symbol)).await;
        if let Ok(data) = ticker {
            println!("{:#?}", data[0]);
        }
        let spot_ticker = market.get_spot_tickers(Some(symbol)).await;
        if let Ok(data) = spot_ticker {
            println!("{:#?}", data[0]);
        }
    }

    #[tokio::test]
    async fn test_recent_trades() {
        let market: MarketData = Bybit::new(None, None);
        let request = RecentTradesRequest::new(Category::Linear, Some("MATICUSDT"), None, None);
        let trades = market.get_recent_trades(request).await;
        if let Ok(data) = trades {
            println!("{:#?}", data);
        }
    }

    #[tokio::test]
    async fn test_funding_rate() {
        let market: MarketData = Bybit::new(None, None);
        let symbol = "BTCUSDT";
        let request = FundingHistoryRequest::new(Category::Linear, symbol, None, None, None);
        let funding_rate = market.get_funding_history(request).await;
        if let Ok(data) = funding_rate {
            println!("{:#?}", data.last().unwrap());
        }
    }

    #[tokio::test]
    async fn test_open_interest() {
        let market: MarketData = Bybit::new(None, None);
        let request =
            OpenInterestRequest::new(Category::Linear, "MATICUSDT", "4h", None, None, None);
        let open_interest = market.get_open_interest(request).await;
        if let Ok(data) = open_interest {
            println!("{:#?}", data.list.last().unwrap());
        }
    }

    #[tokio::test]
    async fn test_historical_volatility() {
        let market: MarketData = Bybit::new(None, None);
        let symbol = "ETH";
        let request: HistoricalVolatilityRequest<'_> =
            HistoricalVolatilityRequest::new(Some(symbol), None, None, None);
        let historical_volatility = market.get_historical_volatility(request).await;
        if let Ok(data) = historical_volatility {
            println!("{:#?}", data);
        }
    }

    #[tokio::test]
    async fn test_insurance() {
        let market: MarketData = Bybit::new(None, None);
        let symbol = Some("BTC");
        let insurance = market.get_insurance(symbol).await;
        if let Ok(data) = insurance {
            println!("{:#?}", data);
        }
    }

    #[tokio::test]
    async fn test_risk_limit() {
        let market: MarketData =
            Bybit::new_with_config(&Config::default().set_recv_window(1000), None, None);
        let symbol = "MATICUSDT";
        let request: RiskLimitRequest<'_> = RiskLimitRequest::new(Category::Linear, Some(symbol));
        let risk_limit = market.get_risk_limit(request).await;
        if let Ok(data) = risk_limit {
            println!("{:#?}", data);
        }
    }

    #[tokio::test]
    async fn test_delivery_price() {
        let market: MarketData = Bybit::new(None, None);
        let symbol = "BTCUSDT";
        let delivery_price = market
            .get_delivery_price(Category::Linear, Some(symbol), None, None)
            .await;
        println!("{:#?}", delivery_price);
    }

    #[tokio::test]
    async fn test_longshort_ratio() {
        let market: MarketData = Bybit::new(None, None);
        let symbol = "BTCUSDT";
        let longshort_ratio = market
            .get_longshort_ratio(Category::Linear, symbol, "4h", None)
            .await;
        if let Ok(data) = longshort_ratio {
            println!("{:#?}", data);
        }
    }
}
