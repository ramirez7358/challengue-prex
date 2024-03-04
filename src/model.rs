use std::sync::{Arc, Mutex};

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ClientInfo {
    pub name: String,
    pub birth_date: String,
    pub document_number: String,
    pub country: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Client {
    pub id: String,
    pub balance: Decimal,
    pub info: ClientInfo,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreditOrDebitRequest {
    pub client_id: String,
    pub amount: Decimal,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreditOrDebitResponse {
    pub client_id: String,
    pub new_balance: Decimal,
}

#[derive(Debug)]
pub struct AppState {
    pub clients_temp_db: Arc<Mutex<Vec<Client>>>,
}

impl AppState {
    pub fn init() -> AppState {
        AppState {
            clients_temp_db: Arc::new(Mutex::new(Vec::new())),
        }
    }
}
