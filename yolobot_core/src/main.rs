#![allow(unused_imports)]
use anyhow::Result;
use exchange_manager;
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

    let data_map = Arc::clone(&ticker.get_all());

    //let ticker_clone1 = Arc::clone(&ticker);
    //let ticker_clone2 = Arc::clone(&ticker);

    //let join_handle1 = tokio::spawn(async move {
    //    loop {
    //       sleep(Duration::from_millis(50)).await;
    //        println!("{:#?}", ticker_clone1.get("BTC/USD").unwrap());
    //    }
    //});

    //let join_handle2 = tokio::spawn(async move {
    //    loop {
    //        sleep(Duration::from_millis(50)).await;
    //        println!("{:#?}", ticker_clone2.get("ETH/USD").unwrap());
    //    }
    //});

    //let _x = tokio::join!(join_handle1, join_handle2);

    let join = tokio::spawn(async move {
        loop {
            print!("\x1B[2J\x1B[1;1H");
            println!("{:?}", data_map);
        }
    });

    let _ = join.await;

    //return Ok(());
}
