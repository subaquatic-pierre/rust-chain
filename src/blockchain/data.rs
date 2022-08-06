use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct TransferData {
    pub sender: String,
    pub receiver: String,
    pub amount: f64,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub enum TransactionData {}
