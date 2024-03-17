use bybit::api::*;
use bybit::model::*;
use bybit::trade::*;
use tokio;

#[cfg(test)]
mod tests {

    use super::*;

    static API_KEY: &str = ""; //Mockup string
    static SECRET: &str = ""; // Mockup string

    #[tokio::test]
    async fn test_trade() {
        let trade: Trader = Bybit::new(Some(API_KEY.to_string()), Some(SECRET.to_string()));
        let order = trade
            .place_futures_limit_order(Category::Linear, "MATICUSDT", Side::Buy, 100.0, 0.7500, 0)
            .await;
        println!("{:#?}", order);
    }

    #[tokio::test]
    async fn test_order_history() {
        let trade: Trader = Bybit::new(Some(API_KEY.to_string()), Some(SECRET.to_string()));
        let data: OrderHistoryRequest = OrderHistoryRequest::new(
            Category::Linear,
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
        );
        let order_history = trade.get_order_history(data).await;
        println!("{:#?}", order_history);
    }

    #[tokio::test]
    async fn test_trade_history() {
        let trade: Trader = Bybit::new(Some(API_KEY.to_string()), Some(SECRET.to_string()));
        let data: TradeHistoryRequest = TradeHistoryRequest::new(
            Category::Linear,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        );
        let trade_history = trade.get_trade_history(data).await;
        if let Ok(data) = trade_history {
            println!("{:#?}", data.result.list);
        }
    }

    #[tokio::test]
    async fn test_batch() {
        let trade: Trader = Bybit::new(Some(API_KEY.to_string()), Some(SECRET.to_string()));
        let request = vec![
            OrderRequest {
                symbol: "MATICUSDT".into(),
                side: Side::Buy,
                qty: 100.0,
                order_type: OrderType::Market,
                ..Default::default()
            },
            OrderRequest {
                symbol: "BTCUSDT".into(),
                side: Side::Buy,
                qty: 100.0,
                order_type: OrderType::Market,
                ..Default::default()
            },
        ];
        let data: BatchPlaceRequest = BatchPlaceRequest::new(Category::Linear, request);
        let batch = trade.batch_place_order(data).await;
        println!("{:#?}", batch);
    }
}
