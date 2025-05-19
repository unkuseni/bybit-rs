use bybit::prelude::*;

/// Module containing unit tests for the Bybit API.
mod tests {
    // Import the parent module.
    use super::*;
    // Import the tokio test module.
    use tokio::test;

    /// Test case that checks the functionality of the `get_server_time` and `ping`
    /// methods of the `Bybit::General` struct.
    ///
    /// This test case creates a new instance of `Bybit::General`, calls
    /// `get_server_time` and `ping` asynchronously, and prints the result or
    /// error.
    #[test]
    async fn test_time() {
        // Create a new instance of `Bybit::General`.
        let general: General = Bybit::new(None, None);

        // Call `get_server_time` asynchronously and match the result.
        match general.get_server_time().await {
            // If the call is successful, print the data.
            Ok(data) => println!("{:#?}", data),
            // If the call fails, print the error.
            Err(err) => println!("{:#?}", err),
        }
    }

    #[test]
    async fn test_ping() {
        let general: General = Bybit::new(None, None);
        // Call `ping` asynchronously and match the result.
        match general.ping().await {
            // If the call is successful, print the data.
            Ok(data) => println!("{:#?}", data),
            // If the call fails, print the error.
            Err(err) => println!("{:#?}", err),
        }
    }
}
