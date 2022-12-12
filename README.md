# eth_address

Eth custom address prefix

```bash
Usage: eth_address [OPTIONS]

Options:
  -p, --prefix <PREFIX>        Address prefix [default: 00000000]
  -d, --directory <DIRECTORY>  Save file directory [default: ./]
  -h, --help                   Print help information
  -V, --version
```

## Installation

```bash
cargo build --release
```

## Use

```bash
./target/release/eth_address -p 000000 -d ../../ETH
```
