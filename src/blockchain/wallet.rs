use std::io::BufWriter;
use std::str::FromStr;
use std::{fs::OpenOptions, io::BufReader};

use rand::thread_rng;
use secp256k1::hashes::sha256;
use secp256k1::{generate_keypair, Message};
use secp256k1::{
    rand::{rngs, SeedableRng},
    PublicKey, SecretKey,
};
use tiny_keccak::keccak256;
use web3::types::Address;

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Wallet {
    public_key: String,
    private_key: String,
    address: String,
}

impl Wallet {
    pub fn new() -> Self {
        let (private_key, public_key) = Self::generate_keypair();
        let address = Self::public_key_address(&public_key);

        Self {
            public_key: public_key.to_string(),
            private_key: "private_key".to_string(),
            address: format!("{:?}", address),
        }
    }

    pub fn save_to_file(&self, file_path: &str) -> Result<()> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(file_path)?;
        let buf_writer = BufWriter::new(file);
        serde_json::to_writer_pretty(buf_writer, self)?;
        Ok(())
    }

    pub fn get_secret_key(&self) -> Result<SecretKey> {
        let secret_key = SecretKey::from_str(&self.private_key)?;
        Ok(secret_key)
    }

    pub fn get_public_key(&self) -> Result<PublicKey> {
        let pub_key = PublicKey::from_str(&self.public_key)?;
        Ok(pub_key)
    }

    // ---
    // Accessor methods
    // ---

    pub fn private_key_str(&self) -> String {
        self.private_key.clone()
    }

    pub fn public_key_str(&self) -> String {
        self.public_key.clone()
    }

    pub fn address_str(&self) -> String {
        self.address.clone()
    }

    // ---
    // Public functions
    // ---

    pub fn from_file(file_path: &str) -> Result<Wallet> {
        let file = OpenOptions::new().read(true).open(file_path)?;
        let buf_reader = BufReader::new(file);
        let wallet: Wallet = serde_json::from_reader(buf_reader)?;
        Ok(wallet)
    }

    pub fn generate_keypair() -> (SecretKey, PublicKey) {
        generate_keypair(&mut thread_rng())
    }

    pub fn public_key_address(public_key: &PublicKey) -> Address {
        let public_key = public_key.serialize_uncompressed();
        debug_assert_eq!(public_key[0], 0x04);
        let hash = keccak256(&public_key[1..]);
        Address::from_slice(&hash[12..])
    }
}

impl Default for Wallet {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn create_wallet() {
        let wallet = Wallet::new();

        println!("public_key: {:}", wallet.public_key_str());
        println!("private_key: {:}", wallet.private_key_str());
        println!("address: {:}", wallet.address_str());
    }

    #[test]
    fn save_wallet() {
        let wallet = Wallet::new();
        println!("public_key: {:}", wallet.public_key_str());
        println!("private_key: {:}", wallet.private_key_str());
        println!("address: {:}", wallet.address_str());

        wallet.save_to_file("crypto_wallet.json").unwrap();
    }

    #[test]
    fn wallet_from_file() {
        let file_path = "crypto_wallet.json";
        let wallet = Wallet::from_file(file_path).unwrap();

        println!("public_key: {:}", wallet.public_key_str());
        println!("private_key: {:}", wallet.private_key_str());
        println!("address: {:}", wallet.address_str());
    }
}
