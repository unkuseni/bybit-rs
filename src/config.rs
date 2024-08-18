#[derive(Clone, Debug)]
pub struct Config<'a> {
    pub rest_api_endpoint: &'a str,
    pub ws_endpoint: &'a str,
    pub recv_window: u64,
}

impl <'a> Config<'_> {
    pub const DEFAULT_REST_API_ENDPOINT: &'a str = "https://api.bybit.com";
    pub const DEFAULT_WS_ENDPOINT: &'a str = "wss://stream.bybit.com/v5";

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

    pub const fn set_recv_window(self, recv_window: u64) -> Self {
        Self {
            recv_window,
            ..self
        }
    }
}
