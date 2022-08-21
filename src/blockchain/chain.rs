use serde::Serialize;

use super::block::Block;
use super::config::ChainConfig;
use super::hasher::{Hash, Hasher};
use super::models::TransactionData;
use super::transaction::{Transaction, TransactionStatus, TransactionType};
use super::utils::timestamp;

const CHAIN_DIFFICULTY: usize = 2;

#[derive(Clone, Serialize)]
pub struct Chain {
    config: ChainConfig,
    miner_address: String,
    blocks: Vec<Block>,
    current_tx: Vec<Transaction>,
}

impl Chain {
    pub fn new(config: ChainConfig, miner_addr: &str) -> Self {
        let blocks = Chain::get_blocks();
        let mut chain = Chain {
            config,
            blocks,
            miner_address: miner_addr.to_string(),
            current_tx: Vec::new(),
        };

        // TODO: REMOVE FROM CODEBASE IN PRODUCTION
        if chain.blocks.is_empty() {
            chain.genesis_block()
        };
        chain
    }

    // ---
    // Public methods
    // ---

    pub fn mine_new_block(&mut self) -> &Block {
        if self.current_tx.is_empty() {
            return self.blocks.last().unwrap();
        }
        // Get previous block info
        let last_block = self.blocks().last().unwrap();
        let last_nonce = last_block.header.nonce;
        let previous_hash = last_block.header.merkle_root.clone();

        // Run proof of work algorithm
        let nonce = self.proof_of_work(last_nonce);

        // Get new block info
        let index = self.blocks.len();

        // Create empty tx array for new block
        let mut transactions: Vec<Transaction> = Vec::new();

        // Remove all current transactions from blocks, add new new tx vec for new block
        while !self.current_tx.is_empty() {
            transactions.push(self.current_tx.pop().unwrap())
        }

        // Create new reward tx
        let data = TransactionData::TransferData {
            sender: "Root".to_string(),
            receiver: self.miner_address.clone(),
            amount: self.reward(),
        };
        let reward_tx = Chain::new_transaction(data, TransactionType::Reward);

        // Add reward tx to block tx vec
        transactions.push(reward_tx);

        // Change all transaction status to confirmed
        let transactions = Chain::confirm_transactions(transactions);

        // Get new merkle_root root of transactions in block
        let merkle_root = Hasher::merkle_root(&transactions);

        // Make new block
        let block = Block::new(index, nonce, transactions, merkle_root, previous_hash);

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
        // Find tx in current transactions
        for curr_tx in &self.current_tx {
            if curr_tx.hash.to_string() == tx_hash {
                return Some(curr_tx.clone());
            }
        }

        // Find tx in blocks
        for block in self.blocks() {
            for tx in &block.txs {
                if tx.hash.to_string() == tx_hash {
                    return Some(tx.clone());
                }
            }
        }

        // No tx found
        None
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
        self.config.difficulty = difficulty
    }

    pub fn set_reward(&mut self, reward: f64) {
        self.config.reward = reward
    }

    pub fn reward(&self) -> f64 {
        self.config.reward
    }
    pub fn difficulty(&self) -> usize {
        self.config.difficulty
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

        let hashed_guess = Hasher::hash(guess);

        let last_chars =
            &hashed_guess.as_bytes()[hashed_guess.as_bytes().len() - CHAIN_DIFFICULTY..];

        let difficulty_string = [b'0'; CHAIN_DIFFICULTY];

        // for (i, c) in "000".chars().enumerate() {
        //     difficulty_string[i] = c as u8
        // }

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
        let previous_hash = Hash::from_char_slice(&['0'; 64]);

        // Create empty tx array for new block
        let mut transactions: Vec<Transaction> = Vec::new();

        // Create new reward tx
        let data = TransactionData::TransferData {
            sender: "Root".to_string(),
            receiver: self.miner_address.clone(),
            amount: self.reward(),
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
        let block = Block::new(index, nonce, transactions, merkle_root, previous_hash);

        // Append block to blocks
        self.blocks.push(block);
    }

    // ---
    // Static methods
    // ---

    pub fn new_transaction(tx_data: TransactionData, tx_type: TransactionType) -> Transaction {
        let timestamp = timestamp();
        let hash = Hasher::hash_tx_data(&tx_data, timestamp);
        Transaction::new(tx_data, tx_type, timestamp, hash)
    }

    pub fn confirm_transactions(mut transactions: Vec<Transaction>) -> Vec<Transaction> {
        for tx in &mut transactions {
            *tx.status = TransactionStatus::Confirmed
        }
        transactions
    }
}

#[cfg(test)]
mod tests {
    use test_utils::get_config;

    use super::*;
    use test_utils::{new_tx, new_tx_data};

    #[test]
    fn mine_block() {
        let config = get_config();
        let mut chain = Chain::new(config, "test_miner");

        assert_eq!(chain.blocks().len(), 1);
        chain.mine_new_block();
        assert_eq!(chain.blocks().len(), 1);

        for _ in 0..5 {
            chain
                .add_transaction(&mut new_tx(), "sender", "signature")
                .unwrap();
        }

        chain.mine_new_block();

        let new_block = chain.blocks().last().unwrap();

        let mut reward_tx = new_tx();
        let mut reward_count = 0;
        for tx in new_block.txs.iter() {
            if tx.tx_type == TransactionType::Reward {
                reward_count += 1;
                reward_tx = tx.clone();
            }
        }

        assert_eq!(reward_count, 1);
        assert_eq!(reward_tx.status, TransactionStatus::Confirmed);

        // match reward_tx.tx_data {
        //     TransactionData::TransferData { amount, .. } => {
        //         assert_eq!(amount, chain.reward())
        //     }
        //     _ => (),
        // }

        // Ensure amount is same as chain set reward amount
        if let TransactionData::TransferData { amount, .. } = reward_tx.tx_data {
            assert_eq!(amount, chain.reward())
        }
    }

    #[test]
    fn genesis_block() {
        let config = get_config();
        let chain = Chain::new(config, "test_miner");

        for tx in chain.blocks().last().unwrap().txs.iter() {
            assert_eq!(tx.tx_type, TransactionType::GenesisReward);
        }
        assert_eq!(chain.blocks().len(), 1);
        assert_eq!(chain.blocks().last().unwrap().tx_count, 1);
    }

    #[test]
    fn confirm_transactions() {
        let mut txs: Vec<Transaction> = Vec::new();

        for _ in 0..5 {
            txs.push(new_tx());
        }

        let confirmed_txs = Chain::confirm_transactions(txs);

        for tx in confirmed_txs {
            assert_eq!(tx.status, TransactionStatus::Confirmed);
        }
    }

    #[test]
    fn add_transaction() {
        let config = get_config();
        let mut chain = Chain::new(config, "test_miner");
        let tx_data = new_tx_data(12.1);
        let mut tx1 = Chain::new_transaction(tx_data, TransactionType::Transfer);

        chain
            .add_transaction(&mut tx1, "sender", "signature")
            .unwrap();

        assert_eq!(tx1.status, TransactionStatus::Unconfirmed);
        assert_eq!(chain.current_tx().len(), 1);

        let tx_data = new_tx_data(11.1);
        let mut tx2 = Chain::new_transaction(tx_data, TransactionType::Transfer);

        chain
            .add_transaction(&mut tx2, "sender", "signature")
            .unwrap();

        assert_eq!(tx2.status, TransactionStatus::Unconfirmed);
        assert_eq!(chain.current_tx().len(), 2);
    }

    #[test]
    fn get_transaction() {
        let config = get_config();
        let mut chain = Chain::new(config, "test_miner");
        let tx_data = new_tx_data(12.1);
        let mut tx = Chain::new_transaction(tx_data, TransactionType::Transfer);

        chain
            .add_transaction(&mut tx, "sender", "signature")
            .unwrap();

        let tx_from_chain = chain.get_transaction(&tx.hash.to_string()).unwrap();

        assert_eq!(tx.hash.to_string(), tx_from_chain.hash.to_string());

        let not_found = chain.get_transaction("not found");
        match not_found {
            None => (),
            _ => panic!("Should not be found"),
        }
    }
    #[test]
    #[should_panic]
    fn get_transaction_not_found() {
        let config = get_config();
        let chain = Chain::new(config, "test_miner");

        chain.get_transaction("not found").unwrap();
    }

    #[test]
    fn new_chain() {
        let config = get_config();
        let chain = Chain::new(config, "test_miner");

        assert_eq!(chain.miner_address, "test_miner");
        assert_eq!(chain.difficulty(), 0);
        assert_eq!(chain.reward(), 12.1);
    }

    #[test]
    fn set_difficulty() {
        let config = get_config();
        let mut chain = Chain::new(config, "test_miner");

        chain.set_difficulty(1);

        assert_eq!(chain.difficulty(), 1);
    }

    #[test]
    fn get_blocks() {
        let config = get_config();
        let chain = Chain::new(config, "test_miner");

        let blocks = chain.blocks();

        assert_ne!(blocks.len(), 0);
    }

    #[test]
    fn set_reward() {
        let config = get_config();
        let mut chain = Chain::new(config, "test_miner");

        chain.set_reward(24.2);

        assert_eq!(chain.reward() as f64, 24.2);
    }

    mod test_utils {
        use crate::blockchain::{
            config::ChainConfig,
            hasher::Hasher,
            models::TransactionData,
            transaction::{Transaction, TransactionType},
            utils::timestamp,
        };

        pub fn new_tx_data(amount: f64) -> TransactionData {
            TransactionData::TransferData {
                sender: "me".to_string(),
                receiver: "you".to_string(),
                amount,
            }
        }

        pub fn get_config() -> ChainConfig {
            ChainConfig {
                difficulty: 0,
                reward: 12.1,
            }
        }

        pub fn new_tx() -> Transaction {
            let tx_data = TransactionData::TransferData {
                sender: "me".to_string(),
                receiver: "you".to_string(),
                amount: 22.4,
            };
            let timestamp = timestamp();
            let hash = Hasher::hash_tx_data(&tx_data, timestamp);
            Transaction::new(tx_data, TransactionType::Transfer, 1, hash.to_owned())
        }
    }
}
