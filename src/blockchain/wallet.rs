use std::fmt::Debug;
use std::fmt::{self, Display};
use std::io::BufWriter;
use std::{fs::OpenOptions, io::BufReader};

use serde::de::{self, Deserialize, Deserializer, MapAccess, SeqAccess, Visitor};
use serde::Serialize;

use secp256k1::{generate_keypair, rand::thread_rng, PublicKey, Secp256k1, SecretKey};
// use web3::types::Address;

use anyhow::Result;

/// use secp256k1::{SecretKey, Secp256k1, PublicKey};
///
/// let secp = Secp256k1::new();
/// let secret_key = SecretKey::from_slice(&[0xcd; 32]).expect("32 bytes, within curve order");
/// let public_key = PublicKey::from_secret_key(&secp, &secret_key);
///

#[derive(Serialize, Debug)]
pub struct Wallet {
    public_key: PublicKey,
    secret_key: SecretKey,
}

impl Display for Wallet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sec_key_hex = self.secret_key_to_hex();
        let pub_key_hex = self.public_key_to_hex();
        let string = format!(
            "{{\n    public_key: {}\n    secret_key: {}\n}}",
            pub_key_hex, sec_key_hex
        );
        write!(f, "{}", string)
    }
}

impl Wallet {
    pub fn new() -> Self {
        let (secret_key, public_key) = generate_keypair(&mut thread_rng());
        Self {
            public_key,
            secret_key,
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

    pub fn secret_key(&self) -> [u8; 32] {
        self.secret_key.secret_bytes()
    }

    pub fn public_key(&self) -> [u8; 33] {
        self.public_key.serialize()
    }

    pub fn secret_key_to_hex(&self) -> String {
        hex::encode(&self.secret_key.secret_bytes())
    }

    pub fn public_key_to_hex(&self) -> String {
        self.public_key.to_string()
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

    fn secret_key_from_hex(hex_str: &str) -> SecretKey {
        let bytes = hex::decode(hex_str).unwrap();
        SecretKey::from_slice(&bytes).unwrap()
    }

    fn public_key_from_secret_key(secret_key: &SecretKey) -> PublicKey {
        let secp = Secp256k1::new();
        PublicKey::from_secret_key(&secp, secret_key)
    }
}

impl Default for Wallet {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for Wallet {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            SecretKey,
            PublicKey,
        }
        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`secret_key` or `public_key`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "secret_key" => Ok(Field::SecretKey),
                            "public_key" => Ok(Field::PublicKey),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct DurationVisitor;

        impl<'de> Visitor<'de> for DurationVisitor {
            type Value = Wallet;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Duration")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Wallet, V::Error>
            where
                V: SeqAccess<'de>,
            {
                unimplemented!();
            }

            fn visit_map<V>(self, mut map: V) -> Result<Wallet, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut public_key: Option<PublicKey> = None;
                let mut secret_key: Option<SecretKey> = None;

                let saved_map_val = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::SecretKey => {
                            if secret_key.is_some() {
                                return Err(de::Error::duplicate_field("secret_key"));
                            }
                            secret_key = Some(Wallet::secret_key_from_hex(map.next_value()?));
                        }
                        Field::PublicKey => {
                            if public_key.is_some() {
                                return Err(de::Error::duplicate_field("public_key"));
                            }

                            let s_key = map[key] = Some(map.next_value()?);
                        }
                    }
                }
                let public_key =
                    public_key.ok_or_else(|| de::Error::missing_field("public_key"))?;
                let secret_key =
                    secret_key.ok_or_else(|| de::Error::missing_field("secret_key"))?;

                Ok(Wallet {
                    public_key,
                    secret_key,
                })
            }
        }

        const FIELDS: &'static [&'static str] = &["secs", "nanos"];
        deserializer.deserialize_struct("Duration", FIELDS, DurationVisitor)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn create_wallet() {
        let wallet = Wallet::new();

        println!("{}", wallet);
        println!("{:?}", wallet);
    }

    #[test]
    fn wallet_test() {
        let wallet = Wallet::new();

        println!("public_key len: {:?}", wallet.public_key().len());
        println!("secret_key len: {:?}", wallet.secret_key().len());

        println!("public_key hex len {:?}", wallet.public_key_to_hex().len());
        println!("secret_key hex len {:?}", wallet.secret_key_to_hex().len());
        // println!("{:?}", wallet);
    }

    // #[test]
    // fn save_wallet() {
    //     let wallet = Wallet::new();
    //     println!("public_key: {:}", wallet.public_key_str());
    //     println!("private_key: {:}", wallet.private_key_str());
    //     println!("address: {:}", wallet.address_str());

    //     wallet.save_to_file("crypto_wallet.json").unwrap();
    // }

    // #[test]
    // fn wallet_from_file() {
    //     let file_path = "crypto_wallet.json";
    //     let wallet = Wallet::from_file(file_path).unwrap();

    //     println!("public_key: {:}", wallet.public_key_str());
    //     println!("private_key: {:}", wallet.private_key_str());
    //     println!("address: {:}", wallet.address_str());
    // }
}
