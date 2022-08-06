use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct TransferData {
    sender: String,
    receiver: String,
    amount: f64,
}

impl TransferData {
    pub fn new(sender: &str, receiver: &str, amount: f64) -> Self {
        TransferData {
            sender: sender.to_string(),
            receiver: receiver.to_string(),
            amount,
        }
    }
}
