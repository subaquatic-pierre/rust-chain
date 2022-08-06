use serde::{Deserialize, Serialize};

use super::block::Block;
use super::data::TransferData;
use super::hasher::Hasher;
use super::transaction::{Transaction, TransactionStatus, TransactionType};

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Chain {
    difficulty: usize,
    reward: f64,
    miner_address: String,
    blocks: Vec<Block>,
    current_transactions: Vec<Transaction>,
}

impl Chain {
    pub fn new(difficulty: usize, miner_addr: &str, reward: f64) -> Self {
        let mut chain = Chain {
            difficulty,
            reward,
            miner_address: miner_addr.to_string(),
            blocks: Vec::new(),
            current_transactions: Vec::new(),
        };

        // Mine genesis block
        chain.genesis_block();
        chain
    }

    // ---
    // Accessor methods
    // ---

    pub fn blocks(&self) -> &Vec<Block> {
        &self.blocks
    }

    pub fn current_transactions(&self) -> &Vec<Transaction> {
        &self.current_transactions
    }

    // ---
    // Public methods
    // ---

    pub fn mine_new_block(&mut self) -> &Block {
        // Get previous block info
        let last_block = self.blocks.last().unwrap();
        let last_nonce = last_block.header.nonce;
        let previous_hash = last_block.header.merkle_root.clone();

        // Run proof of work algorithm
        let nonce = self.proof_of_work(last_nonce);

        // Get new block info
        let index = self.blocks.len();

        // Create empty transaction array for new block
        let mut transactions: Vec<Transaction> = Vec::new();

        // Remove all current transactions from blocks, add new new transaction vec for new block
        while self.current_transactions.len() > 0 {
            transactions.push(self.current_transactions.pop().unwrap())
        }

        // Create new reward transaction
        let data = TransferData {
            sender: "Root".to_string(),
            receiver: self.miner_address.clone(),
            amount: self.reward,
        };
        let reward_transaction = Transaction::new(data, TransactionType::Reward);

        // Add reward transaction to block transaction vec
        transactions.push(reward_transaction);

        for transaction in &mut transactions {
            *transaction.status = TransactionStatus::Confirmed
        }

        // Get new merkle_root root of transactions in block
        let merkle_root = Hasher::merkle_root(&transactions);

        // Make new block
        let block = Block::new(index, nonce, transactions, &merkle_root, &previous_hash);

        // Append block to chain
        self.blocks.push(block);
        self.blocks.last().unwrap()
    }

    pub fn add_transaction<'a>(&mut self, transaction: &'a mut Transaction) -> &'a Transaction {
        transaction.status = TransactionStatus::Unconfirmed;
        self.current_transactions.push(transaction.clone());
        transaction
    }

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

    fn genesis_block(&mut self) {
        // Get previous block info
        let last_nonce = 0;
        let previous_hash = [0; 64]
            .iter()
            .map(ToString::to_string)
            .collect::<String>()
            .to_string();

        // Run proof of work algorithm
        let nonce = self.proof_of_work(last_nonce);

        // Get new block info
        let index = self.blocks.len();

        // Create empty transaction array for new block
        let mut transactions: Vec<Transaction> = Vec::new();

        // Remove all current transactions from blocks, add new new transaction vec for new block
        while self.current_transactions.len() > 0 {
            transactions.push(self.current_transactions.pop().unwrap())
        }

        // Create new reward transaction
        let data = TransferData {
            sender: "Root".to_string(),
            receiver: self.miner_address.clone(),
            amount: self.reward,
        };
        let reward_transaction = Transaction::new(data, TransactionType::GenesisReward);

        // Add reward transaction to block transaction vec
        transactions.push(reward_transaction);

        for transaction in &mut transactions {
            *transaction.status = TransactionStatus::Confirmed
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

    pub fn new_transfer_transaction(sender: &str, receiver: &str, amount: f64) -> Transaction {
        let data = TransferData {
            sender: sender.to_string(),
            receiver: receiver.to_string(),
            amount,
        };
        let transaction = Transaction::new(data, TransactionType::Transfer);
        transaction
    }
}

#[cfg(test)]
mod tests {
    use crate::blockchain::transaction::TransactionType;
    use crate::{DIFFICULTY_LEVEL, MINER_ADDRESS, REWARD};

    use super::*;

    // #[test]
    // fn add_transaction() {
    //     let mut chain = Chain::new(DIFFICULTY_LEVEL, MINER_ADDRESS, REWARD);

    //     let mut transaction_1 = Transaction::new("me", "you", 1.0, TransactionType::Transfer);

    //     chain.add_transaction(&mut transaction_1);

    //     assert_eq!(transaction_1.status, TransactionStatus::Unconfirmed);
    //     assert_eq!(chain.current_transactions().len(), 1);

    //     let mut transaction_2 = Transaction::new("me", "you", 1.0, TransactionType::Transfer);

    //     chain.add_transaction(&mut transaction_2);

    //     assert_eq!(transaction_2.status, TransactionStatus::Unconfirmed);
    //     assert_eq!(chain.current_transactions().len(), 2);
    // }
}
