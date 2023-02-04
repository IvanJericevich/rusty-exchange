use std::time::Duration;

#[async_std::main]
async fn main() {
    // Establish connection to database and apply migrations
    let db = Engine::connect().await.unwrap(); // Allow to panic if unsuccessful
    Migrator::up(&db, None).await.unwrap(); // Allow to panic if unsuccessful

    let environment = Environment::builder()
        .host("localhost")
        .port(5552)
        .build()
        .await
        .unwrap();
    let _ = environment.delete_stream("orders").await; // Delete stream if it exists
    let _ = environment.delete_stream("fills").await; // Delete stream if it exists
    let stream_creator = environment // Create stream at producer
        .stream_creator()
        .max_length(ByteCapacity::MB(50))
        .max_age(Duration::new(30, 0));

    stream_creator.create("orders").await.unwrap();
    stream_creator.create("fills").await.unwrap();
}
