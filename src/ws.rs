use crate::api::{Public, WebsocketAPI};
use crate::client::Client;
use crate::errors::Result;
use crate::model::*;
use crate::util::{build_json_request, generate_random_uid};
use error_chain::bail;
use serde_json::Value;
use std::collections::BTreeMap;
use std::net::TcpStream;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::{Message as WsMessage, WebSocket};

pub struct Stream {
    pub client: Client,
}

impl Stream {
    pub async fn ws_ping(&self, private: bool) -> Result<()> {
        let mut parameters: BTreeMap<String, Value> = BTreeMap::new();
        parameters.insert("req_id".into(), generate_random_uid(8).into());
        parameters.insert("op".into(), "ping".into());
        let request = build_json_request(&parameters);
        let response = self
            .client
            .wss_connect(
                WebsocketAPI::Public(Public::Linear),
                Some(request),
                private,
                None,
                |event| {
                    match event {
                        WebsocketEvents::Pong(pong) => match pong {
                            PongResponse::PublicPong(category) => match category {
                                PongCategory::Perpetual(data) => {
                                    println!("{:#?}", data);
                                }
                                PongCategory::Spot(data) => {
                                    println!("{:#?}", data);
                                }
                            },
                            PongResponse::PrivatePong(data) => println!("{:#?}", data), // Handle Ping event
                        },
                        _ => {
                            bail!("Unexpected event");
                        }
                    }
                    Ok(())
                },
            )
            .await?;
        Ok(response)
    }

    pub async fn ws_priv_subscribe<'a, F>(&self, req: Subscription<'a>, handler: F) -> Result<()>
    where
        F: FnMut(WebsocketEvents) -> Result<()> + 'static + Send,
    {
        let request = Self::build_subscription(req);
        let response = self
            .client
            .wss_connect(WebsocketAPI::Private, Some(request), true, Some(8), handler)
            .await?;
        Ok(response)
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
            .wss_connect(endpoint, Some(request), false, None, handler)
            .await?;
        Ok(response)
    }

    pub async fn ws_unsubscribe<'a>(&self, req: Subscription<'a>) -> Result<()> {
        let request = Self::build_subscription(req);
        let response = self
            .client
            .wss_connect(WebsocketAPI::Private, Some(request), true, None, |_| Ok(()))
            .await?;
        Ok(response)
    }

    pub fn disconnect(&self) -> Result<()> {
        drop(&self.client);
        Ok(())
    }

    pub fn build_subscription(action: Subscription) -> String {
        let mut parameters: BTreeMap<String, Value> = BTreeMap::new();
        parameters.insert("req_id".into(), generate_random_uid(8).into());
        parameters.insert("op".into(), action.op.into());
        parameters.insert("args".into(), action.args.into());
        let request = build_json_request(&parameters);
        request
    }

    fn handle_msg(msg: &str, mut parser: impl FnMut(WebsocketEvents) -> Result<()>) -> Result<()> {
        let update: Value = serde_json::from_str(msg)?;
        if let Some(data) = update.get("data") {
            let event = serde_json::from_value::<WebsocketEvents>(data.clone())?;
            parser(event)?;
        }

        Ok(())
    }

    pub fn event_loop(
        mut stream: WebSocket<MaybeTlsStream<TcpStream>>,
        mut parser: impl FnMut(WebsocketEvents) -> Result<()> + Send + 'static,
    ) -> Result<()> {
        loop {
            let msg = stream.read()?;
            match msg {
                WsMessage::Text(ref msg) => {
                    if let Err(e) = Stream::handle_msg(msg, &mut parser) {
                        bail!(format!("Error on handling stream message: {}", e));
                    }
                }
                _ => {}
            }
        }
    }
}
