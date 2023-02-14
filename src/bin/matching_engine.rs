use std::time::Duration;

use tokio::time::sleep;

use orderbook::OrderBook;

#[tokio::main]
async fn main() {
    sleep(Duration::from_secs(5)).await;
    let mut orderbook = OrderBook::new(1).await;
    orderbook.run().await;
}
