use serde::{Deserialize, Serialize};

use super::block::Block;
use super::data::TransferData;
use super::hasher::Hasher;
use super::transaction::{Transaction, TransactionStatus, TransactionType};

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Chain<T> {
    difficulty: usize,
    reward: f64,
    miner_address: String,
    chain: Vec<Block<T>>,
    current_transactions: Vec<Transaction<T>>,
}

impl<T> Chain<T> {
    pub fn new(difficulty: usize, miner_addr: &str, reward: f64) -> Self {
        let mut chain = Chain {
            difficulty,
            reward,
            miner_address: miner_addr.to_string(),
            chain: Vec::new(),
            current_transactions: Vec::new(),
        };

        // Mine genesis block
        chain.mine_new_block();
        chain
    }

    // ---
    // Accessor methods
    // ---

    pub fn blocks(&self) -> &Vec<Block<T>> {
        &self.chain
    }

    pub fn current_transactions(&self) -> &Vec<Transaction<T>> {
        &self.current_transactions
    }

    // ---
    // Public methods
    // ---

    pub fn mine_new_block(&mut self) -> &Block<T>
    where
        T: Serialize,
    {
        // Get previous block info
        let last_nonce = self.last_block_nonce();
        let previous_hash = self.last_block_hash();

        // Run proof of work algorithm
        let nonce = self.proof_of_work(last_nonce);

        // Get new block info
        let index = self.chain.len();

        // Create empty transaction array for new block
        let mut transactions: Vec<Transaction<T>> = Vec::new();

        // Remove all current transactions from chain, add new new transaction vec for new block
        while self.current_transactions.len() > 0 {
            transactions.push(self.current_transactions.pop().unwrap())
        }

        // Check if genesis hash
        let tx_type = if previous_hash == Chain::genesis_hash() {
            TransactionType::GenesisReward
        } else {
            TransactionType::Reward
        };

        // Create new reward transaction
        let data = TransferData::new("Root", &self.miner_address, self.reward);
        let reward_transaction = Transaction::new(data, tx_type);

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
        self.chain.push(block);
        self.chain.last().unwrap()
    }

    pub fn add_transaction<'a>(
        &mut self,
        transaction: &'a mut Transaction<T>,
    ) -> &'a Transaction<T> {
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

    // ---
    // Private methods
    // ---

    fn last_block_nonce(&self) -> u64 {
        match &self.chain.last() {
            Some(block) => block.header.nonce,
            None => 0,
        }
    }
    fn last_block_hash(&self) -> String {
        match &self.chain.last() {
            Some(block) => block.header.merkle_root.clone(),
            None => Chain::genesis_hash(),
        }
    }

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

    // ---
    // Static methods
    // ---

    fn genesis_hash() -> String {
        [0; 64].iter().map(ToString::to_string).collect()
    }

    pub fn new_transfer_transaction(
        sender: &str,
        reciever: &str,
        amount: f64,
    ) -> Transaction<TransferData> {
        let data = TransferData::new(sender, reciever, amount);
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
    //     let mut blockchain = Chain::new(DIFFICULTY_LEVEL, MINER_ADDRESS, REWARD);

    //     let mut transaction_1 = Transaction::new("me", "you", 1.0, TransactionType::Transfer);

    //     blockchain.add_transaction(&mut transaction_1);

    //     assert_eq!(transaction_1.status, TransactionStatus::Unconfirmed);
    //     assert_eq!(blockchain.current_transactions().len(), 1);

    //     let mut transaction_2 = Transaction::new("me", "you", 1.0, TransactionType::Transfer);

    //     blockchain.add_transaction(&mut transaction_2);

    //     assert_eq!(transaction_2.status, TransactionStatus::Unconfirmed);
    //     assert_eq!(blockchain.current_transactions().len(), 2);
    // }
}
