// TODO: Improve error documentation
// TODO: Make sure swagger-docs have the correct headers
// TODO: Ensure correct type definitions for OpenApi models/schemas
// TODO: Improve OpenApi schema code
// TODO: Research the use of clone() or copy(). Should String arguments be &str?
// TODO: Hide foreign keys from openapi schema
// TODO: what about datetime provided as timestamps
// TODO; Create index.html
// TODO: Test error responses
mod models;
mod routes;

use database::{DatabaseConnection, Engine, Migrator, MigratorTrait};

use actix_web::{middleware::Logger, web, App, HttpServer, HttpResponse, post};

use std::time::Duration;
use actix_web::dev::ServerHandle;
use parking_lot::Mutex;

use rabbitmq_stream_client::{Environment, NoDedup, Producer};
use rabbitmq_stream_client::types::ByteCapacity;

use routes::router;

// ----------------------------------------------------------------------

struct AppState {
    db: DatabaseConnection,
    producer: Option<Producer<NoDedup>>, // Make optional for unit tests
    stop_handle: StopHandle
}

// ----------------------------------------------------------------------

#[post("/stop/{graceful}")]
async fn stop(path: web::Path<bool>, data: web::Data<AppState>,) -> HttpResponse {
    let graceful = path.into_inner();
    if let Some(producer) = data.producer.clone() { // TODO: Is it correct to clone the producer
        producer.close().await.unwrap();
    }
    let _ = &data.stop_handle.stop(graceful);
    HttpResponse::NoContent().finish()
}

// ----------------------------------------------------------------------

#[actix_web::main]
async fn run() -> std::io::Result<()> {
    tracing_subscriber::fmt().init(); // Log SQL operations

    // Establish connection to database and apply migrations
    let db = Engine::connect().await.unwrap(); // Allow to panic if unsuccessful
    Migrator::up(&db, None).await.unwrap(); // Allow to panic if unsuccessful

    let producer = if !cfg!(test) {
        // Establish connection to RabbitMQ
        let environment = Environment::builder()
            .host("localhost")
            .port(5552)
            .build()
            .await
            .unwrap();
        let _ = environment.delete_stream("orders").await; // Delete stream if it exists
        environment // Create stream at producer
            .stream_creator()
            .max_length(ByteCapacity::MB(50))
            .max_age(Duration::new(30, 0))
            .create("orders")
            .await
            .unwrap();
        Some( // TODO: Mutex?
             environment
                 .producer()
                 .build("orders")
                 .await
                 .unwrap()
        )
    } else {
        None
    };

    let state = web::Data::new(AppState {
        db,
        producer,
        stop_handle: StopHandle::default()
    }); // Build app state

    let server = HttpServer::new({
        let state = state.clone(); // Ensure that state isn't moved
        move || {
            App::new()
                .wrap(Logger::new("%r %s (%Ts)"))
                .app_data(state.clone())
                .configure(router)
        }
    })
    .bind(("127.0.0.1", 8080))?
    .workers(1)
    .run();

    state.stop_handle.register(server.handle());

    server.await?;

    Ok(())
}

#[derive(Default)]
struct StopHandle {
    inner: Mutex<Option<ServerHandle>>,
}

impl StopHandle {
    /// Sets the server handle to stop.
    pub(crate) fn register(&self, handle: ServerHandle) {
        *self.inner.lock() = Some(handle);
    }

    /// Sends stop signal through contained server handle.
    pub(crate) fn stop(&self, graceful: bool) {
        let _ = self.inner.lock().as_ref().unwrap().stop(graceful);
    }
}

pub fn main() {
    if let Some(err) = run().err() {
        println!("Error: {}", err);
    }
}
