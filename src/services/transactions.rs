use actix_web::{
    get, post,
    web::{scope, Data, Json},
    HttpResponse, Scope,
};
use serde::{Deserialize, Serialize};

use crate::{
    app::AppState,
    blockchain::{
        blockchain::Chain,
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
pub struct CreateTransactionResponse<T>
where
    T: Serialize,
{
    next_index: usize,
    transaction: Transaction<T>,
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

    let mut blockchain = app.blockchain.lock().unwrap();

    blockchain.add_transaction(&mut transaction);

    match transaction.status {
        TransactionStatus::Unconfirmed => HttpResponse::Ok().json(CreateTransactionResponse {
            next_index: blockchain.current_transactions().len(),
            transaction,
        }),
        _ => HttpResponse::InternalServerError().body("Error adding transaction to blockchain"),
    }
}
#[get("/list-current-transactions")]
async fn list_current_transactions(app: Data<AppState>) -> HttpResponse {
    let blockchain = app.blockchain.lock().unwrap();
    let transactions = blockchain.current_transactions();

    HttpResponse::Ok().json(transactions)
}

pub fn register_transaction_service() -> Scope {
    scope("transaction")
        .service(create_transaction)
        .service(list_current_transactions)
}
