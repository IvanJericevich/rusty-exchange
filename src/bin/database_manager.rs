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

/// Runs a task on the main thread (blocking) for updating orders from the matching engine.
async fn spawn_fills_worker(db: DatabaseConnection, rabbitmq: &RabbitMQ) {
    let mut consumer = rabbitmq.consumer(Stream::Fills).await;
    loop {
        if let Some(fill) = consumer.next::<Fill>().await {
            let fill = Mutation::create_fill(&db, fill).await.unwrap();
            Mutation::upsert_position_from_fill(&db, fill.clone()).await.unwrap();
            Mutation::update_order_from_fill(&db, fill).await.unwrap();
        }
    }
}


// Spawns a future on the current thread as a new task for recording fills from the matching engine.
// The task also updates client positions from the fill.
// async fn spawn_orders_worker(db: DatabaseConnection, rabbitmq: &RabbitMQ) -> JoinHandle<()> {
//     let mut consumer = rabbitmq.consumer(Stream::OpenOrders).await;
//     task::spawn(async move {
//         loop {
//             if let Some(order) = consumer.next::<Order>().await {
//                 let fill = Mutation::create_fill(&db, fill).await.unwrap();
//                 Mutation::upsert_position_from_fill(&db, fill).await.unwrap();
//             }
//         }
//     })
// }