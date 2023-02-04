// TODO: what about datetime provided as timestamps
mod jobs;
mod models;
mod routes;

use std::env;
use std::sync::Arc;

use database::{DatabaseConnection, Engine, Migrator, MigratorTrait}; // TODO: move this to prelude

use actix_web::dev::ServerHandle;
use actix_web::{middleware::Logger, web, App, HttpServer};

use parking_lot::Mutex;

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

    let enable_rabbitmq = env::args().any(|arg| arg.to_lowercase() == "enable_rabbitmq");

    let db = Engine::connect().await.unwrap();
    Migrator::up(&db, None).await.unwrap(); // Always run migration

    let producer = if !cfg!(test) && enable_rabbitmq {
        // Establish connection to RabbitMQ
        Some(
            Environment::builder()
                .host("localhost")
                .port(5552)
                .build()
                .await
                .unwrap()
                .producer()
                .build("orders")
                .await
                .unwrap(), // TODO: Should this be Arc? check the internals
        )
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
