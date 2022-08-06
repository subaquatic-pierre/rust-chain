use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

use super::hasher::Hasher;
use super::models::TransactionData;
use super::utils::timestamp;

#[derive(Clone, Deserialize, Serialize)]
pub struct Transaction {
    pub hash: String,
    pub timestamp: u64,
    pub tx_type: TransactionType,
    pub status: TransactionStatus,
    pub data: TransactionData,
}

impl Transaction {
    pub fn new(data: TransactionData, tx_type: TransactionType) -> Self {
        let timestamp = timestamp();
        let pre_hash_data = format!("timestamp:{timestamp}|{}", data.clone());
        println!("{pre_hash_data}");
        let hash = Hasher::hash_serializable(pre_hash_data);
        Transaction {
            hash,
            timestamp,
            data,
            tx_type,
            status: TransactionStatus::Created,
        }
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

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
pub enum TransactionType {
    Transfer,
    Reward,
    GenesisReward,
}

#[cfg(test)]
mod test {
    use super::*;
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
