use crate::api::{Market, API};
use crate::client::Client;
use crate::errors::Result;
use crate::model::{ServerTime, ServerTimeResponse};

#[derive(Clone)]
pub struct General {
    pub client: Client,
}

impl General {
    /// Tests for connectivity
    pub async fn ping(&self) -> Result<String> {
        // Call the get method on the client field of self, passing in the time variable and None as arguments, and return the result
        let _response: ServerTimeResponse= self.client.get(API::Market(Market::Time), None).await?;
        // prints pong to the console
        Ok("pong".to_string())
    }

    /// Checks server time
    pub async fn get_server_time(&self) -> Result<ServerTime> {
        // Create a variable called time and set it to an API::Market enum variant with a Market::Time value
        // Call the get method on the client field of self, passing in the time variable and None as arguments, and return the result
        let response: ServerTimeResponse = self.client.get(API::Market(Market::Time), None).await?;
    
        Ok(response.result)
    }
}
