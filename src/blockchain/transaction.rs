use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

use super::models::TransactionData;

#[derive(Clone, Deserialize, Serialize)]
pub struct Transaction {
    pub hash: String,
    pub timestamp: u64,
    pub status: TransactionStatus,
    pub tx_type: TransactionType,
    pub tx_data: TransactionData,
}

impl Transaction {
    pub fn new(
        tx_data: TransactionData,
        tx_type: TransactionType,
        hash: &str,
        timestamp: u64,
    ) -> Self {
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
    // use super::*;
    // #[test]
    // fn new_transaction() {
    //     let transaction = Transaction::new();

    //     assert_eq!(transaction.sender, "me");
    //     assert_eq!(transaction.receiver, "you");
    //     assert_eq!(transaction.amount, 1.0);
    //     assert_eq!(transaction.tx_type, TransactionType::Transfer);
    //     assert_eq!(transaction.status, TransactionStatus::Created);
    // }
}
