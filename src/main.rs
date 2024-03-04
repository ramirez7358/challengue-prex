mod balance_storage;
mod handler;
mod model;
mod response;

use actix_web::middleware::Logger;
use actix_web::{ web, App, HttpServer };
use tracing_subscriber::fmt::format::FmtSpan;

use crate::model::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = tracing_subscriber
        ::fmt()
        // Use a more compact, abbreviated log format
        .compact()
        // Display source code file paths
        .with_file(true)
        // Display source code line numbers
        .with_line_number(true)
        // Display the thread ID an event was recorded on
        .with_thread_ids(true)
        // Don't display the event's target (module path)
        .with_target(true)
        .with_span_events(FmtSpan::CLOSE)
        // Build the subscriber
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    env_logger::init();

    let memory_db = AppState::init();
    let app_data = web::Data::new(memory_db);

    println!("🚀 Server started successfully. Listening on port 8080! 🚀");

    HttpServer::new(move || {
        App::new().app_data(app_data.clone()).configure(handler::config).wrap(Logger::default())
    })
        .bind(("127.0.0.1", 8080))?
        .run().await
}
