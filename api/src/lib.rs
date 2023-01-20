// TODO: Improve error documentation
// TODO: Make sure swagger-docs have the correct headers
// TODO: Ensure correct type definitions for OpenApi models/schemas
// TODO: Improve OpenApi schema code
// TODO: Research the use of clone() or copy(). Should String arguments be &str?
// TODO: Hide foreign keys from openapi schema
// TODO: what about datetime provided as timestamps
// TODO: Server error handler or tear down function

mod models;
mod routes;

use database::{DatabaseConnection, Engine, Migrator, MigratorTrait};

use actix_web::{middleware::Logger, web, App, HttpServer};

use std::time::Duration;

use rabbitmq_stream_client::{Environment, NoDedup, Producer};
use rabbitmq_stream_client::types::ByteCapacity;

use routes::router;

// ----------------------------------------------------------------------
#[derive(Clone)]
struct AppState {
    db: DatabaseConnection,
    producer: Option<Producer<NoDedup>> // Make optional for unit tests
}

#[actix_web::main]
async fn run() -> std::io::Result<()> {
    tracing_subscriber::fmt().init(); // Log SQL operations
    std::env::set_var("RUST_LOG", "actix_web=trace");

    // Establish connection to database and apply migrations
    let db = Engine::connect().await.unwrap(); // Allow to panic if unsuccessful
    Migrator::up(&db, None).await.unwrap(); // Allow to panic if unsuccessful

    // Establish connection to RabbitMQ
    let environment = Environment::builder()
        .host("localhost")
        .port(5552)
        .build()
        .await
        .unwrap();
    environment
        .stream_creator()
        .max_length(ByteCapacity::MB(50))
        .max_age(Duration::new(30, 0))
        .create("orders")
        .await
        .unwrap();
    let producer = Some(
        environment
            .producer()
            .build("orders")
            .await
            .unwrap()
    );

    let state = AppState { db, producer }; // Build app state

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::new("%r %s (%Ts)"))
            .app_data(web::Data::new(state.clone()))
            .configure(router)
    })
    .bind(("127.0.0.1", 8080))?
    .workers(1)
    .run()
    .await?;

    Ok(())
}

pub fn main() {
    let result = run();

    if let Some(err) = result.err() {
        println!("Error: {}", err);
    }
}
