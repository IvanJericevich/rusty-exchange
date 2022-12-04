// TODO: Improve error documentation
// TODO: Make sure swagger-docs have the correct headers

mod models;
mod routes;

use database::{DatabaseConnection, Engine, Migrator, MigratorTrait};

use actix_web::{middleware::Logger, web, App, HttpServer};

use routes::router;

// ----------------------------------------------------------------------

#[derive(Debug, Clone)]
struct AppState {
    db: DatabaseConnection,
}

#[actix_web::main]
async fn run() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    tracing_subscriber::fmt().init(); // Log SQL operations

    // Establish connection to database and apply migrations
    let db = Engine::connect().await.unwrap(); // Allow to panic if unsuccessful
    Migrator::up(&db, None).await.unwrap(); // Allow to panic if unsuccessful

    let state = AppState { db }; // Build app state

    HttpServer::new(move || {
        let logger = Logger::new("%r %s (%Ts)");

        App::new()
            .wrap(logger)
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
