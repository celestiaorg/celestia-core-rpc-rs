[![Crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]

# celestia-core-rpc-rs

A Rust implementation of the core types returned by a Celestia Core node's RPC
endpoint. These can be used to deserialize JSON-RPC responses.

All networking related features will be feature guarded to keep the
dependencies small in cases where only the core types are needed.

[//]: # (badges)

[crate-image]: https://img.shields.io/crates/v/tendermint-rpc.svg
[crate-link]: https://crates.io/crates/tendermint-rpc
[docs-image]: https://docs.rs/tendermint-rpc/badge.svg
[docs-link]: https://github.com/celestiaorg/celestia-core/tree/main/docs

[//]: # (general links)

