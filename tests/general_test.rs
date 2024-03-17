use bybit::api::Bybit;

mod tests {
    use super::*;
    use bybit::general::General;
    use tokio::test;

    #[test]
    async fn test_time() {
        let general: General = Bybit::new(None, None);
        match general.get_server_time().await {
            Ok(data) => println!("{:#?}", data),
            Err(err) => println!("{:#?}", err),
        }
        match general.ping().await {
            Ok(data) => println!("{:#?}", data),
            Err(err) => println!("{:#?}", err),
        }
    }
}
