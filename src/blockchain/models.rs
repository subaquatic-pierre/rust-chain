use std::fmt::Display;

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

impl Display for TransactionData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            TransactionData::TransferData {
                sender,
                receiver,
                amount,
            } => {
                write!(f, "sender:{sender}|receiver:{receiver}|amount:{amount}",)
            }
        }
    }
}
