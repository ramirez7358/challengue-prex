use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;

#[post("/new_client")]
async fn new_client() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/new_credit_transaction")]
async fn new_credit_transaction(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[post("new_debit_transaction")]
async fn new_debit_transaction(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[post("store_balances")]
async fn store_balances(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("client_balance")]
async fn client_balance(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[get("/healthchecker")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "Build Simple CRUD API with Rust and Actix Web";

    let response_json = &GenericResponse {
        status: "success".to_string(),
        message: MESSAGE.to_string(),
    };
    HttpResponse::Ok().json(response_json)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info")
    }

    env_logger::init();

    println!("🚀 Server started successfully. Listening on port 8080! 🚀");
    

    HttpServer::new(move || App::new().service(web::scope("/app").service(health_checker_handler)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
