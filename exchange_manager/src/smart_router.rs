use crate::data_models;
use crate::data_models::Response;
use crate::web_socket;
use anyhow::Result;
use tokio_tungstenite::tungstenite::Message;

pub struct Router {
    exchange: web_socket::WebSocketSession,
}

impl Router {
    pub async fn new() -> Self {
        let kraken = web_socket::WebSocketSession::new("wss://ws.kraken.com/v2")
            .await
            .expect("connection failed");

        return Self { exchange: kraken };
    }

    pub async fn send(&mut self, msg: Message) -> Result<()> {
        self.exchange.send(msg).await?;
        return Ok(());
    }

    pub async fn recv(&mut self) -> Option<data_models::Response> {
        match self.exchange.recv().await {
            Some(message) => match message {
                Ok(msg) => {
                    if let Message::Text(mut content) = msg {
                        let res: Response = unsafe {
                            simd_json::from_slice(content.as_bytes_mut()).expect(&content)
                        };
                        return Some(res);
                    } else {
                        return None;
                    }
                }
                Err(e) => panic!("{}", e),
            },
            None => {
                return None;
            }
        }
    }

    pub async fn close(&mut self) -> Result<()> {
        self.exchange.close().await?;
        return Ok(());
    }
}
