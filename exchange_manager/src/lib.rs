pub mod data_models;
pub mod kraken;
pub mod smart_router;
pub mod web_socket;

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests connection to Kraken exchange.
    ///
    /// # Panics
    ///
    /// Panics if web socket connection fails.
    #[tokio::test]
    async fn it_connects_to_kraken() {
        let _session = web_socket::WebSocketSession::new("wss://ws.kraken.com/v2")
            .await
            .expect("web socket connection failed");
    }
}
