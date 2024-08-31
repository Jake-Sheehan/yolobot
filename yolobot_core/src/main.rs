#![allow(unused_imports)]
use anyhow::Result;
use exchange_manager::{self, data_models::TickerData};
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
        print!("\x1B[2J\x1B[1;1H");
        if data.is_some() {
            println!("{:#?}", data);
        }
    }
}
