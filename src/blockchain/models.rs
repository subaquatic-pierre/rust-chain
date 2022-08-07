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
    LoginData {
        user: String,
        timestamp: u64,
    },
}

impl Display for TransactionData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Self::TransferData {
                sender,
                receiver,
                amount,
            } => {
                write!(f, "sender:{sender}|receiver:{receiver}|amount:{amount}",)
            }
            Self::LoginData { user, timestamp } => {
                write!(f, "user:{user}|timestamp:{timestamp}",)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn display_transfer_data() {
        let tx_data = TransactionData::TransferData {
            sender: "me".to_string(),
            receiver: "you".to_string(),
            amount: 10.1,
        };

        assert_eq!(
            format!("{tx_data}"),
            format!("sender:me|receiver:you|amount:10.1")
        );
    }
}
