mod eth;
mod file;

use async_std::task;
use clap::Parser;
use eth::KeyPair;

/// Args of prefix
#[derive(Parser, Debug, Clone)]
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
    let prefix = Args::parse().prefix;
    let number = num_cpus::get();

    for _i in 0..number {
        let prefix_address = prefix.clone();
        task::spawn( async move {
            get_address(&prefix_address).await;
        });
    }

    loop {

    }
}

async fn get_address(prefix: &str) {
    loop {
        let keypair = KeyPair::generate();

        if keypair.is_match(prefix) {
            let directory = Args::parse().directory;
            keypair.print_info(&directory);
        }
    }
}
