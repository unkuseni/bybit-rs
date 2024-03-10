use bybit::api::*;
use bybit::model::*;
use tokio;

#[cfg(test)]
mod tests {
    use bybit::account::AccountManager;

    use super::*;
    static API_KEY: &str = ""; //Mockup string
    static SECRET: &str = ""; // Mockup string

    #[tokio::test]
    async fn test_wallet() {
        let account: AccountManager =
            Bybit::new(Some(API_KEY.to_string()), Some(SECRET.to_string()));
        let wallet = account.get_wallet_balance("UNIFIED", None).await;

        println!("{:?}", wallet);
    }

    #[tokio::test]
    async fn test_fee_rate() {
        let account: AccountManager =
            Bybit::new(Some(API_KEY.to_string()), Some(SECRET.to_string()));
        let wallet = account.get_fee_rate(Category::Linear, Some("BTCUSDT".to_string())).await;

        println!("{:?}", wallet);
    }

    #[tokio::test]
    async fn test_borrow_history() {
        let account: AccountManager =
            Bybit::new(Some(API_KEY.to_string()), Some(SECRET.to_string()));
        let wallet = account.get_fee_rate(Category::Spot, None).await;

        println!("{:?}", wallet);
    }
}
