use actix_web::{
    get, post,
    web::{scope, Data, Json, Path},
    HttpResponse, Scope,
};
use serde::{Deserialize, Serialize};

use crate::{
    app::AppState,
    blockchain::{
        chain::Chain,
        transaction::{Transaction, TransactionStatus, TransactionType},
    },
};

#[derive(Serialize, Deserialize)]
pub struct CreateTransactionRequest {
    sender: String,
    receiver: String,
    amount: f64,
    transaction_type: TransactionType,
}

#[derive(Serialize, Deserialize)]
pub struct CreateTransactionResponse {
    next_index: usize,
    transaction: Transaction,
}

#[post("/create-transaction")]
async fn create_transaction(
    app: Data<AppState>,
    new_transaction: Json<CreateTransactionRequest>,
) -> HttpResponse {
    let mut transaction = Chain::new_transfer_transaction(
        &new_transaction.sender,
        &new_transaction.receiver,
        new_transaction.amount,
    );

    let mut chain = app.chain.lock().unwrap();

    chain.add_transaction(&mut transaction);

    match transaction.status {
        TransactionStatus::Unconfirmed => HttpResponse::Ok().json(CreateTransactionResponse {
            next_index: chain.current_tx().len(),
            transaction,
        }),
        _ => HttpResponse::InternalServerError().body("Error adding transaction to chain"),
    }
}

#[get("/list-current-transactions")]
async fn list_current_transactions(app: Data<AppState>) -> HttpResponse {
    let chain = app.chain.lock().unwrap();
    let transactions = chain.current_tx();

    HttpResponse::Ok().json(transactions)
}

#[get("/{tx_hash}")]
async fn get_transaction(info: Path<String>, app: Data<AppState>) -> HttpResponse {
    let chain = app.chain.lock().unwrap();
    let transactions = chain.current_tx();

    HttpResponse::Ok().json(transactions)
}

pub fn register_transaction_service() -> Scope {
    scope("transaction")
        .service(create_transaction)
        .service(list_current_transactions)
        .service(get_transaction)
}
