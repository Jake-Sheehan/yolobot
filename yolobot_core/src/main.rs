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
    let ticker = Arc::new(
        market_data::ticker::Ticker::new(vec!["BTC/USD".to_string(), "ETH/USD".to_string()])
            .await
            .unwrap(),
    );

    let mut eth = TickerData::new("ETH/USD".to_string());
    let mut btc = TickerData::new("BTC/USD".to_string());

    loop {
        eth = ticker.get(&eth.symbol).await.expect("ahhhhh");
        btc = ticker.get(&btc.symbol).await.expect("ahhhhh");
        print!("\x1B[2J\x1B[1;1H");
        println!("{:?}", eth);
        println!("{:?}", btc);
    }
}
