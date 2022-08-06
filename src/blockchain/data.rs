use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum TransactionData {
    TransferData {
        sender: String,
        receiver: String,
        amount: f64,
    },
}
