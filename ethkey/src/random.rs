use super::{Error, Generator, KeyPair, SECP256K1};
use rand::os::OsRng;

/// Randomly generates new keypair.
pub struct Random;

impl Generator for Random {
    fn generate(self) -> Result<KeyPair, Error> {
        let context = &SECP256K1;
        let mut rng = OsRng::new()?;
        let (sec, publ) = context.generate_keypair(&mut rng)?;

        Ok(KeyPair::from_keypair(sec, publ))
    }
}
