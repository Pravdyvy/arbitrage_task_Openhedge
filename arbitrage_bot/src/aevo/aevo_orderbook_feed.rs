use std::sync::Arc;

use anyhow::Result;
use futures::{SinkExt, StreamExt};
use tokio::{net::TcpStream, sync::Mutex, task::JoinHandle};
use tokio_tungstenite::{tungstenite::Message, MaybeTlsStream, WebSocketStream};
use url::Url;

use super::aevo_structs::{
    AuthPayloadAEVO, ChannelsPayloadAEVO, ChannelsResponseAEVO, OrderbookAEVO,
    OrderbookAEVOResponse, OrderbookPayloadAEVO,
};

pub struct AEVOWSAuthenticator<'a> {
    pub wss_addr: &'a str,
}

impl<'a> AEVOWSAuthenticator<'a> {
    pub fn new(wss_addr: &'a str) -> Self {
        Self { wss_addr }
    }

    fn generate_api_key(&self) -> (String, String) {
        //Considering the fact, that we are simulating swaps we may as well simplify there
        ("API_KEY".to_string(), "SECRET_KEY".to_string())
    }

    fn generate_auth_message(&self) -> Message {
        let (api_key, secret) = self.generate_api_key();
        Message::Text(serde_json::to_string(&AuthPayloadAEVO::new(api_key, secret)).unwrap())
    }

    pub async fn authenticate(&self) -> Result<WebSocketStream<MaybeTlsStream<TcpStream>>> {
        let (mut websocket, _) =
            tokio_tungstenite::connect_async(Url::parse(&self.wss_addr)?).await?;

        let auth_message = self.generate_auth_message();

        websocket.send(auth_message).await?;

        Ok(websocket)
    }
}

pub struct AEVOWSOrderbookFeed {
    wss_socket_stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
}

impl AEVOWSOrderbookFeed {
    pub fn new(wss_socket_stream: WebSocketStream<MaybeTlsStream<TcpStream>>) -> Self {
        Self { wss_socket_stream }
    }

    fn generate_channels_message(&self) -> Message {
        Message::Text(serde_json::to_string(&ChannelsPayloadAEVO::new()).unwrap())
    }

    fn generate_orderbook_message(&self, channels: Vec<String>) -> Message {
        Message::Text(serde_json::to_string(&OrderbookPayloadAEVO::new(channels)).unwrap())
    }

    async fn subscribe_for_feed(&mut self) -> Result<()> {
        let channels_message = self.generate_channels_message();

        self.wss_socket_stream.send(channels_message).await?;

        let search_channel = if let Some(Ok(channels)) = self.wss_socket_stream.next().await {
            if let Message::Text(channels_text) = channels {
                let channels_decoded: ChannelsResponseAEVO = serde_json::from_str(&channels_text)?;

                //ToDo: Correct channel parsing
                channels_decoded
                    .data
                    .into_iter()
                    .find(|el| el.contains("ETH"))
                    .unwrap()
            } else {
                anyhow::bail!("Wrond message format")
            }
        } else {
            anyhow::bail!("Failed to receive channels")
        };

        let orderbook_message = self.generate_orderbook_message(vec![search_channel]);

        self.wss_socket_stream.send(orderbook_message).await?;

        Ok(())
    }

    pub async fn spawn_feed(
        mut self,
        orderbook_ref: Arc<Mutex<OrderbookAEVO>>,
    ) -> Result<JoinHandle<Result<()>>> {
        self.subscribe_for_feed().await?;

        let handle = tokio::spawn(async move {
            while let Some(resp) = self.wss_socket_stream.next().await {
                if let Ok(message) = resp {
                    if let Message::Text(feed_text) = message {
                        let feed_decoded: OrderbookAEVOResponse = serde_json::from_str(&feed_text)?;

                        {
                            let mut guard = orderbook_ref.lock().await;
                            guard.apply_changes(feed_decoded)
                        }
                    } else {
                        anyhow::bail!("Wrond message format")
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
