use crate::api::{Public, WebsocketAPI};
use crate::client::Client;
use crate::errors::Result;
use crate::model::{
    Category, ExecutionData, KlineData, OrderBookUpdate, OrderData, PongResponse, PositionData,
    Subscription, Tickers, WalletData, WebsocketEvents, WsTrade,
};
use crate::util::{build_json_request, generate_random_uid};
use error_chain::bail;
use futures::StreamExt;
use serde_json::Value;
use std::collections::BTreeMap;
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::{tungstenite::Message as WsMessage, MaybeTlsStream};

#[derive(Clone)]
pub struct Stream {
    pub client: Client,
}

impl Stream {
    pub async fn ws_ping(&self, private: bool) -> Result<()> {
        let mut parameters: BTreeMap<String, Value> = BTreeMap::new();
        parameters.insert("req_id".into(), generate_random_uid(8).into());
        parameters.insert("op".into(), "ping".into());
        let request = build_json_request(&parameters);
        let endpoint = if private {
            WebsocketAPI::Private
        } else {
            WebsocketAPI::Public(Public::Linear)
        };
        let mut response = self
            .client
            .wss_connect(endpoint, Some(request), private, None)
            .await?;
        let data = response.next().await.unwrap()?;
        match data {
            WsMessage::Text(data) => {
                let response: PongResponse = serde_json::from_str(&data)?;
                match response {
                    PongResponse::PublicPong(pong) => {
                        println!("Pong received successfully");
                        println!("Connection ID: {}", pong.conn_id);
                    }
                    PongResponse::PrivatePong(pong) => {
                        println!("Pong received successfully");
                        println!("Connection ID: {}", pong.conn_id);
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }

    pub async fn ws_priv_subscribe<'a, F>(&self, req: Subscription<'a>, handler: F) -> Result<()>
    where
        F: FnMut(WebsocketEvents) -> Result<()> + 'static + Send,
    {
        let request = Self::build_subscription(req);
        let response = self
            .client
            .wss_connect(WebsocketAPI::Private, Some(request), true, Some(9))
            .await?;
        Self::event_loop(response, handler).await?;
        Ok(())
    }

    pub async fn ws_subscribe<'a, F>(
        &self,
        req: Subscription<'a>,
        category: Category,
        handler: F,
    ) -> Result<()>
    where
        F: FnMut(WebsocketEvents) -> Result<()> + 'static + Send,
    {
        let endpoint = {
            match category {
                Category::Linear => WebsocketAPI::Public(Public::Linear),
                Category::Inverse => WebsocketAPI::Public(Public::Inverse),
                Category::Spot => WebsocketAPI::Public(Public::Spot),
                _ => bail!("Option has not been implemented"),
            }
        };
        let request = Self::build_subscription(req);
        let response = self
            .client
            .wss_connect(endpoint, Some(request), false, None)
            .await?;
        Self::event_loop(response, handler).await?;
        Ok(())
    }

    pub fn build_subscription(action: Subscription) -> String {
        let mut parameters: BTreeMap<String, Value> = BTreeMap::new();
        parameters.insert("req_id".into(), generate_random_uid(8).into());
        parameters.insert("op".into(), action.op.into());
        let args_value: Value = action
            .args
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .into();
        parameters.insert("args".into(), args_value);

        build_json_request(&parameters)
    }

    /// Subscribes to the specified order book updates and handles the order book events
    ///
    /// # Arguments
    ///
    /// * `subs` - A vector of tuples containing the order book ID and symbol
    /// * `category` - The category of the order book
    ///
    /// # Example
    ///
    /// ```
    /// use your_crate_name::Category;
    /// let subs = vec![(1, "BTC"), (2, "ETH")];
    /// ```
    pub async fn ws_orderbook(
        &self,
        subs: Vec<(i32, &str)>,
        category: Category,
        sender: mpsc::UnboundedSender<OrderBookUpdate>,
    ) -> Result<()> {
        let arr: Vec<String> = subs
            .into_iter()
            .map(|(num, sym)| format!("orderbook.{}.{}", num, sym.to_uppercase()))
            .collect();
        let request = Subscription::new("subscribe", arr.iter().map(AsRef::as_ref).collect());
        self.ws_subscribe(request, category, move |event| {
            if let WebsocketEvents::OrderBookEvent(order_book) = event {
                sender.send(order_book).unwrap();
            }
            Ok(())
        })
        .await
    }

    /// This function subscribes to the specified trades and handles the trade events.
    /// # Arguments
    ///
    /// * `subs` - A vector of trade subscriptions
    /// * `category` - The category of the trades
    ///
    /// # Example
    ///
    /// ```
    /// use your_crate_name::Category;
    /// let subs = vec!["BTCUSD", "ETHUSD"];
    /// let category = Category::Linear;
    /// ws_trades(subs, category);
    /// ```
    pub async fn ws_trades(
        &self,
        subs: Vec<&str>,
        category: Category,
        sender: mpsc::UnboundedSender<WsTrade>,
    ) -> Result<()> {
        let arr: Vec<String> = subs
            .iter()
            .map(|&sub| format!("publicTrade.{}", sub.to_uppercase()))
            .collect();
        let request = Subscription::new("subscribe", arr.iter().map(AsRef::as_ref).collect());
        let handler = move |event| {
            if let WebsocketEvents::TradeEvent(trades) = event {
                for trade in trades.data {
                    sender.send(trade).unwrap();
                }
            }
            Ok(())
        };

        self.ws_subscribe(request, category, handler).await
    }

    /// Subscribes to ticker events for the specified symbols and category.
    ///
    /// # Arguments
    ///
    /// * `subs` - A vector of symbols for which ticker events are subscribed.
    /// * `category` - The category for which ticker events are subscribed.
    ///
    /// # Examples
    ///
    /// ```
    /// use your_crate_name::Category;
    /// let subs = vec!["BTCUSD", "ETHUSD"];
    /// let category = Category::Linear;
    /// let sender = UnboundedSender<Tickers>;
    /// ws_tickers(subs, category, sender);
    /// ```
    pub async fn ws_tickers(
        &self,
        subs: Vec<&str>,
        category: Category,
        sender: mpsc::UnboundedSender<Tickers>,
    ) -> Result<()> {
        let arr: Vec<String> = subs
            .into_iter()
            .map(|sub| format!("tickers.{}", sub.to_uppercase()))
            .collect();
        let request = Subscription::new("subscribe", arr.iter().map(String::as_str).collect());

        let handler = move |event| {
            if let WebsocketEvents::TickerEvent(tickers) = event {
                match tickers.data {
                    Tickers::Linear(linear_ticker) => {
                        sender.send(Tickers::Linear(linear_ticker)).unwrap()
                    }
                    Tickers::Spot(spot_ticker) => sender.send(Tickers::Spot(spot_ticker)).unwrap(),
                }
            }
            Ok(())
        };

        self.ws_subscribe(request, category, handler).await
    }

    pub async fn ws_klines(
        &self,
        subs: Vec<(&str, &str)>,
        category: Category,
        sender: mpsc::UnboundedSender<KlineData>,
    ) -> Result<()> {
        let arr: Vec<String> = subs
            .into_iter()
            .map(|(interval, sym)| format!("kline.{}.{}", interval, sym.to_uppercase()))
            .collect();
        let request = Subscription::new("subscribe", arr.iter().map(AsRef::as_ref).collect());
        self.ws_subscribe(request, category, move |event| {
            if let WebsocketEvents::KlineEvent(kline) = event {
                for v in kline.data {
                    sender.send(v).unwrap();
                }
            }
            Ok(())
        })
        .await
    }

    pub async fn ws_position(
        &self,
        cat: Option<Category>,
        sender: mpsc::UnboundedSender<PositionData>,
    ) -> Result<()> {
        let sub_str = if let Some(v) = cat {
            match v {
                Category::Linear => "position.linear",
                Category::Inverse => "position.inverse",
                _ => bail!("Option and Spot has not been implemented"),
            }
        } else {
            "position"
        };

        let request = Subscription::new("subscribe", vec![sub_str]);
        self.ws_priv_subscribe(request, move |event| {
            if let WebsocketEvents::PositionEvent(position) = event {
                for v in position.data {
                    sender.send(v).unwrap();
                }
            }
            Ok(())
        })
        .await
    }

    pub async fn ws_executions(
        &self,
        cat: Option<Category>,
        sender: mpsc::UnboundedSender<ExecutionData>,
    ) -> Result<()> {
        let sub_str = if let Some(v) = cat {
            match v {
                Category::Linear => "execution.linear",
                Category::Inverse => "execution.inverse",
                Category::Spot => "execution.spot",
                Category::Option => "execution.option",
            }
        } else {
            "execution"
        };

        let request = Subscription::new("subscribe", vec![sub_str]);
        self.ws_priv_subscribe(request, move |event| {
            if let WebsocketEvents::ExecutionEvent(execute) = event {
                for v in execute.data {
                    sender.send(v).unwrap();
                }
            }
            Ok(())
        })
        .await
    }

    pub async fn ws_orders(
        &self,
        cat: Option<Category>,
        sender: mpsc::UnboundedSender<OrderData>,
    ) -> Result<()> {
        let sub_str = if let Some(v) = cat {
            match v {
                Category::Linear => "order.linear",
                Category::Inverse => "order.inverse",
                Category::Spot => "order.spot",
                Category::Option => "order.option",
            }
        } else {
            "order"
        };

        let request = Subscription::new("subscribe", vec![sub_str]);
        self.ws_priv_subscribe(request, move |event| {
            if let WebsocketEvents::OrderEvent(order) = event {
                for v in order.data {
                    sender.send(v).unwrap();
                }
            }
            Ok(())
        })
        .await
    }

    pub async fn ws_wallet(&self, sender: mpsc::UnboundedSender<WalletData>) -> Result<()> {
        let sub_str = "wallet";
        let request = Subscription::new("subscribe", vec![sub_str]);
        self.ws_priv_subscribe(request, move |event| {
            if let WebsocketEvents::Wallet(wallet) = event {
                for v in wallet.data {
                    sender.send(v).unwrap();
                }
            }
            Ok(())
        })
        .await
    }

    pub async fn event_loop<H>(
        mut stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
        mut handler: H,
    ) -> Result<()>
    where
        H: WebSocketHandler,
    {
        loop {
            let msg = stream.next().await.unwrap()?;
            match msg {
                WsMessage::Text(ref msg) => {
                    if let Err(e) = handler.handle_msg(msg) {
                        bail!(format!("Error handling stream message: {:?}", e));
                    }
                }
                _ => {}
            }
        }
    }
}

pub trait WebSocketHandler {
    type Event;
    fn handle_msg(&mut self, msg: &str) -> Result<()>;
}

impl<F> WebSocketHandler for F
where
    F: FnMut(WebsocketEvents) -> Result<()>,
{
    type Event = WebsocketEvents;
    fn handle_msg(&mut self, msg: &str) -> Result<()> {
        let update: Value = serde_json::from_str(msg)?;
        if let Ok(event) = serde_json::from_value::<WebsocketEvents>(update.clone()) {
            self(event)?;
        }

        Ok(())
    }
}
