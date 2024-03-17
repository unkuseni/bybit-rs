mod tests {
    use bybit::{
        api::*,
        model::{Category, LeverageRequest, PositionRequest},
        position::PositionManager,
    };
    use tokio::test;

    static API_KEY: &str = "api_key";
    static SECRET_KEY: &str = "secret_key";

    #[test]
    async fn position_info() {
        let position: PositionManager =
            Bybit::new(Some(API_KEY.to_string()), Some(SECRET_KEY.to_string()));
        let request = PositionRequest::new(Category::Linear, Some("BTCUSDT"), None, None, None);
        match position.get_info(request).await {
            Ok(data) => println!("{:?}", data),
            Err(e) => println!("{:?}", e),
        }
    }

    #[test]
    async fn set_leverage() {
        let position: PositionManager =
            Bybit::new(Some(API_KEY.to_string()), Some(SECRET_KEY.to_string()));
        let request = LeverageRequest::new(Category::Linear, "BTCUSDT", 10);
        match position.set_leverage(request).await {
            Ok(data) => println!("{:?}", data),
            Err(e) => println!("{:?}", e),
        }
    }
    
}
