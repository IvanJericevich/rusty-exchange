// TODO: Improve error documentation
// TODO: Make sure swagger-docs have the correct headers
// TODO: Ensure correct type definitions for OpenApi models/schemas
// TODO: Improve OpenApi schema code
// TODO: Research the use of clone() or copy(). Should String arguments be &str?
// TODO: Hide foreign keys from openapi schema
// TODO: what about datetime provided as timestamps
// TODO; Create index.html
// TODO: Test error responses
mod jobs;
mod models;
mod routes;

use std::sync::Arc;
use std::time::Duration;

use database::{DatabaseConnection, Engine, Migrator, MigratorTrait};

use actix_web::dev::ServerHandle;
use actix_web::{middleware::Logger, web, App, HttpServer};

use parking_lot::Mutex;

use rabbitmq_stream_client::types::ByteCapacity;
use rabbitmq_stream_client::{Environment, NoDedup, Producer};

use crate::jobs::Broadcaster;
use crate::routes::router;

// ----------------------------------------------------------------------

struct AppState {
    db: DatabaseConnection,
    producer: Option<Producer<NoDedup>>, // Make optional for unit tests
    stop_handle: StopHandle,
    broadcaster: Arc<Broadcaster>,
}

// ----------------------------------------------------------------------

async fn init() -> AppState {
    tracing_subscriber::fmt().init(); // Log SQL operations

    // Establish connection to database and apply migrations
    let db = Engine::connect().await.unwrap(); // Allow to panic if unsuccessful
    Migrator::up(&db, None).await.unwrap(); // Allow to panic if unsuccessful

    let producer = if !cfg!(test) {
        // TODO: env variable if in dev
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
        Some(environment.producer().build("orders").await.unwrap()) // TODO: Understand dedup
    } else {
        None
    };

    AppState {
        db,
        producer,
        stop_handle: StopHandle::default(),
        broadcaster: Broadcaster::create(),
    }
}

// ----------------------------------------------------------------------

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    let state = web::Data::new(init().await); // Build app state

    let server = HttpServer::new({
        let state = state.clone();
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

    server.await
}

// ----------------------------------------------------------------------

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
        #[allow(clippy::let_underscore_future)]
        let _ = self.inner.lock().as_ref().unwrap().stop(graceful);
    }
}
