extern crate rand;
#[macro_use]
extern crate lazy_static;
extern crate rustc_serialize;
extern crate secp256k1;
extern crate tiny_keccak;

mod brain;
mod error;
mod keccak;
mod keypair;
mod prefix;
mod primitive;
mod random;

lazy_static! {
    static ref SECP256K1: secp256k1::Secp256k1 = secp256k1::Secp256k1::new();
}

/// Generates new keypair.
pub trait Generator {
    /// Should be called to generate new keypair.
    fn generate(self) -> Result<KeyPair, Error>;
}

pub use self::brain::Brain;
pub use self::error::Error;
pub use self::keypair::{public_to_address, KeyPair};
pub use self::prefix::Prefix;
pub use self::primitive::{Address, Message, Public, Secret};
pub use self::random::Random;
