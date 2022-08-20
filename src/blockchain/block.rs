use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::transaction::Transaction;
use super::utils::timestamp;

#[derive(Clone, Serialize)]
pub struct BlockHeader {
    pub index: usize,
    pub previous_hash: String,
    pub merkle_root: String,
    pub timestamp: u64,
    pub nonce: u64,
}

#[derive(Clone, Serialize)]
pub struct Block {
    pub header: BlockHeader,
    pub tx_count: usize,
    pub txs: Vec<Transaction>,
}

impl Block {
    pub fn new(
        index: usize,
        nonce: u64,
        new_txs: Vec<Transaction>,
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

        let mut txs = Vec::new();

        for tx in new_txs {
            txs.push(tx);
        }

        Block {
            header,
            tx_count: txs.len(),
            txs,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn block_header() {
        let header = BlockHeader {
            index: 1,
            nonce: 2,
            previous_hash: "prev_hash".to_string(),
            merkle_root: "merkle_root".to_string(),
            timestamp: 3,
        };

        assert_eq!(header.index, 1);
        assert_eq!(header.nonce, 2);
        assert_eq!(header.previous_hash, "prev_hash");
        assert_eq!(header.merkle_root, "merkle_root");
        assert_eq!(header.timestamp, 3);
    }

    #[test]
    fn new_block() {
        let block = Block::new(1, 2, Vec::new(), "merkle", "prev_hash");

        assert_eq!(block.header.index, 1);
        assert_eq!(block.header.nonce, 2);
        assert!(block.txs.is_empty());
        assert_eq!(block.header.merkle_root, "merkle");
        assert_eq!(block.header.previous_hash, "prev_hash");
    }
}
