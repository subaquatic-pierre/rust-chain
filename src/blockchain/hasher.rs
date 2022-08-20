use std::fmt;

use hex_fmt::HexFmt;
use serde::{Serialize, Serializer};
use sha2::{Digest, Sha256};

use super::{models::TransactionData, transaction::Transaction};

#[derive(Clone, Debug)]
pub struct Hash {
    bytes: [u8; 64],
}

impl Hash {
    pub fn new() -> Self {
        Self { bytes: [0u8; 64] }
    }

    pub fn write(&mut self, string: &str) {
        for (i, c) in string.chars().enumerate() {
            self.bytes[i] = c as u8
        }
    }
}

impl Serialize for Hash {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl Default for Hash {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.bytes))
    }
}

pub struct Hasher {}

impl Hasher {
    pub fn hash(data: impl Serialize, buf: &mut Hash) -> &mut Hash {
        let bytes = bincode::serialize(&data).unwrap();
        // create a Sha256 object
        let mut hasher = Sha256::new();

        // write input message
        hasher.update(bytes);

        // read hash digest and consume hasher
        let result = hasher.finalize();

        let str_hash = format!("{}", HexFmt(result));

        buf.write(&str_hash);
        buf
    }

    pub fn hash_tx_data<'a>(
        tx_data: &TransactionData,
        timestamp: u64,
        buf: &'a mut Hash,
    ) -> &'a mut Hash {
        let data = format!("timestamp:{timestamp}|{}", tx_data);
        let bytes = bincode::serialize(&data).unwrap();
        // create a Sha256 object
        let mut hasher = Sha256::new();

        // write input message
        hasher.update(bytes);

        // read hash digest and consume hasher
        let result = hasher.finalize();

        let str_hash = format!("{}", HexFmt(result));

        buf.write(&str_hash);
        buf
    }

    pub fn merkle_root(txs: &[Transaction]) -> String {
        match txs.len() {
            1 => Hasher::hash_two_txs(&txs[0], &txs[0]),
            2 => Hasher::hash_two_txs(&txs[0], &txs[1]),
            _ => Hasher::merkle_root(&txs[..txs.len() - 2]),
        }
    }

    // ---
    // Private methods
    // ---
    fn hash_two_txs(tx1: &Transaction, tx2: &Transaction) -> String {
        let data = format!("{}{}", tx1.hash, tx2.hash);

        let bytes = bincode::serialize(&data).unwrap();
        // create a Sha256 object
        let mut hasher = Sha256::new();

        // write input message
        hasher.update(bytes);

        // read hash digest and consume hasher
        let result = hasher.finalize();

        format!("{}", HexFmt(result))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use test_utils::new_tx;

    #[derive(Serialize)]
    struct Data {
        x: i16,
        y: i16,
    }

    #[test]
    fn hash_serializable() {
        let d1 = Data { x: 1, y: 2 };
        // // let d2 = Data { x: 1, y: 2 };
        // let d3 = Data { x: 42, y: 24 };

        let mut buf = Hash::new();

        Hasher::hash(d1, &mut buf);

        println!("{buf}");

        // let d2 = Hasher::hash_serializable(d2);
        // let d3 = Hasher::hash_serializable(d3);

        // assert_ne!(d1, d3);
        // assert_eq!(d1, d2);
    }

    #[test]
    fn merkle_root() {
        let mut tx_vec_1: Vec<Transaction> = Vec::new();
        let mut tx_vec_2: Vec<Transaction> = Vec::new();

        let tx = new_tx(10.1, 1);
        let tx2 = new_tx(10.12, 2);

        for _ in 0..5 {
            tx_vec_1.push(tx.clone());
            tx_vec_2.push(tx2.clone());
        }

        let merkle_1 = Hasher::merkle_root(&tx_vec_1);
        let merkle_2 = Hasher::merkle_root(&tx_vec_2);

        assert_ne!(merkle_1, merkle_2);

        let merkle_3 = Hasher::merkle_root(&[tx.clone(), tx.clone()]);
        let merkle_4 = Hasher::merkle_root(&[tx.clone()]);

        assert_eq!(merkle_3, merkle_4);

        let merkle_5 = Hasher::merkle_root(&[tx]);
        let merkle_6 = Hasher::merkle_root(&[tx2]);

        assert_ne!(merkle_5, merkle_6);
    }

    mod test_utils {
        use super::*;
        use crate::blockchain::{
            hasher::{Hash, Hasher},
            models::TransactionData,
            transaction::TransactionType,
        };

        pub fn new_tx(amount: f64, timestamp: u64) -> Transaction {
            let tx_data = TransactionData::TransferData {
                sender: "me".to_string(),
                receiver: "you".to_string(),
                amount,
            };
            let mut hash_buf = Hash::new();
            let hash = Hasher::hash_tx_data(&tx_data, timestamp, &mut hash_buf);
            Transaction::new(
                tx_data,
                TransactionType::Transfer,
                timestamp,
                hash.to_owned(),
            )
        }
    }
}
