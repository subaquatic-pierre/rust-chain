use std::fmt;
use std::str;

use hex;
use hex_fmt::HexFmt;
use serde::{Serialize, Serializer};
use sha2::{Digest, Sha256};

// use super::utils::{as_base64, from_base64};
use super::{models::TransactionData, transaction::Transaction};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Hash {
    // HASH::SHA2_256
    bytes: Vec<u8>,
}

impl Hash {
    pub fn new() -> Self {
        Self { bytes: Vec::new() }
    }

    pub fn as_bytes(&self) -> &Vec<u8> {
        &self.bytes
    }

    pub fn from_char_slice(slice: &[char]) -> Self {
        let mut buf = Vec::new();
        for &hex_char in slice.iter() {
            buf.push(hex_char as u8)
        }
        let bytes = hex::decode(buf).unwrap();

        Self { bytes }
    }

    pub fn from_hex_str(hex: &str) -> Self {
        // let mut decoded = [0u8; 32];
        let bytes = hex::decode(hex).unwrap();

        Self { bytes }
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
        write!(f, "{}", hex::encode(self.as_bytes()))
    }
}

pub struct Hasher {}

impl Hasher {
    pub fn hash(data: impl Serialize) -> Hash {
        let bytes = bincode::serialize(&data).unwrap();

        // create a Sha256 object
        let mut hasher = Sha256::new();

        // write input message
        hasher.update(bytes);

        // read hash digest and consume hasher
        let result = hasher.finalize();

        Hash::from_hex_str(&format!("{}", HexFmt(result)))
    }

    pub fn hash_tx_data(tx_data: &TransactionData, timestamp: u64) -> Hash {
        let data = format!("timestamp:{timestamp}|{}", tx_data);

        Self::hash(data)
    }

    pub fn merkle_root(txs: &[Transaction]) -> Hash {
        match txs.len() {
            1 => Hasher::hash_two_txs(&txs[0], &txs[0]),
            2 => Hasher::hash_two_txs(&txs[0], &txs[1]),
            _ => Hasher::merkle_root(&txs[..txs.len() - 2]),
        }
    }

    // ---
    // Private methods
    // ---
    fn hash_two_txs(tx1: &Transaction, tx2: &Transaction) -> Hash {
        let data = format!("{}{}", tx1.hash, tx2.hash);

        Self::hash(data)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use test_utils::new_tx;

    #[derive(Serialize, PartialEq, PartialOrd)]
    struct Data {
        x: i16,
        y: i16,
    }

    #[test]
    fn hash_from_slice() {
        let hash_slice = Hash::from_char_slice(&['0'; 64]);
        assert_eq!(
            format!("{hash_slice}"),
            "0000000000000000000000000000000000000000000000000000000000000000"
        );
    }

    #[test]
    fn hash_from_hex_str() {
        let str = "7b11c1133330cd161071bf23a0c9b6ce5320a8f3a0f83620035a72be46df4104";
        let hash = Hash::from_hex_str(str);

        println!("{:?}", hash.as_bytes())
    }

    #[test]
    fn hash_serializable() {
        let d1 = Data { x: 1, y: 2 };

        let hash = Hasher::hash(d1);

        assert!(!hash.as_bytes().is_empty());
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
            hasher::Hasher, models::TransactionData, transaction::TransactionType,
        };

        pub fn new_tx(amount: f64, timestamp: u64) -> Transaction {
            let tx_data = TransactionData::TransferData {
                sender: "me".to_string(),
                receiver: "you".to_string(),
                amount,
            };
            let hash = Hasher::hash_tx_data(&tx_data, timestamp);
            Transaction::new(tx_data, TransactionType::Transfer, timestamp, hash)
        }
    }
}
