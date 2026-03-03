[![Crate][crate-image]][crate-link]

# celestia-core-rpc-rs

A Rust implementation of the core types returned by a Celestia Core node's RPC
endpoint. These can be used to deserialize JSON-RPC responses.

All networking related features will be feature guarded to keep the
dependencies small in cases where only the core types are needed.

## Features

- `http-client`: JSON-RPC over HTTP(S) client.
- `websocket-client`: JSON-RPC over WebSocket client with subscriptions.
- `cli`: enables the bundled CLI (requires HTTP + WebSocket features).

## Compatibility

- Supports RPC dialects v0.34, v0.37, and v0.38 (default: latest).
- Set `CompatMode` on clients when you need to target a specific dialect.

## Celestia-specific decoding

- Block data uses `celestia-types` and custom JSON decoding for Celestia fields
  (e.g. `square_size`, `hash`).

## Using as a library

Add to `Cargo.toml`:

```toml
[dependencies]
celestia-core-rpc = { version = "0.39.25", features = ["http-client"] }
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

Example:

```rust
use celestia_core_rpc::{Client, HttpClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = HttpClient::new("http://127.0.0.1:26657")?;
    let status = client.status().await?;
    println!("latest height: {}", status.sync_info.latest_block_height);
    Ok(())
}
```

## Testing

- Unit tests: `cargo test`.
- Remote smoke tests (optional): set `CELESTIA_RPC_URL` and run
  `cargo test --all-features`.
- Enable verbose remote test logging with `CELESTIA_RPC_LOG=1` and
  `cargo test --all-features --test remote -- --nocapture`.
- Optional dialect override for remote tests: `CELESTIA_RPC_COMPAT=v0.37`.
- Optional height override for remote tests: `CELESTIA_RPC_HEIGHT=123456`.
- Optional transaction checks:
  - `CELESTIA_RPC_TX_HASH` to exercise `/tx_status`.
  - `CELESTIA_RPC_TX_BASE64` to exercise `/check_tx`.
  - `CELESTIA_RPC_TX_HASHES` (comma-separated) to exercise `/tx_status_batch`.
- `CELESTIA_RPC_LOG=1` to emit request logs in `tests/remote.rs`.

Example:

```bash
CELESTIA_RPC_URL="http://127.0.0.1:26657" cargo test --all-features
```

Optional extras:

```bash
CELESTIA_RPC_URL="http://127.0.0.1:26657" \
CELESTIA_RPC_COMPAT=v0.38 \
CELESTIA_RPC_HEIGHT=1000 \
CELESTIA_RPC_TX_HASH="<HEX_HASH>" \
CELESTIA_RPC_TX_BASE64="<BASE64_TX>" \
cargo test --all-features --test remote
```

## CLI

- `celestiacore`: command-style CLI for common RPC endpoints. It can print the
  JSON payload, emit curl commands, and compare results against live curl calls.

## Install the CLI

Build locally:

```bash
cargo build --release --features cli --bin celestiacore
./target/release/celestiacore --help
```

Install to your PATH:

```bash
cargo install --path . --features cli --bin celestiacore
```

Install from crates.io:

```bash
cargo install celestia-core-rpc --features cli --bin celestiacore
```

Install via curl (prebuilt release assets):

```bash
curl -sSfL https://raw.githubusercontent.com/celestiaorg/celestia-core-rpc-rs/main/scripts/install.sh | bash
```

You can override defaults:

```bash
REPO=celestiaorg/celestia-core-rpc-rs \
BIN=celestiacore \
VERSION=v0.39.25 \
PREFIX=$HOME/.local \
curl -sSfL https://raw.githubusercontent.com/celestiaorg/celestia-core-rpc-rs/main/scripts/install.sh | bash
```

Example:

```bash
cargo run --features cli --bin celestiacore -- \
  --url "http://127.0.0.1:26657" \
  status
```

Compare with curl:

```bash
cargo run --features cli --bin celestiacore -- \
  --url "http://127.0.0.1:26657" \
  --run-curl \
  block --height 1000
```

Simple request examples:

```bash
cargo run --features cli --bin celestiacore -- \
  --url "http://127.0.0.1:26657" \
  status

cargo run --features cli --bin celestiacore -- \
  --url "http://127.0.0.1:26657" \
  tx "<HEX_HASH>"
```

Global flags like `--url` can be placed before or after the subcommand:

```bash
cargo run --features cli --bin celestiacore -- \
  tx "<HEX_HASH>" --url "http://127.0.0.1:26657"
```

By default, byte arrays (e.g. `tx`, `tx_result.data`) are rendered as hex
strings. If you need raw byte arrays, add `--bytes-array`.

Include request payloads and curl commands:

```bash
cargo run --features cli --bin celestiacore -- \
  --url "http://127.0.0.1:26657" \
  --show-request --show-curl \
  block --height 1000
```

If your RPC endpoint is load-balanced, node-specific fields (e.g. `node_info`,
`validator_info`) may differ between the client call and curl. You can compare
only a stable subset using a JSON pointer:

```bash
cargo run --features cli --bin celestiacore -- \
  --url "http://127.0.0.1:26657" \
  --run-curl \
  --compare-path /sync_info \
  status
```

[//]: # (badges)

[crate-image]: https://img.shields.io/crates/v/celestia-core-rpc.svg
[crate-link]: https://crates.io/crates/celestia-core-rpc

[//]: # (general links)
