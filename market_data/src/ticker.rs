use anyhow::Result;
use exchange_manager::data_models::{Response, StreamResponse, TickerData};
use exchange_manager::kraken;
use exchange_manager::smart_router::Router;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tokio::task::JoinHandle;
use tokio_tungstenite::tungstenite::Message;

pub enum TickerError {
    RecvError,
}

pub struct Ticker {
    data_map: Arc<HashMap<String, RwLock<TickerData>>>,
    join_handle: JoinHandle<TickerError>,
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
        let join_handle = tokio::spawn(async move {
            let subscribe_msg = kraken::subscribe("ticker", &symbols);
            let _ = router.send(Message::Binary(subscribe_msg)).await;
            loop {
                let res = router.recv().await;
                match res {
                    Some(msg) => {
                        if let Response::Stream(stream) = msg {
                            if let StreamResponse::ticker { data, .. } = stream {
                                let ticker_data = &data[0];
                                let lock = data_map_clone.get(&ticker_data.symbol).unwrap();
                                let mut unlock = lock.write().unwrap();
                                *unlock = ticker_data.clone();
                            }
                        }
                    }
                    None => {
                        return TickerError::RecvError;
                    }
                }
            }
        });

        return Ok(Self {
            data_map,
            join_handle,
        });
    }

    pub async fn get(&self, symbol: &str) -> Option<TickerData> {
        let lock = self.data_map.get(symbol).unwrap();
        let unlock = lock.read().unwrap();
        return Some(unlock.clone());
    }

    pub async fn close(&self) {
        self.join_handle.abort();
    }
}
