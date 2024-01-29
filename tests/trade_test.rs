use bybit_rs::api::*;
use bybit_rs::model::*;
use bybit_rs::trade::*;
use tokio;

#[cfg(test)]
mod tests {
    use super::*;

    static API_KEY: &str = "ckvcnbvnbnjklvclibjnj"; //Mockup string
    static SECRET: &str = "rfogobvjkkblvjlcnm"; // Mockup string

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
        let data: TradeHistoryRequest =
            TradeHistoryRequest::new(Category::Linear, None, None, None, None, None, None, None, None);
        let trade_history = trade.get_trade_history(data).await;
        if let Ok(data) = trade_history {
            println!("{:#?}", data.list);
        }
    }
}
