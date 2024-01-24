#[derive(Clone, Debug)]
pub struct Config {
    pub rest_api_endpoint: String,
    pub ws_endpoint: String,
    pub private_ws_endpoint: String,
    pub recv_window: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            rest_api_endpoint: "https://api.bybit.com".to_string(),
            ws_endpoint: "wss://stream.bybit.com/v5/public".to_string(),
            private_ws_endpoint: "wss://stream.bybit.com/v5/private".to_string(),
            recv_window: 5000,
        }
    }
}

impl Config {
    pub fn testnet() -> Self {
        Self::default()
            .set_rest_api_endpoint("https://api-testnet.bybit.com")
            .set_ws_endpoint("wss://stream-testnet.bybit.com/v5/public")
            .set_private_ws_endpoint("wss://stream-testnet.bybit.com/v5/private")
    }

    pub fn set_rest_api_endpoint(mut self, rest_api_endpoint: impl Into<String>) -> Self {
        self.rest_api_endpoint = rest_api_endpoint.into();
        self
    }

    pub fn set_ws_endpoint(mut self, ws_endpoint: impl Into<String>) -> Self {
        self.ws_endpoint = ws_endpoint.into();
        self
    }

    pub fn set_private_ws_endpoint(mut self, private_ws_endpoint: impl Into<String>) -> Self {
        self.private_ws_endpoint = private_ws_endpoint.into();
        self
    }

    pub fn set_recv_window(mut self, recv_window: u64) ->  Self {
        self.recv_window = recv_window;
        self
    }
}
