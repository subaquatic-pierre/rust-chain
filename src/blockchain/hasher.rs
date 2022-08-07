use hex_fmt::HexFmt;
use serde::Serialize;
use sha2::{Digest, Sha256};

use super::transaction::Transaction;

pub struct Hasher {}

impl Hasher {
    pub fn hash_serializable(data: impl Serialize) -> String {
        let bytes = bincode::serialize(&data).unwrap();
        // create a Sha256 object
        let mut hasher = Sha256::new();

        // write input message
        hasher.update(bytes);

        // read hash digest and consume hasher
        let result = hasher.finalize();

        format!("{}", HexFmt(result))
    }

    pub fn merkle_root(transactions: &[Transaction]) -> String {
        match transactions.len() {
            1 => Hasher::hash_serializable(format!(
                "{}{}",
                transactions[0].hash, transactions[0].hash
            )),
            2 => Hasher::hash_serializable(format!(
                "{}{}",
                transactions[0].hash, transactions[1].hash
            )),
            _ => Hasher::merkle_root(&transactions[..transactions.len() - 2]),
        }
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
        let d2 = Data { x: 1, y: 2 };
        let d3 = Data { x: 42, y: 24 };

        let d1 = Hasher::hash_serializable(d1);
        let d2 = Hasher::hash_serializable(d2);
        let d3 = Hasher::hash_serializable(d3);

        assert_ne!(d1, d3);
        assert_eq!(d1, d2);
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

        let merkle_5 = Hasher::merkle_root(&[tx.clone()]);
        let merkle_6 = Hasher::merkle_root(&[tx2.clone()]);

        assert_ne!(merkle_5, merkle_6);
    }

    mod test_utils {
        use super::*;
        use crate::blockchain::{models::TransactionData, transaction::TransactionType};

        pub fn new_tx(amount: f64, timestamp: u64) -> Transaction {
            let tx_data = TransactionData::TransferData {
                sender: "me".to_string(),
                receiver: "you".to_string(),
                amount,
            };
            let tx = Transaction::new(tx_data, TransactionType::Transfer, timestamp);
            tx
        }
    }
}
