extern crate clap;
extern crate ethkey;
extern crate hex;
extern crate rand;

use clap::App;
use ethkey::{KeyPair, Secret};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use crate::file::file_operation;

mod file;

fn main() {
    let matches = App::new("eth_address")
        .version("0.1.0")
        .args_from_usage(
            "
      -v, --verbose          'Run the tool in verbose mode'
      <WORD>                 'Enter the hex value you want to have in your vanity address'
      ",
        )
        .get_matches();
    let desired_word = matches.value_of("WORD").unwrap();
    println!("Desired word in vanity address: {}", desired_word);

    loop {
        let random32char: String = thread_rng().sample_iter(&Alphanumeric).take(32).collect();
        let secret = Secret::from_slice(random32char.as_bytes());
        let secret_literal = hex::encode(&random32char);
        let kp = KeyPair::from_secret(secret.unwrap());
        let address = kp.unwrap().address();

        let complete_vanity = String::from("0x") + desired_word;
        let address_literal = address.to_string();

        if address_literal.contains(&complete_vanity) {
            println!("Your new private key: {}", secret_literal);
            println!("Your new ethereum vanity address: {:? }", address);
            file_operation(secret_literal, address);
        }
    }
}
