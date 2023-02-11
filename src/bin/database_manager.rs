use std::time::Duration;
use rabbitmq_stream_client::Environment;
use rabbitmq_stream_client::types::{ByteCapacity};
use common::rabbitmq::Stream;
use common::util;
use database::{Engine, Migrator, MigratorTrait};

#[tokio::main]
async fn main() {
    // Establish connection to database and apply migrations
    let db = Engine::connect().await.unwrap(); // Allow to panic if unsuccessful
    Migrator::up(&db, None).await.unwrap(); // Allow to panic if unsuccessful

    let environment = Environment::builder()
        .host(if util::is_running_in_container() { "rabbitmq" } else { "localhost" })
        .port(5552)
        .build()
        .await
        .unwrap();

    let _ = Stream::iter()
        .map(|s| async {
            let _ = environment.delete_stream(s.as_str()).await; // Delete stream
            environment // Create stream
                .stream_creator()
                .max_length(ByteCapacity::MB(10))
                .max_age(Duration::new(30, 0))
                .create(s.as_str()).await.unwrap();
        });
}

// async fn spawn_worker<T>(db: DatabaseConnection, stream: &str) {
//     task::spawn(async move {
//         // Spawns a future on the current thread as a new task
//         let mut consumer = Environment::builder()
//             .host("localhost")
//             .port(5552)
//             .build()
//             .await
//             .unwrap()
//             .consumer()
//             .offset(OffsetSpecification::First)
//             .build(stream)
//             .await
//             .unwrap();
//         loop {
//             if let Some(Ok(delivery)) = consumer.next().await {
//                 if let Some(fill) = delivery
//                     .message()
//                     .data()
//                     .map(|data|
//                          serde_json::from_str::<T>(std::str::from_utf8(data).unwrap())
//                     )
//                 {
//                     Mutation::create_fill(&db, fill).await.unwrap();
//                 }
//             }
//         }
//     });
// }
