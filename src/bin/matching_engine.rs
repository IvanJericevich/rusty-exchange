use std::time::Duration;

use tokio::time::sleep;

use orderbook::OrderBook;

#[tokio::main]
async fn main() {
    sleep(Duration::from_secs(5)).await;
    OrderBook::run(1).await;
}
