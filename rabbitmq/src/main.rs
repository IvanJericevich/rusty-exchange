use std::time::Duration;
use chrono::{Utc};
use futures::StreamExt;
use rabbitmq_stream_client::{
    types::{ByteCapacity, Message, OffsetSpecification},
    Environment,
};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use database::{OrderModel, OrderSide, OrderStatus, OrderType};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    let environment = Environment::builder()
        .host("localhost")
        .port(5552)
        .build()
        .await?;

    let message_count = 10;
    environment
        .stream_creator()
        .max_length(ByteCapacity::MB(50))
        .max_age(Duration::new(30, 0))
        .create("test")
        .await?;

    let mut producer = environment
        .producer()
        .name("test_producer")
        .build("test")
        .await?;
    let x = OrderModel {
        id: 1,
        client_order_id: None,
        price: None,
        size: 10.0,
        filled_size: 0.0,
        side: OrderSide::Buy,
        r#type: OrderType::Limit,
        status: OrderStatus::Open,
        open_at: Utc::now().naive_utc(),
        closed_at: None,
        sub_account_id: 1,
        market_id: 1,
    };
    for i in 0..message_count {
        producer
            .send_with_confirm(Message::builder().body(serde_json::to_string(&x).unwrap()).build())
            .await?;
    }

    producer.close().await?;

    let mut consumer = environment
        .consumer()
        .offset(OffsetSpecification::First)
        .build("test")
        .await
        .unwrap();

    for _ in 0..message_count {
        let delivery = consumer.next().await.unwrap()?;
        info!(
            "Got message : {:?} with offset {}",
            delivery
                .message()
                .data()
                .map(|data| serde_json::from_str::<OrderModel>(
                std::str::from_utf8(&data.to_vec()).unwrap()).unwrap()
            ),
            delivery.offset()
        );
    }

    consumer.handle().close().await.unwrap();

    environment.delete_stream("test").await?;
    Ok(())
}