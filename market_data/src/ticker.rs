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
use yolobot_utils::sized_stack::SizedStack;

#[derive(Error, Debug)]
pub enum TickerError {
    #[error("subscribe failed")]
    SubscribeError,
    #[error("stream contained no ticker data")]
    NoData,
}

pub struct Ticker {
    buffer: Arc<RwLock<SizedStack<Message>>>,
    join_handle: JoinHandle<Result<()>>,
}

impl Ticker {
    pub async fn new(symbol: &'static str, buffer_capacity: usize) -> Result<Self> {
        let mut router = Router::new().await;

        let stack = Arc::new(RwLock::new(SizedStack::new(buffer_capacity)));

        let stack_clone = Arc::clone(&stack);
        let join_handle: JoinHandle<Result<()>> = tokio::spawn(async move {
            let subscribe_msg = kraken::subscribe("ticker", &vec![symbol.to_string()]);
            router.send(Message::Binary(subscribe_msg)).await?;
            loop {
                let res = router.recv().await;
                if let Some(msg) = res {
                    let mut write_guard = stack_clone.write().await;
                    write_guard.push(msg);
                }
            }
            return Ok(());
        });

        return Ok(Self {
            join_handle,
            buffer: stack,
        });
    }

    pub async fn get(&self) -> Option<Response> {
        let stack = Arc::clone(&self.buffer);
        let mut write_guard = stack.write().await;
        let raw_msg = write_guard.pop();
        if let Some(msg) = raw_msg {
            if let Message::Text(mut content) = msg {
                let json: Response =
                    unsafe { simd_json::from_slice(content.as_bytes_mut()).expect("ahhhh") };
                return Some(json);
            } else {
                return None;
            }
        } else {
            return None;
        }
    }

    pub fn blocking_get(&self, symbol: &str) {}

    pub async fn close(&self) {
        self.join_handle.abort();
    }
}
