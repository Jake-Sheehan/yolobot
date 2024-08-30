use anyhow::Result;
use exchange_manager::data_models::Response::{Stream, Subscribe};
use exchange_manager::data_models::{Response, StreamResponse, TickerData};
use exchange_manager::kraken;
use exchange_manager::smart_router::Router;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::RwLock;
use tokio::task::JoinHandle;
use tokio_tungstenite::tungstenite::Message;
use yolobot_utils::error;
use yolobot_utils::ring_buffer::RingBuffer;

#[derive(Error, Debug)]
pub enum TickerError {
    #[error("subscribe failed")]
    SubscribeError,
    #[error("stream contained no ticker data")]
    NoData,
}

pub struct Ticker {
    buffer: RingBuffer,
    join_handle: JoinHandle<Result<()>>,
}

impl Ticker {
    pub async fn new(symbol: &'static str) -> Result<Self> {
        let mut router = Router::new().await;

        let join_handle: JoinHandle<Result<()>> = tokio::spawn(async move {
            let subscribe_msg = kraken::subscribe("ticker", &vec![symbol.to_string()]);
            router.send(Message::Binary(subscribe_msg)).await?;
            loop {
                let res = router.recv().await;
            }
            return Ok(());
        });

        return Ok(Self { join_handle });
    }

    pub async fn get(&self, symbol: &str) {}

    pub fn blocking_get(&self, symbol: &str) {}

    pub fn get_all(&self) {}

    pub async fn close(&self) {
        self.join_handle.abort();
    }
}
