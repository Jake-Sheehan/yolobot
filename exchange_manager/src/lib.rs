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
        let (_tx, _rx, join_handle) = web_socket::new("wss://ws.kraken.com/v2")
            .await
            .expect("web socket connection failed");
        join_handle.abort();
    }
}
