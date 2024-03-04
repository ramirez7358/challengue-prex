use actix_web::{
    get, post, rt::time::sleep, web, HttpResponse, Responder
};
use rust_decimal::Decimal;
use tracing::instrument;
use uuid::Uuid;

use crate::balance_storage::{DataStorage, FileStorage, FILE_PATH};
use crate::{
    model::{AppState, Client, ClientInfo, CreditOrDebitRequest, CreditOrDebitResponse},
    response::{CreateClientResponse, GenericResponse, SingleResponse},
};

/// Configures the service routes for the web application.
///
/// This function sets up the Actix service configuration by adding various service endpoints.
///
/// # Arguments
/// * `conf` - A mutable reference to the ServiceConfig to which the routes will be added.
pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/app")
        .service(health_checker_handler)
        .service(new_client)
        .service(new_credit_transaction)
        .service(new_debit_transaction)
        .service(client_balance)
        .service(store_balances);

    conf.service(scope);
}

/// Creates a new client.
///
/// This endpoint processes a request to create a new client, ensuring no duplicate clients
/// are created based on the document number.
///
/// # Arguments
/// * `body` - The JSON payload containing the client's information.
/// * `data` - Shared application state, specifically a list of current clients.
///
/// # Returns
/// * A success response with the new client's ID, or a conflict response if a duplicate is found.
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

/// Processes a new credit transaction.
///
/// This endpoint applies a credit transaction to a client's account. It adds the specified
/// amount to the client's balance.
///
/// # Arguments
/// * `body` - JSON payload containing the transaction request details.
/// * `data` - Shared application state, including client data.
///
/// # Returns
/// * A response indicating success or failure of the transaction.
#[post("/new_credit_transaction")]
async fn new_credit_transaction(
    body: web::Json<CreditOrDebitRequest>,
    data: web::Data<AppState>,
) -> impl Responder {
    process_transaction(body, data, |client, amount| {
        client.balance += amount;
        Ok(())
    }).await
}

/// Processes a new debit transaction.
///
/// This endpoint applies a debit transaction to a client's account. It deducts the specified
/// amount from the client's balance if sufficient funds are available.
///
/// # Arguments
/// * `body` - JSON payload containing the transaction request details.
/// * `data` - Shared application state, including client data.
///
/// # Returns
/// * A response indicating success or failure of the transaction.
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
    }).await
    
}

/// Stores client balances to a file and resets them in memory.
///
/// This endpoint saves all client balances to a file and then resets each balance to zero.
/// The file is named based on the current date and a count of previously stored files.
///
/// # Arguments
/// * `data` - Shared application state, including client data.
///
/// # Returns
/// * A success response if the balances are stored and reset, or an error response otherwise.
#[post("store_balances")]
async fn store_balances(data: web::Data<AppState>) -> impl Responder {
    let mut vec = data.clients_temp_db.lock().unwrap();
    let store_method = FileStorage::new(FILE_PATH);

    match store_method.store_data(&vec) {
        Ok(_) => {
            vec.iter_mut()
                .for_each(|client| client.balance = Decimal::new(0, 0));
            HttpResponse::Ok().json(SingleResponse {
                status: "success".to_string(),
                data: "Balances stored on file and correctly reset from memory!",
            })
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

/// Retrieves the balance of a specific client.
///
/// This endpoint returns the current balance of a client based on their ID.
///
/// # Arguments
/// * `path` - The path parameter containing the client's ID.
/// * `data` - Shared application state, including client data.
///
/// # Returns
/// * A response with the client's balance or a failure message if the client is not found.
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

/// Health check endpoint for the web application.
///
/// This endpoint provides a basic health check, returning a success message.
///
/// # Returns
/// * A success response with a predefined health check message.
#[get("/healthchecker")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "Build Simple CRUD API with Rust and Actix Web";

    let response_json = &GenericResponse {
        status: "success".to_string(),
        message: MESSAGE.to_string(),
    };
    HttpResponse::Ok().json(response_json)
}

/// Generic function to process credit or debit transactions.
///
/// This function applies a transaction to a client's account based on the provided logic.
/// It is used to encapsulate common logic for credit and debit operations.
///
/// # Type Parameters
/// * `F`: The type of closure that defines the transaction logic.
///
/// # Arguments
/// * `body` - JSON payload containing the transaction request details.
/// * `data` - Shared application state, including client data.
/// * `transaction_logic` - The closure that contains the specific logic for the transaction.
///
/// # Returns
/// * A response indicating success or failure of the transaction.
#[instrument(skip(transaction_logic))]
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
