extern crate ethkey;
extern crate hex;
extern crate num_cpus;
extern crate rand;

use async_std::task;
use ethkey::{Generator, Random};

use crate::file::file_operation;

mod file;

fn main() {
    let prefix = "00000000";
    let number = num_cpus::get() / 2;

    for _i in 0..number {
        task::spawn( async move {
            get_address(prefix).await;
        });
    }

    loop {

    }
}

async fn get_address(prefix: &str) {
    loop {
        let keypair = Random.generate().unwrap();

        let prefix_addr = &keypair.address().to_string()[..8];
        if prefix_addr == prefix {
            println!("Your new ethereum vanity address: {:? }", keypair.address());
            println!("Your new private key: {}", keypair.secret());
            file_operation(keypair.address().to_string(), keypair.secret().to_string());
        }
    }
}
