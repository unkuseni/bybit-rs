use bybit_rs::api::*;
use bybit_rs::market::*;
use bybit_rs::general::*;
use std::borrow::Cow;
use tokio;
use tokio::time::{self, Duration, Instant};

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn test_market() {
        let market: MarketData = Bybit::new(None, None);
        let five_minutes = Duration::from_secs(5 * 60);
        let start = Instant::now();
        while Instant::now() - start < five_minutes {
            let order_book = market
                .get_depth("MATICUSDT", Category::Linear, Some(10))
                .await;
            assert!(order_book.is_ok());
            if let Ok(order_book) = order_book {
                println!("{:?}", order_book);
            }
            time::sleep(Duration::from_secs(2)).await;
        }
    }
    #[tokio::test]
async fn test_time() {
    let general: MarketData = Bybit::new(None, None);
    match general
        .get_index_price_klines(Some(Category::Linear), "MATICUSDT".into(), "1".into(), None, None, Some(10))
        .await {
        Ok(time) => println!("{:#?}", time),
        Err(e) => eprintln!("Error: {:?}", e), // This will print if there's an error
    }
}
}
