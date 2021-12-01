extern crate ethkey;
extern crate hex;
extern crate rand;

use ethkey::Random;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use crate::file::file_operation;

mod file;

fn main() {
    let prefix = "00000000";
    loop {
        let keypair = try!(Random.generate());

        if keypair.address().starts_with(&prefix) {
            println!("Your new ethereum vanity address: {:? }", keypair.address());
            println!("Your new private key: {}", keypair.secret());
            file_operation(keypair.address(), keypair.secret());
        }
    }
}
