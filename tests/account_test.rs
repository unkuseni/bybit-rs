
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
            let req = WalletRequest::new("UNIFIED", Some("BTC"));
        let wallet = account.get_wallet_balance(req)
            .await;

        println!("{:?}", wallet);
    }
}
