use orderbook::OrderBook;

#[async_std::main]
async fn main() {
    let mut orderbook = OrderBook::new(1).await;
    orderbook.run().await;
}