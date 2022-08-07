use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

use super::hasher::Hasher;
use super::models::TransactionData;

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Transaction {
    pub hash: String,
    pub timestamp: u64,
    pub status: TransactionStatus,
    pub tx_type: TransactionType,
    pub tx_data: TransactionData,
}

impl Transaction {
    pub fn new(tx_data: TransactionData, tx_type: TransactionType, timestamp: u64) -> Self {
        let hash = Hasher::hash_serializable(format!("timestamp:{timestamp}|{}", &tx_data));

        Transaction {
            hash: hash.to_string(),
            timestamp,
            tx_data,
            tx_type,
            status: TransactionStatus::Created,
        }
    }

    pub fn verify(&self, _sender: &str, _signature: &str) -> bool {
        // TODO: VERIFY SIGNATURE AGAINST tx_data
        true
    }
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
pub enum TransactionStatus {
    Created,
    Unconfirmed,
    Confirmed,
}

impl Deref for TransactionStatus {
    type Target = TransactionStatus;
    fn deref(&self) -> &Self::Target {
        &self
    }
}

impl DerefMut for TransactionStatus {
    fn deref_mut(&mut self) -> &mut TransactionStatus {
        self
    }
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, Copy)]
pub enum TransactionType {
    Transfer,
    Reward,
    GenesisReward,
}

#[cfg(test)]
mod test {
    use super::*;
    use test_utils::new_tx;

    #[test]
    fn new_transfer_transaction() {
        let tx = new_tx();

        match tx.tx_data {
            TransactionData::TransferData {
                sender,
                receiver,
                amount,
            } => {
                assert!(sender == "me");
                assert!(receiver == "you");
                assert!(amount == 10.0);
            }
            _ => (),
        }
        assert_eq!(tx.tx_type, TransactionType::Transfer);
        assert_eq!(tx.status, TransactionStatus::Created);
    }

    #[test]
    fn verify_transaction() {
        let tx = new_tx();

        assert_eq!(tx.verify("me", "signature"), true);
    }

    mod test_utils {
        use super::*;

        pub fn new_tx() -> Transaction {
            let tx_data = TransactionData::TransferData {
                sender: "me".to_string(),
                receiver: "you".to_string(),
                amount: 10.0,
            };
            let tx = Transaction::new(tx_data, TransactionType::Transfer, 1);
            tx
        }
    }
}
