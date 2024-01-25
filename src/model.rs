use std::sync::{Arc, Mutex};

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Client {
    pub id: Option<String>,
    pub name: String,
    pub birth_date: String,
    pub document_number: String,
    pub country: String,
    pub balance: Option<Decimal>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreditRequest {
    pub client_id: String,
    pub credit_amount: Decimal
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreditResponse {
    pub client_id: String,
    pub balance: Decimal
}

pub struct AppState {
    pub clients_temp_db: Arc<Mutex<Vec<Client>>>
}

impl AppState {
    pub fn init() -> AppState {
        AppState {
            clients_temp_db: Arc::new(Mutex::new(Vec::new()))
        }
    }
}