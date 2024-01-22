use std::borrow::Cow;

#[derive(Clone, Debug)]
pub struct Config {
    pub rest_api_endpoint: Cow<'static, str>,
    pub ws_endpoint: Cow<'static, str>,
    pub private_ws_endpoint: Cow<'static, str>,
    pub recv_window: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            rest_api_endpoint: Cow::Borrowed("https://api.bybit.com"),
            ws_endpoint: Cow::Borrowed("wss://stream.bybit.com/v5/public"),
            private_ws_endpoint: Cow::Borrowed("wss://stream.bybit.com/v5/private"),
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

    pub fn set_rest_api_endpoint<T: Into<Cow<'static, str>>>(mut self, rest_api_endpoint: T) -> Self {
        self.rest_api_endpoint = rest_api_endpoint.into();
        self
    }

    pub fn set_ws_endpoint<T: Into<Cow<'static, str>>>(mut self, ws_endpoint: T) -> Self {
        self.ws_endpoint = ws_endpoint.into();
        self
    }

    pub fn set_private_ws_endpoint<T: Into<Cow<'static, str>>>(mut self, private_ws_endpoint: T) -> Self {
        self.private_ws_endpoint = private_ws_endpoint.into();
        self
    }

    pub fn set_recv_window(mut self, recv_window: u64) -> Self {
        self.recv_window = recv_window;
        self
    }
}