//! # Web Socket
//!
//! 'web_socket' is a collection of utils for connecting to web sockets.
//!

use anyhow::Result;
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::{Error, Message};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

type SinkType = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;
type StreamType = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;

#[allow(dead_code)]
pub struct WebSocketSession {
    outbound: SinkType,
    pub inbound: StreamType,
}

impl WebSocketSession {
    pub async fn new(url: &str) -> Result<Self> {
        let (ws, _res) = tokio_tungstenite::connect_async(url).await?;
        let (sink, stream) = ws.split();
        return Ok(Self {
            outbound: sink,
            inbound: stream,
        });
    }

    pub async fn send(&mut self, msg: Message) -> Result<()> {
        self.outbound.send(msg).await?;
        return Ok(());
    }

    pub async fn recv(&mut self) -> Option<Result<Message, Error>> {
        return self.inbound.next().await;
    }

    pub async fn close(&mut self) -> Result<()> {
        self.outbound.close().await?;
        return Ok(());
    }
}
