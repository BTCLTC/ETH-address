mod eth;
mod file;

use async_std::task;
use clap::Parser;
use eth::KeyPair;

/// Args of prefix
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Address prefix
    #[arg(short, long, default_value_t = String::from("00000000"))]
    prefix: String,

    /// Save file directory
    #[arg(short, long, default_value_t = String::from("./"))]
    directory: String
}

fn main() {
    let number = num_cpus::get();

    for _i in 0..number {
        task::spawn( async {
            get_address().await;
        });
    }

    loop {

    }
}

async fn get_address() {
    loop {
        let prefix = Args::parse().prefix;
        let keypair = KeyPair::generate();

        if keypair.is_match(&prefix) {
            let directory = Args::parse().directory;
            keypair.print_info(&directory);
        }
    }
}
