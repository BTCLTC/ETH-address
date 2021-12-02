extern crate ethkey;
extern crate hex;
extern crate rand;
extern crate rustc_serialize;

use ethkey::{Generator, Random};

use crate::file::file_operation;

use rustc_serialize::hex::FromHex;

mod file;

fn main() {
    let prefix = "00000000";
    let hex = &prefix.from_hex().unwrap();

    loop {
        let keypair = Random.generate().unwrap();

        if keypair.address().starts_with(hex) {
            println!("Your new ethereum vanity address: {:? }", keypair.address());
            println!("Your new private key: {}", keypair.secret());
            file_operation(keypair.address().to_string(), keypair.secret().to_string());
        }
    }
}
