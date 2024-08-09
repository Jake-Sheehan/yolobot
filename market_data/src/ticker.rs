use anyhow::Result;
use exchange_manager::data_models::Response::{Stream, Subscribe};
use exchange_manager::data_models::{Response, StreamResponse, TickerData};
use exchange_manager::kraken;
use exchange_manager::smart_router::Router;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use thiserror::Error;
use tokio::task::JoinHandle;
use tokio_tungstenite::tungstenite::Message;
use yolobot_utils::error;

#[derive(Error, Debug)]
pub enum TickerError {
    #[error("subscribe failed")]
    SubscribeError,
    #[error("stream contained no ticker data")]
    NoData,
}

pub struct Ticker {
    data_map: Arc<HashMap<String, RwLock<TickerData>>>,
    join_handle: JoinHandle<Result<()>>,
}

impl Ticker {
    pub async fn new(symbols: Vec<String>) -> Result<Self> {
        let mut data_map: HashMap<String, RwLock<TickerData>> = HashMap::new();

        for symbol in &symbols {
            data_map.insert(
                symbol.to_string(),
                RwLock::new(TickerData::new(symbol.to_string())),
            );
        }

        let data_map = Arc::new(data_map);

        let mut router = Router::new().await;

        let data_map_clone = Arc::clone(&data_map);
        let join_handle: JoinHandle<Result<()>> = tokio::spawn(async move {
            let subscribe_msg = kraken::subscribe("ticker", &symbols);
            router.send(Message::Binary(subscribe_msg)).await?;
            loop {
                let res = router.recv().await;
                match res {
                    Some(Stream(stream)) => {
                        if let StreamResponse::ticker { mut data, .. } = stream {
                            if data.is_empty() {
                                error::log(TickerError::NoData);
                                continue;
                            }

                            data.drain(..).for_each(|ticker_data| {
                                match data_map_clone.get(&ticker_data.symbol) {
                                    Some(lock) => {
                                        let mut write_guard =
                                            lock.write().unwrap_or_else(|poisoned| {
                                                return poisoned.into_inner();
                                            });
                                        *write_guard = ticker_data;
                                    }

                                    None => (),
                                }
                            })
                        }
                    }
                    Some(Subscribe(subscribe)) => {
                        if subscribe.success == false {
                            error::log(TickerError::SubscribeError);
                            break;
                        }
                    }
                    None => break,
                }
            }
            return Ok(());
        });

        return Ok(Self {
            data_map,
            join_handle,
        });
    }

    pub fn get(&self, symbol: &str) -> Option<TickerData> {
        let lock = self.data_map.get(symbol)?;
        let read_guard = lock.read().unwrap();
        return Some(read_guard.clone());
    }

    pub fn get_all(&self) -> Arc<HashMap<String, RwLock<TickerData>>> {
        return Arc::clone(&self.data_map);
    }

    pub async fn close(&self) {
        self.join_handle.abort();
    }
}
