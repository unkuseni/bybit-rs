use bybit::api::*;
use tokio;

#[cfg(test)]
mod tests {

    use bybit::{
        model::{Category, Subscription, Tickers, WebsocketEvents},
        ws::Stream,
    };
    use tokio::time::Instant;

    use super::*;

    static API_KEY: &str = ""; //Mockup string
    static SECRET: &str = ""; // Mockup string

    #[tokio::test]
    async fn test_auth() {
        let ws: Stream = Bybit::new(Some(API_KEY.to_string()), Some(SECRET.to_string()));
        ws.ws_wallet().await.unwrap();
    }

    #[tokio::test]
    async fn ping() {
        let ws: Stream = Bybit::new(None, None);
        let response = ws.ws_ping(true).await;
        println!("{:#?}", response);
    }

    #[tokio::test]
    async fn test_order_book() {
        let ws: Stream = Bybit::new(None, None);
        let request = Subscription {
            args: vec!["publicTrade.ADAUSDT"],
            op: "subscribe",
        };

        let response = ws.ws_subscribe(request, Category::Linear, |event| {
            match event {
                WebsocketEvents::TradeEvent(trade) => {
                    // Handle Trade
                    for v in trade.data {
                        println!(
                            "Volume: {:.3} USD, Timestamp: {}, Side: {} Time:{}",
                            v.volume * v.price,
                            v.timestamp / 6000,
                            v.side,
                            Instant::now().elapsed().as_nanos()
                        );
                    }
                }
                WebsocketEvents::OrderBookEvent(order_book) => {
                    println!("{:#?}", order_book.data);
                    // Handle OrderBook event
                }
                // Add additional matches for other variants of the WebsocketEvents enum
                WebsocketEvents::TickerEvent(ticker) => {
                    // Handle Ticker event
                    match ticker.data {
                        Tickers::Linear(linear_ticker) => {
                            println!("{:#?}", linear_ticker);
                        }
                        Tickers::Spot(spot_ticker) => {
                            println!("{:#?}", spot_ticker);
                        }
                    }
                }
                WebsocketEvents::KlineEvent(kline) => {
                    // Handle Kline
                    for v in kline.data {
                        println!("{:#?}", v);
                    }
                }
                WebsocketEvents::LiquidationEvent(liquidation) => {
                    // Handle Liquidation
                    println!("{:#?}", liquidation.data);
                }
                _ => {
                    
                }
            };
            Ok(())
        }).await;
        println!("{:#?}", response);
    }

    #[tokio::test]
    async fn test_default_orderbook() {
        let ws: Stream = Bybit::new(None, None);
        let request = vec![(1, "MATICUSDT")];
        ws.ws_orderbook(request, Category::Linear).await.unwrap();
    }

    #[tokio::test]
    async fn test_default_trades() {
        let ws: Stream = Bybit::new(None, None);
        let request = vec!["BTCUSDT", "MATICUSDT", "ETHUSDT", "ADAUSDT"];
        ws.ws_trades(request, Category::Linear).await.unwrap();
    }

    #[tokio::test]
    async fn test_default_tickers() {
        let ws: Stream = Bybit::new(None, None);
        let request = vec!["ADAUSDT", "MATICUSDT"];
        ws.ws_tickers(request, Category::Spot).await.unwrap();
    }

    #[tokio::test]
    async fn test_default_klines() {
        let ws: Stream = Bybit::new(None, None);
        let request = vec![("1", "MATICUSDT")];
        ws.ws_klines(request, Category::Linear).await.unwrap();
    }
}
