#[derive(Clone, Debug)]
pub struct Config {
    pub rest_api_endpoint: &'static str,
    pub ws_endpoint: &'static str,
    pub recv_window: u16,
}

impl Config {
    pub const DEFAULT_REST_API_ENDPOINT: &'static str = "https://api.bybit.com";
    pub const DEFAULT_WS_ENDPOINT: &'static str = "wss://stream.bybit.com/v5";

    pub const fn default() -> Self {
        Self {
            rest_api_endpoint: Self::DEFAULT_REST_API_ENDPOINT,
            ws_endpoint: Self::DEFAULT_WS_ENDPOINT,
            recv_window: 5000,
        }
    }

    pub const fn testnet() -> Self {
        Self {
            rest_api_endpoint: "https://api-testnet.bybit.com",
            ws_endpoint: "wss://stream-testnet.bybit.com/v5",
            recv_window: 5000,
        }
    }

    pub const fn set_recv_window(self, recv_window: u16) -> Self {
        Self {
            recv_window,
            ..self
        }
    }
}
