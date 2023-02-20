use std::sync::Arc;

use tokio::task;
use tokio::task::JoinHandle;

use common::rabbitmq::{RabbitMQ, Stream};
use database::{DatabaseConnection, Engine, Migrator, MigratorTrait, Mutation};
use database::fills::Fill;

#[tokio::main]
async fn main() {
    // Establish connection to database and apply migrations
    let db = Engine::connect().await.unwrap(); // Allow to panic if unsuccessful
    Migrator::up(&db, None).await.unwrap(); // Allow to panic if unsuccessful

    let rabbitmq = RabbitMQ::new(true).await;

    spawn_fills_worker(db, &rabbitmq).await;
}

/// Spawns a future on the current thread as a new task for recording fills from the matching engine.
/// The task also updates client positions from the fill.
async fn spawn_fills_worker(db: DatabaseConnection, rabbitmq: &RabbitMQ) -> JoinHandle<()> {
    let mut consumer = rabbitmq.consumer(Stream::Fills).await;
    task::spawn(async move {
        loop {
            if let Some(fill) = consumer.next::<Fill>().await {
                let fill = Mutation::create_fill(&db, fill).await.unwrap();
                Mutation::upsert_position_from_fill(&db, fill).await.unwrap();
            }
        }
    })
}
