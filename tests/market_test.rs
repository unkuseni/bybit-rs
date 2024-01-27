use bybit_rs::api::*;
use bybit_rs::market::*;
use bybit_rs::general::General;
use bybit_rs::config::*;
use bybit_rs::model::{Category, KlineRequest, InstrumentRequest, OrderbookRequest};
use tokio;
use tokio::time::{Duration, Instant};

#[cfg(test)]
mod tests {

    use super::*;
    use bybit_rs::{model::{HistoricalVolatilityRequest, OpenInterestRequest, RecentTradesRequest, RiskLimitRequest, FundingHistoryRequest}, util::get_timestamp};

    #[tokio::test]
    async fn test_time() {
        let market: General = Bybit::new(None, None);
        let time = market.get_server_time().await;
        println!("{:#?} , {:?}",  time, get_timestamp());
    }
    #[tokio::test]
    async fn test_kline() {
        let market: MarketData = Bybit::new(None, None);
        let request = KlineRequest::new(Some(Category::Linear), "MATICUSDT", "1h", None, None, None);
        let premium = market
            .get_klines(
                request
            )
            .await;
        println!("{:#?}", premium);
    }

    #[tokio::test]
    async fn test_instrument() {
        let market: MarketData = Bybit::new(None, None);
        let request = InstrumentRequest::new(Category::Linear, Some("MATICUSDT"), None, None, None);
        let instrument = market
            .get_futures_instrument_info(request.clone())
            .await;
        println!("{:#?}", instrument);
        let spot_instrument = market
            .get_spot_instrument_info(request)
            .await;
        println!("{:#?}", spot_instrument);
    }

    #[tokio::test]
    async fn test_market() {
        let market: MarketData = Bybit::new(None, None);
        let _five_minutes = Duration::from_secs(5 * 60);
        let request = OrderbookRequest::new("SKLUSDT", Category::Linear, None);
        let _start = Instant::now();
        // while Instant::now() - start < five_minutes {
            let order_book = market.get_depth(request).await;
            assert!(order_book.is_ok());
            if let Ok(order_book) = order_book {
                let imbalance = order_book.asks[0].price - order_book.bids[0].price;
                println!("{:#?} , Imbalance: {:.5}", order_book, imbalance);
            }
        //     time::sleep(Duration::from_millis(600)).await;
        // }
    }
    #[tokio::test]
    async fn test_ticker() {
        let market: MarketData = Bybit::new(None, None);
        let symbol = "BTCUSDT";
        let ticker = market.get_futures_tickers(Some(symbol)).await;
        println!("{:#?}", ticker);
        let spot_ticker = market.get_spot_tickers(Some(symbol)).await;
        println!("{:#?}", spot_ticker);
    }

    #[tokio::test]
    async fn test_recent_trades() {
        let market: MarketData = Bybit::new(None, None);
        let request = RecentTradesRequest::new(Category::Linear, Some("MATICUSDT"), None, None);
        let trades = market
            .get_recent_trades(request)
            .await;
        println!("{:#?}", trades);
    }

    #[tokio::test]
    async fn test_funding_rate() {
        let market: MarketData = Bybit::new(None, None);
        let symbol = "BTCUSDT";
        let request = FundingHistoryRequest::new(Category::Linear, symbol, None, None, None);
        let funding_rate = market.get_funding_history(request)
            .await;
        println!("{:#?}", funding_rate);
    }

    #[tokio::test]
    async fn test_open_interest() {
        let market: MarketData = Bybit::new(None, None);
        let request = OpenInterestRequest::new(Category::Linear, "MATICUSDT", "4h", None, None, None);
        let open_interest = market
            .get_open_interest(request)
            .await;
        println!("{:#?}", open_interest);
    }

    #[tokio::test]
    async fn test_historical_volatility() {
        let market: MarketData = Bybit::new(None, None);
        let symbol = "ETH";
        let request: HistoricalVolatilityRequest<'_> = HistoricalVolatilityRequest::new(Some(symbol), None, None, None);
        let historical_volatility = market.get_historical_volatility(request).await;
        println!("{:#?}", historical_volatility);
    }

    #[tokio::test]
    async fn test_insurance() {
        let market: MarketData = Bybit::new(None, None);
        let symbol = Some("BTC");
        let insurance = market.get_insurance(symbol).await;
        println!("{:#?}", insurance);
    }

    #[tokio::test]
    async fn test_risk_limit() {
        let market: MarketData = Bybit::new_with_config(&Config::default().set_recv_window(1000), None, None);
        let symbol = "MATICUSDT";
        let request: RiskLimitRequest<'_> = RiskLimitRequest::new(Category::Linear, Some(symbol));
        let risk_limit = market.get_risk_limit(request).await;
        println!("{:#?}", risk_limit);
    }

    #[tokio::test]
    async fn test_delivery_price() {
        let market: MarketData = Bybit::new(None, None);
        let symbol = "BTCUSDT";
        let delivery_price = market.get_delivery_price(Category::Linear, Some(symbol), None, None).await;
        println!("{:#?}", delivery_price);
    }

    #[tokio::test]
    async fn test_longshort_ratio() {
        let market: MarketData = Bybit::new(None, None);
        let symbol = "BTCUSDT";
        let longshort_ratio = market.get_longshort_ratio(Category::Linear, symbol, "4h", None).await;
        println!("{:#?}", longshort_ratio);
    }
}
