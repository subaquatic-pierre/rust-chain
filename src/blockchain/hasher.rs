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

    pub fn merkle_root<T>(transactions: &[Transaction<T>]) -> String
    where
        T: Serialize,
    {
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
