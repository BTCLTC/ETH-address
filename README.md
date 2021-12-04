# eth_address

以太坊靓号生成器

## Edit

src/main.rs

```bash
let prefix = "00000000"; // prefix

let prefix_addr = &keypair.address().to_string()[..8]; // edit 8: prefix length
```

## Installation

```bash
cargo build --release
```
