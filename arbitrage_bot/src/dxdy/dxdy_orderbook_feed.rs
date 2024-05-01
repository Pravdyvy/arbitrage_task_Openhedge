use std::sync::Arc;

use anyhow::Result;
use futures::{SinkExt, StreamExt};
use tokio::{net::TcpStream, sync::Mutex, task::JoinHandle};
use tokio_tungstenite::{tungstenite::Message, MaybeTlsStream, WebSocketStream};
use url::Url;

use super::dxdy_structs::{OrderbookDXDY, OrderbookDXDYResponse, OrderbookPayloadDXDY};

pub struct DXDYWSAuthenticator<'a> {
    pub wss_addr: &'a str,
}

impl<'a> DXDYWSAuthenticator<'a> {
    pub fn new(wss_addr: &'a str) -> Self {
        Self { wss_addr }
    }

    pub async fn authenticate(&self) -> Result<WebSocketStream<MaybeTlsStream<TcpStream>>> {
        let (websocket, _) = tokio_tungstenite::connect_async(Url::parse(self.wss_addr)?).await?;

        //No authorization requests detailed in docs

        Ok(websocket)
    }
}

pub struct DXDYWSOrderbookFeed {
    wss_socket_stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
}

impl DXDYWSOrderbookFeed {
    pub fn new(wss_socket_stream: WebSocketStream<MaybeTlsStream<TcpStream>>) -> Self {
        Self { wss_socket_stream }
    }

    fn generate_orderbook_message(&self) -> Message {
        Message::Text(serde_json::to_string(&OrderbookPayloadDXDY::default()).unwrap())
    }

    async fn subscribe_for_feed(&mut self) -> Result<()> {
        let orderbook_message = self.generate_orderbook_message();

        self.wss_socket_stream.send(orderbook_message).await?;

        Ok(())
    }

    pub async fn spawn_feed(
        mut self,
        orderbook_ref: Arc<Mutex<OrderbookDXDY>>,
    ) -> Result<JoinHandle<Result<()>>> {
        self.subscribe_for_feed().await?;

        let handle = tokio::spawn(async move {
            while let Some(resp) = self.wss_socket_stream.next().await {
                if let Ok(message) = resp {
                    match message {
                        Message::Text(feed_text) => {
                            let feed_decoded: OrderbookDXDYResponse =
                                serde_json::from_str(&feed_text)?;

                            {
                                let mut guard = orderbook_ref.lock().await;
                                guard.apply_changes(feed_decoded)
                            }
                        }
                        Message::Ping(_) => {
                            self.wss_socket_stream.send(Message::Pong(vec![])).await?
                        }
                        _ => anyhow::bail!("Unavaited message format"),
                    }
                } else {
                    anyhow::bail!("Failed to receive feed")
                }
            }

            Ok(())
        });

        Ok(handle)
    }
}
