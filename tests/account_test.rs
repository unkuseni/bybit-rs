use bybit::prelude::*;

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;
    static API_KEY: &str = ""; //Mockup string
    static SECRET: &str = ""; // Mockup string

    #[test]
    async fn test_wallet() {
        let account: AccountManager = Bybit::new(Some(API_KEY.into()), Some(SECRET.into()));
        match account.get_wallet_balance("UNIFIED", None).await {
            Ok(v) => println!("{:#?}", v),
            Err(e) => println!("{:#?}", e),
        }
    }

    #[test]
    async fn test_fee_rate() {
        let account: AccountManager = Bybit::new(Some(API_KEY.into()), Some(SECRET.into()));
        match account
            .get_fee_rate(Category::Spot, Some("BTCUSDT".to_string()))
            .await
        {
            Ok(v) => println!("{:#?}", v),
            Err(e) => println!("{:#?}", e),
        }
    }

    #[test]
    async fn test_borrow_history() {
        let account: AccountManager = Bybit::new(Some(API_KEY.into()), Some(SECRET.into()));
        let wallet = account.get_fee_rate(Category::Spot, None).await;

        println!("{:?}", wallet);
    }
}
