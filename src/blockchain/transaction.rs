use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

use super::hasher::Hash;
use super::models::TransactionData;

#[derive(Clone, Serialize, Debug)]
pub struct Transaction {
    pub hash: Hash,
    pub timestamp: u64,
    pub status: TransactionStatus,
    pub tx_type: TransactionType,
    pub tx_data: TransactionData,
}

impl Transaction {
    pub fn new(
        tx_data: TransactionData,
        tx_type: TransactionType,
        timestamp: u64,
        hash: Hash,
    ) -> Self {
        Transaction {
            hash,
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
        self
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

        // match tx.tx_data {
        //     TransactionData::TransferData {
        //         sender,
        //         receiver,
        //         amount,
        //     } => {
        //         assert!(sender == "me");
        //         assert!(receiver == "you");
        //         assert!(amount == 10.0);
        //     }
        //     _ => (),
        // }

        if let TransactionData::TransferData {
            sender,
            receiver,
            amount,
        } = tx.tx_data
        {
            assert!(sender == "me");
            assert!(receiver == "you");
            assert!(amount == 10.0);
        }

        assert_eq!(tx.tx_type, TransactionType::Transfer);
        assert_eq!(tx.status, TransactionStatus::Created);
    }

    #[test]
    fn verify_transaction() {
        let tx = new_tx();

        assert!(tx.verify("me", "signature"));
    }

    mod test_utils {
        use super::*;
        use crate::blockchain::{hasher::Hasher, utils::timestamp};

        pub fn new_tx() -> Transaction {
            let tx_data = TransactionData::TransferData {
                sender: "me".to_string(),
                receiver: "you".to_string(),
                amount: 10.0,
            };
            let timestamp = timestamp();
            let hash = Hasher::hash_tx_data(&tx_data, timestamp);
            Transaction::new(tx_data, TransactionType::Transfer, 1, hash)
        }
    }
}
