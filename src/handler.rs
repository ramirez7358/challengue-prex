use actix_web::{
    get, post,
    web::{self},
    HttpResponse, Responder,
};
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::{
    model::{AppState, Client, ClientInfo, CreditOrDebitRequest, CreditOrDebitResponse},
    response::{CreateClientResponse, GenericResponse, SingleResponse},
};

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/app")
        .service(health_checker_handler)
        .service(new_client)
        .service(new_credit_transaction)
        .service(new_debit_transaction)
        .service(client_balance);

    conf.service(scope);
}

#[post("/new_client")]
async fn new_client(body: web::Json<ClientInfo>, data: web::Data<AppState>) -> impl Responder {
    let mut vec = data.clients_temp_db.lock().unwrap();

    if vec
        .iter()
        .any(|client| client.info.document_number == body.document_number)
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

    vec.push(Client {
        id: uuid_id.clone(),
        balance: Decimal::new(0, 0),
        info: body.to_owned(),
    });

    HttpResponse::Ok().json(SingleResponse {
        status: "success".to_string(),
        data: CreateClientResponse { id: uuid_id },
    })
}

#[post("/new_credit_transaction")]
async fn new_credit_transaction(
    body: web::Json<CreditOrDebitRequest>,
    data: web::Data<AppState>,
) -> impl Responder {
    process_transaction(body, data, |client, amount| {
        client.balance += amount;
        Ok(())
    })
    .await
}

#[post("new_debit_transaction")]
async fn new_debit_transaction(
    body: web::Json<CreditOrDebitRequest>,
    data: web::Data<AppState>,
) -> impl Responder {
    process_transaction(body, data, |client, amount| {
        if client.balance < amount {
            Err("Insufficient balance to debit!")
        } else {
            client.balance -= amount;
            Ok(())
        }
    })
    .await
}

#[post("store_balances")]
async fn store_balances(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("client_balance/{client_id}")]
async fn client_balance(path: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let mut vec = data.clients_temp_db.lock().unwrap();
    let client_id = path.into_inner();
    if let Some(client) = vec.iter_mut().find(|client| client.id.eq(&client_id)) {
        HttpResponse::Ok().json(SingleResponse {
            status: "success".to_string(),
            data: client,
        })
    } else {
        HttpResponse::Conflict().json(GenericResponse {
            status: "fail".to_string(),
            message: format!("Client with id: '{}' doesn't exist", client_id),
        })
    }
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

async fn process_transaction<F>(
    body: web::Json<CreditOrDebitRequest>,
    data: web::Data<AppState>,
    transaction_logic: F,
) -> impl Responder
where
    F: FnOnce(&mut Client, Decimal) -> Result<(), &'static str>,
{
    let mut vec = data.clients_temp_db.lock().unwrap();

    if let Some(client) = vec.iter_mut().find(|client| client.id.eq(&body.client_id)) {
        match transaction_logic(client, body.amount) {
            Ok(_) => HttpResponse::Ok().json(SingleResponse {
                status: "success".to_string(),
                data: CreditOrDebitResponse {
                    client_id: body.client_id.to_owned(),
                    new_balance: client.balance,
                },
            }),
            Err(message) => HttpResponse::Conflict().json(GenericResponse {
                status: "fail".to_string(),
                message: message.to_string(),
            }),
        }
    } else {
        HttpResponse::Conflict().json(GenericResponse {
            status: "fail".to_string(),
            message: format!("Client with id: '{}' doesn't exist", body.client_id),
        })
    }
}
