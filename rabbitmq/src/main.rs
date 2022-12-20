use rabbitmq_stream_client::types::{ByteCapacity, Message, OffsetSpecification};
use rabbitmq_stream_client::Environment;
use std::sync::{Arc, Barrier};

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    tracing_subscriber::fmt().init(); // Log SQL operations
    let environment = Environment::builder()
        .host("localhost")
        .port(5672)
        .build()
        .await?;
    let _ = environment.delete_stream("data").await;
    let message_count = 10;
    environment
        .stream_creator()
        .max_length(ByteCapacity::GB(2))
        .create("data")
        .await?;
    let producer = environment.producer().build("data").await?;
    let barrier = Arc::new(Barrier::new(message_count + 1));
    for i in 0..message_count {
        let producer_cloned = producer.clone();
        let barrier_cloned = barrier.clone();
        tokio::task::spawn(async move {
            producer_cloned
                .send_with_confirm(Message::builder().body(format!("message{}", i)).build())
                .await
                .unwrap();
            barrier_cloned.wait().await;
        });
    }
    barrier.wait().await;
    producer.close().await?;
    let mut consumer = environment
        .consumer()
        .offset(OffsetSpecification::First)
        .build("data")
        .await
        .unwrap();
    for _ in 0..message_count {
        let delivery = consumer.next().await.unwrap()?;
        println!(
            "Got message : {:?} with offset {}",
            delivery
                .message()
                .data()
                .map(|data| String::from_utf8(data.to_vec())),
            delivery.offset()
        );
    }
    consumer.handle().close().await.unwrap();
    environment.delete_stream("data").await?;
    Ok(())
}
