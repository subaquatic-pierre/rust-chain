use serde::{Deserialize, Serialize};

use super::transaction::Transaction;
use super::utils::timestamp;

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct BlockHeader {
    pub index: usize,
    pub previous_hash: String,
    pub merkle_root: String,
    pub timestamp: u64,
    pub nonce: u64,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Block<T> {
    pub header: BlockHeader,
    pub tx_count: usize,
    pub data: Vec<Transaction<T>>,
}

impl<T> Block<T> {
    pub fn new(
        index: usize,
        nonce: u64,
        transactions: Vec<Transaction<T>>,
        merkle_root: &str,
        previous_hash: &str,
    ) -> Self {
        let header = BlockHeader {
            index,
            nonce,
            previous_hash: previous_hash.to_string(),
            merkle_root: merkle_root.to_string(),
            timestamp: timestamp(),
        };

        Block {
            header,
            tx_count: transactions.len(),
            data: transactions,
        }
    }
}
