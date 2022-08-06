use serde::{Deserialize, Serialize};

use super::transaction::Transaction;
use super::utils::timestamp;

#[derive(Clone, Deserialize, Serialize)]
pub struct BlockHeader {
    pub index: usize,
    pub previous_hash: String,
    pub merkle_root: String,
    pub timestamp: u64,
    pub nonce: u64,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Block {
    pub header: BlockHeader,
    pub tx_count: usize,
    pub tx: Vec<Transaction>,
}

impl Block {
    pub fn new(
        index: usize,
        nonce: u64,
        transactions: Vec<Transaction>,
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
            tx: transactions,
        }
    }
}
