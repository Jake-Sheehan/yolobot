#![allow(unused_imports)]
use anyhow::Result;
use exchange_manager::data_models::Response::Stream;
use exchange_manager::{
    self,
    data_models::{StreamResponse, TickerData, TickerResponse},
};
use market_data;
use std::sync::Arc;
use tokio::time::{sleep, Duration};
use tokio_tungstenite::tungstenite::Message;
use yolobot_utils;

#[tokio::main]
async fn main() {
    let ticker = market_data::ticker::Ticker::new("BTC/USD", 10)
        .await
        .expect("ahhh");

    loop {
        sleep(Duration::from_millis(33)).await;
        let data = ticker.get().await;

        if data.is_some() {
            if let Some(Stream(stream)) = data {
                if let StreamResponse::ticker { data, .. } = stream {
                    print!("\x1B[2J\x1B[1;1H");
                    println!("{:#?}", data);
                }
            }
        }
    }
}
