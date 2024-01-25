mod handler;
mod model;
mod response;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};

use crate::model::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info")
    }

    env_logger::init();

    let memory_db = AppState::init();
    let app_data = web::Data::new(memory_db);

    println!("🚀 Server started successfully. Listening on port 8080! 🚀");

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .configure(handler::config)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
