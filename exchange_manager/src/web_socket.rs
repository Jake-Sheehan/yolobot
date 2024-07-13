//! # Web Socket
//!
//! 'web_socket' is a collection of utils for connecting to web sockets.
//!

#![allow(dead_code)]

use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use tokio::sync::mpsc;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio_tungstenite::tungstenite::Message;

type JoinHandle = tokio::task::JoinHandle<Result<()>>;

/// Connects to a web socket, wires an unbounded mpsc channel
/// to the socket, and returns a join handle.
///
/// # Errors
///
/// Returns an anyhow Result. Errors can occur if
/// the connection fails, a send fails, or a receive fails.
async fn connect(
    url: &str,
    tx: UnboundedSender<Message>,
    mut rx: UnboundedReceiver<Message>,
) -> Result<JoinHandle> {
    let (ws, _res) = tokio_tungstenite::connect_async(url).await?;
    println!("connected to {}", url);
    let (mut sink, mut stream) = ws.split();

    let join_handle = tokio::spawn(async move {
        loop {
            tokio::select! {
                // receive message to send through web socket
                Some(msg) = rx.recv() => {
                    sink.send(msg).await?;
                }
                // get message from websocket
                Some(msg) = stream.next() => {
                    if let Ok(message) = msg {
                        tx.send(message)?;
                    }
                }
            }
        }
    });

    return Ok(join_handle);
}

/// Creates a new web socket connection. This is part of the
/// public API.
///
/// # Errors
///
/// Returns an anyhow Result. Errors can occur in connect();
pub async fn new(
    url: &str,
) -> Result<(
    UnboundedSender<Message>,
    UnboundedReceiver<Message>,
    JoinHandle,
)> {
    let (tx1, rx1) = mpsc::unbounded_channel();
    let (tx2, rx2) = mpsc::unbounded_channel();
    let join_handle = connect(url, tx1, rx2).await?;
    return Ok((tx2, rx1, join_handle));
}
