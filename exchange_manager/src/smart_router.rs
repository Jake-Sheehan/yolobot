use core::panic;

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

    pub async fn recv(&mut self) -> Option<Message> {
        match self.exchange.recv().await {
            Some(msg) => match msg {
                Ok(result) => Some(result),
                Err(e) => panic!("Error: {e}"),
            },
            None => None,
        }
    }

    pub async fn close(&mut self) -> Result<()> {
        self.exchange.close().await?;
        return Ok(());
    }
}
