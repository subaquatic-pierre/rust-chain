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
        models::TransactionData,
        transaction::{Transaction, TransactionType},
    },
};

#[derive(Serialize, Deserialize)]
pub struct CreateTransactionRequest {
    sender: String,
    receiver: String,
    amount: f64,
    signature: String,
    tx_type: TransactionType,
}

#[derive(Serialize)]
pub struct CreateTransactionResponse {
    next_index: usize,
    transaction: Transaction,
}

#[post("/create-transaction")]
async fn create_transaction(
    app: Data<AppState>,
    new_tx: Json<CreateTransactionRequest>,
) -> HttpResponse {
    let tx_data = TransactionData::TransferData {
        sender: new_tx.sender.clone(),
        receiver: new_tx.receiver.clone(),
        amount: new_tx.amount,
    };
    let mut transaction = Chain::new_transaction(tx_data, new_tx.tx_type);

    let mut chain = app.chain.lock().unwrap();

    // Return http error if transaction not verifiable
    match chain.add_transaction(&mut transaction, &new_tx.sender, &new_tx.signature) {
        Ok(tx) => HttpResponse::Ok().json(CreateTransactionResponse {
            next_index: chain.current_tx().len(),
            transaction: tx.clone(),
        }),
        Err(_) => HttpResponse::Forbidden().json("Transaction not verified"),
    }
}

#[get("/list-current-transactions")]
async fn list_current_transactions(app: Data<AppState>) -> HttpResponse {
    let chain = app.chain.lock().unwrap();
    let transactions = chain.current_tx();

    HttpResponse::Ok().json(transactions)
}

#[get("/{tx_hash}")]
async fn get_transaction(tx_hash: Path<String>, app: Data<AppState>) -> HttpResponse {
    let chain = app.chain.lock().unwrap();

    let tx = chain.get_transaction(&tx_hash.to_string());

    match tx {
        Some(tx) => HttpResponse::Ok().json(tx),
        None => HttpResponse::NotFound().json("Not found"),
    }
}

pub fn register_transaction_service() -> Scope {
    scope("transaction")
        .service(create_transaction)
        .service(list_current_transactions)
        .service(get_transaction)
}
