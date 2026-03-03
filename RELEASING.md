# Releasing celestia-core-rpc

This repository ships a Rust crate (`celestia-core-rpc`) and an optional CLI
binary (`celestiacore`).

## 1) Prepare the release

1. Ensure the working tree is clean and tests pass:

   ```bash
   cargo test --all-features
   ```

2. Update the version in `Cargo.toml`.

3. Optional: update release notes/CHANGELOG if you maintain one.

## 2) Publish the crate to crates.io

1. Login (one-time):

   ```bash
   cargo login
   ```

2. Dry run:

   ```bash
   cargo publish --dry-run
   ```

3. Publish:

   ```bash
   cargo publish
   ```

Consumers can then install the CLI from crates.io:

```bash
cargo install celestia-core-rpc --features cli --bin celestiacore
```

## 3) Build release binaries

Build the binary for the targets you want to support (examples):

```bash
cargo build --release --features cli --bin celestiacore
```

Package artifacts with a consistent naming scheme:

```bash
VERSION=v0.39.25
TARGET=x86_64-unknown-linux-gnu
tar -czf "celestiacore-${VERSION}-${TARGET}.tar.gz" -C target/release celestiacore
```

Repeat for other targets (e.g. `aarch64-apple-darwin`).

## 4) Create a GitHub release

1. Tag the release: `vX.Y.Z`.
2. Create a GitHub release with the tag.
3. Upload the `.tar.gz` artifacts.

The `scripts/install.sh` curl installer expects assets named:

```
<bin>-<tag>-<target>.tar.gz
```

Example:

```
celestiacore-v0.39.25-x86_64-unknown-linux-gnu.tar.gz
```

## 5) Update the install instructions

Once the GitHub release is live, the curl installer will work:

```bash
curl -sSfL https://raw.githubusercontent.com/celestiaorg/celestia-core-rpc-rs/main/scripts/install.sh | bash
```

You can override defaults via env vars:

```bash
REPO=celestiaorg/celestia-core-rpc-rs \
BIN=celestiacore \
VERSION=v0.39.25 \
PREFIX=$HOME/.local \
curl -sSfL https://raw.githubusercontent.com/celestiaorg/celestia-core-rpc-rs/main/scripts/install.sh | bash
```
