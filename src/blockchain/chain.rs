use serde::{Deserialize, Serialize};

use super::block::Block;
use super::hasher::Hasher;
use super::models::TransactionData;
use super::transaction::{Transaction, TransactionStatus, TransactionType};
use super::utils::timestamp;

#[derive(Clone, Deserialize, Serialize)]
pub struct Chain {
    difficulty: usize,
    reward: f64,
    miner_address: String,
    blocks: Vec<Block>,
    current_tx: Vec<Transaction>,
}

impl Chain {
    pub fn new(difficulty: usize, miner_addr: &str, reward: f64) -> Self {
        let blocks = Chain::get_blocks();
        let mut chain = Chain {
            difficulty,
            reward,
            blocks,
            miner_address: miner_addr.to_string(),
            current_tx: Vec::new(),
        };

        // TODO: REMOVE FROM CODEBASE IN PRODUCTION
        if chain.blocks.len() == 0 {
            chain.genesis_block()
        };
        chain
    }

    // ---
    // Public methods
    // ---

    pub fn mine_new_block(&mut self) -> &Block {
        if self.current_tx.len() == 0 {
            return self.blocks.last().unwrap();
        }
        // Get previous block info
        let last_block = self.blocks.last().unwrap();
        let last_nonce = last_block.header.nonce;
        let previous_hash = last_block.header.merkle_root.clone();

        // Run proof of work algorithm
        let nonce = self.proof_of_work(last_nonce);

        // Get new block info
        let index = self.blocks.len();

        // Create empty tx array for new block
        let mut transactions: Vec<Transaction> = Vec::new();

        // Remove all current transactions from blocks, add new new tx vec for new block
        while self.current_tx.len() > 0 {
            transactions.push(self.current_tx.pop().unwrap())
        }

        // Create new reward tx
        let data = TransactionData::TransferData {
            sender: "Root".to_string(),
            receiver: self.miner_address.clone(),
            amount: self.reward,
        };
        let reward_tx = Chain::new_transaction(data, TransactionType::Reward);

        // Add reward tx to block tx vec
        transactions.push(reward_tx);

        for tx in &mut transactions {
            *tx.status = TransactionStatus::Confirmed
        }

        // Get new merkle_root root of transactions in block
        let merkle_root = Hasher::merkle_root(&transactions);

        // Make new block
        let block = Block::new(index, nonce, transactions, &merkle_root, &previous_hash);

        // Append block to chain
        self.blocks.push(block);
        self.blocks.last().unwrap()
    }

    pub fn add_transaction<'a>(
        &mut self,
        tx: &'a mut Transaction,
        sender: &str,
        signature: &str,
    ) -> Result<&'a Transaction, ()> {
        // Verify transaction before adding to current tx vec
        match tx.verify(sender, signature) {
            true => {
                tx.status = TransactionStatus::Unconfirmed;
                self.current_tx.push(tx.clone());
                Ok(tx)
            }
            _ => Err(()),
        }
    }

    pub fn get_transaction(&self, tx_hash: &str) -> Option<Transaction> {
        let tx: Option<Transaction> = None;

        // Find tx in current transactions
        for curr_tx in &self.current_tx {
            if curr_tx.hash == tx_hash {
                return Some(curr_tx.clone());
            }
        }

        // Not found in current tx, search blocks
        for block in self.blocks() {
            match block.tx.get(tx_hash) {
                Some(tx) => return Some(tx.clone()),
                None => continue,
            };
        }
        tx
    }

    // ---
    // Accessor methods
    // ---

    pub fn blocks(&self) -> &Vec<Block> {
        &self.blocks
    }

    pub fn current_tx(&self) -> &Vec<Transaction> {
        &self.current_tx
    }

    // ---
    // Setter methods
    // ---

    pub fn set_difficulty(&mut self, difficulty: usize) {
        self.difficulty = difficulty
    }

    pub fn set_reward(&mut self, reward: f64) {
        self.reward = reward
    }

    pub fn reward(&self) -> f64 {
        self.reward
    }
    pub fn difficulty(&self) -> usize {
        self.difficulty
    }

    // ---
    // Private methods
    // ---

    fn proof_of_work(&self, last_nonce: u64) -> u64 {
        let mut nonce: u64 = 0;

        while !self.valid_proof(last_nonce, nonce) {
            nonce += 1;
        }
        nonce
    }

    fn valid_proof(&self, last_nonce: u64, nonce: u64) -> bool {
        let guess = format!("{last_nonce:}{nonce:}");
        let hashed_guess = Hasher::hash_serializable(guess);

        let last_chars = &hashed_guess[hashed_guess.len() - self.difficulty..];

        let mut difficulty_string = String::new();

        for _ in 0..self.difficulty {
            difficulty_string.push_str("0");
        }

        last_chars == difficulty_string
    }

    fn get_blocks() -> Vec<Block> {
        // TODO: GET BLOCKS FROM STORAGE
        Vec::new()
    }

    fn genesis_block(&mut self) {
        // Build block info
        let nonce = 1;
        let index = 0;
        let previous_hash = [0; 64]
            .iter()
            .map(ToString::to_string)
            .collect::<String>()
            .to_string();

        // Create empty tx array for new block
        let mut transactions: Vec<Transaction> = Vec::new();

        // Create new reward tx
        let data = TransactionData::TransferData {
            sender: "Root".to_string(),
            receiver: self.miner_address.clone(),
            amount: self.reward,
        };

        let reward_tx = Chain::new_transaction(data, TransactionType::GenesisReward);

        // Add reward tx to block tx vec
        transactions.push(reward_tx);

        // Update all tx status to confirmed
        for tx in &mut transactions {
            *tx.status = TransactionStatus::Confirmed
        }

        // Get new merkle_root root of transactions in block
        let merkle_root = Hasher::merkle_root(&transactions);

        // Make new block
        let block = Block::new(index, nonce, transactions, &merkle_root, &previous_hash);

        // Append block to blocks
        self.blocks.push(block);
    }

    // ---
    // Static methods
    // ---

    pub fn new_transaction(tx_data: TransactionData, tx_type: TransactionType) -> Transaction {
        let timestamp = timestamp();
        let tx = Transaction::new(tx_data, tx_type, timestamp);
        tx
    }
}

#[cfg(test)]
mod tests {
    // use crate::blockchain::transaction::TransactionType;
    // use crate::{DIFFICULTY_LEVEL, MINER_ADDRESS, REWARD};

    // use super::*;

    // #[test]
    // fn add_transaction() {
    //     let mut chain = Chain::new(DIFFICULTY_LEVEL, MINER_ADDRESS, REWARD);

    //     let mut transaction_1 = Transaction::new("me", "you", 1.0, TransactionType::Transfer);

    //     chain.add_transaction(&mut transaction_1);

    //     assert_eq!(transaction_1.status, TransactionStatus::Unconfirmed);
    //     assert_eq!(chain.current_tx().len(), 1);

    //     let mut transaction_2 = Transaction::new("me", "you", 1.0, TransactionType::Transfer);

    //     chain.add_transaction(&mut transaction_2);

    //     assert_eq!(transaction_2.status, TransactionStatus::Unconfirmed);
    //     assert_eq!(chain.current_tx().len(), 2);
    // }
}
