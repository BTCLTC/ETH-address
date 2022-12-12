use rand::Rng;
use libsecp256k1::{PublicKey, SecretKey};
use sha3::{Digest, Keccak256};

use crate::file::write_file;

pub struct KeyPair {
  pub address: String,
  private_key: SecretKey
}

impl KeyPair {
  pub fn generate() -> KeyPair {
    let random_bytes = rand::thread_rng().gen::<[u8; 32]>();

    let private_key = SecretKey::parse(&random_bytes).unwrap();
    let public_key = PublicKey::from_secret_key(&private_key);

    let public_key_hash = Keccak256::digest(&public_key.serialize()[1..]);
    let address = &public_key_hash[12..];

    KeyPair {
      address: hex::encode(address),
      private_key
    }
  }

  pub fn is_match(&self, prefix: &str) -> bool {
    &self.address[..prefix.len()] == prefix
  }

  pub fn print_info(&self, directory: &str) {
    let address = format!("0x{}", self.address);
    let private_key = hex::encode(self.private_key.serialize());
    println!("Your new ethereum vanity address: {}", address);
    println!("Your new private key: {}", private_key);
    write_file(&address, &private_key, directory);
  }
}