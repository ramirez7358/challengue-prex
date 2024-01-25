use actix_web::{get, post, web, HttpResponse, Responder};
use uuid::Uuid;

use crate::{
    model::{AppState, Client},
    response::{CreateClientResponse, GenericResponse, SingleResponse},
};

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/app")
        .service(health_checker_handler)
        .service(new_client);

    conf.service(scope);
}

#[post("/new_client")]
async fn new_client(mut body: web::Json<Client>, data: web::Data<AppState>) -> impl Responder {
    let mut vec = data.clients_temp_db.lock().unwrap();


    if vec
        .iter()
        .any(|client| client.document_number == body.document_number)
    {
        return HttpResponse::Conflict().json(GenericResponse {
            status: "fail".to_string(),
            message: format!(
                "Client with document: '{}' already exists",
                body.document_number
            ),
        });
    }

    let uuid_id = Uuid::new_v4().to_string();
    body.id = Some(uuid_id.clone());

    vec.push(body.into_inner());

    HttpResponse::Ok().json(SingleResponse {
        status: "success".to_string(),
        data: CreateClientResponse { id: uuid_id },
    })
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

#[get("/healthchecker")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "Build Simple CRUD API with Rust and Actix Web";

    let response_json = &GenericResponse {
        status: "success".to_string(),
        message: MESSAGE.to_string(),
    };
    HttpResponse::Ok().json(response_json)
}
