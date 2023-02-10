use orderbook::OrderBook;

#[tokio::main]
async fn main() {
    let mut orderbook = OrderBook::new(1).await;
    orderbook.run().await;
}
