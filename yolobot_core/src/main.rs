#![allow(unused_imports)]
use anyhow::Result;
use exchange_manager;
use market_data;
use tokio::time::{sleep, Duration};
use tokio_tungstenite::tungstenite::Message;

#[tokio::main]
async fn main() {
    let ticker =
        market_data::ticker::Ticker::new(vec!["BTC/USD".to_string(), "ETH/USD".to_string()])
            .await
            .unwrap();

    loop {
        sleep(Duration::from_secs(1)).await;
        print!("\x1B[2J\x1B[1;1H");
        println!("{:#?}", ticker.get("BTC/USD").await.unwrap());
        println!("{:#?}", ticker.get("ETH/USD").await.unwrap());
    }
}
