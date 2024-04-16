# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.32.1](https://github.com/eigerco/celestia-tendermint-rs/compare/celestia-tendermint-proto-v0.32.0...celestia-tendermint-proto-v0.32.1) - 2024-01-15

### Other
- fully specify homepage url in metadata ([#27](https://github.com/eigerco/celestia-tendermint-rs/pull/27))
- align authors, repository and homepage keys ([#25](https://github.com/eigerco/celestia-tendermint-rs/pull/25))

## [0.32.0](https://github.com/eigerco/celestia-tendermint-rs/releases/tag/celestia-tendermint-proto-v0.32.0) - 2024-01-12

This is the first release of the celestia-tendermint-rs, fork of tendermint-rs.

### Added
- *(proto)* allow serialization of Event ([#9](https://github.com/eigerco/celestia-tendermint-rs/pull/9))
- use protobuffers from celestia-core ([#1](https://github.com/eigerco/celestia-tendermint-rs/pull/1))

### Fixed
- *(proto)* wrap values in options in bytes serializers ([#12](https://github.com/eigerco/celestia-tendermint-rs/pull/12))
- *(celestia)* commit height serialization as number

### Other
- pre-release cleanups ([#16](https://github.com/eigerco/celestia-tendermint-rs/pull/16))
- update prost to 0.12.0
- remove unneded clones in proto encoding
- Remove unneeded allocations in serializers ([#6](https://github.com/eigerco/celestia-tendermint-rs/pull/6))

### Other - inherited
- Prepare release for v0.32.0 ([#1314](https://github.com/informalsystems/tendermint-rs/pull/1314))
- Attack detector and evidence reporting ([#1292](https://github.com/informalsystems/tendermint-rs/pull/1292))
- Prepare release v0.31.1 ([#1298](https://github.com/informalsystems/tendermint-rs/pull/1298))
- Prepare v0.31.0 release ([#1295](https://github.com/informalsystems/tendermint-rs/pull/1295))
- Parse and fetch proto dependencies from `buf.lock` in the repository source ([#1293](https://github.com/informalsystems/tendermint-rs/pull/1293))
- Release 0.30.0 ([#1282](https://github.com/informalsystems/tendermint-rs/pull/1282))
- Fix typos ([#1266](https://github.com/informalsystems/tendermint-rs/pull/1266))
- Side-by-side support for Tendermint 0.34 and 0.37 ([#1193](https://github.com/informalsystems/tendermint-rs/pull/1193))
- Prepare `0.29.1` release of `tendermint` ([#1273](https://github.com/informalsystems/tendermint-rs/pull/1273))
- `v0.29.0` ([#1267](https://github.com/informalsystems/tendermint-rs/pull/1267))
- Update all crates to Rust edition 2021 and fix clippy warnings introduced in Rust 1.67 ([#1261](https://github.com/informalsystems/tendermint-rs/pull/1261))
- Fix lints that trigger clippy 0.1.66 ([#1253](https://github.com/informalsystems/tendermint-rs/pull/1253))
- Merge pull request from GHSA-xqqc-c5gw-c5r5
- remove the syn version pin ([#1242](https://github.com/informalsystems/tendermint-rs/pull/1242))
- Release v0.27.0 ([#1240](https://github.com/informalsystems/tendermint-rs/pull/1240))
- Fix clippy lints for Rust 1.65 ([#1223](https://github.com/informalsystems/tendermint-rs/pull/1223))
- Replace RPC ABCI types with ABCI domain types ([#1204](https://github.com/informalsystems/tendermint-rs/pull/1204))
- Release v0.26.0 ([#1218](https://github.com/informalsystems/tendermint-rs/pull/1218))
- Apply suggestions from [#1212](https://github.com/informalsystems/tendermint-rs/pull/1212) code review ([#1215](https://github.com/informalsystems/tendermint-rs/pull/1215))
- Fix unclosed HTML tags errors ([#1212](https://github.com/informalsystems/tendermint-rs/pull/1212))
- Serialization of optional values without Clone ([#1207](https://github.com/informalsystems/tendermint-rs/pull/1207))
- Rebase ABCI domain types onto main ([#1203](https://github.com/informalsystems/tendermint-rs/pull/1203))
- Release v0.25.0 ([#1202](https://github.com/informalsystems/tendermint-rs/pull/1202))
- Unpin time dependency ([#1199](https://github.com/informalsystems/tendermint-rs/pull/1199))
- Add support for Tendermint Core v0.34.21 ([#1198](https://github.com/informalsystems/tendermint-rs/pull/1198))
- Fix deserialization of `DuplicateVoteEvidence` in `/block_results` response ([#1195](https://github.com/informalsystems/tendermint-rs/pull/1195))
- Do not generate well-known protobuf types ([#1189](https://github.com/informalsystems/tendermint-rs/pull/1189))
- Update `master` references to `main` ([#1190](https://github.com/informalsystems/tendermint-rs/pull/1190))
- Initialize the `main` branch ([#1172](https://github.com/informalsystems/tendermint-rs/pull/1172))
- Release v0.23.8 ([#1162](https://github.com/informalsystems/tendermint-rs/pull/1162))
- Update to Tendermint v0.34.20 ([#1160](https://github.com/informalsystems/tendermint-rs/pull/1160))
- Fix JSON deserialization of abci::ResponseInfo (backport to v0.23.x) ([#1156](https://github.com/informalsystems/tendermint-rs/pull/1156))
- Release v0.23.8-pre.1 ([#1151](https://github.com/informalsystems/tendermint-rs/pull/1151))
- Support for v0.34.20 prioritized mempool ([#1149](https://github.com/informalsystems/tendermint-rs/pull/1149))
- Release v0.23.7 ([#1122](https://github.com/informalsystems/tendermint-rs/pull/1122))
- [v0.23.x] Update `prost` to v0.10 ([#1114](https://github.com/informalsystems/tendermint-rs/pull/1114))
- Release v0.23.6 ([#1111](https://github.com/informalsystems/tendermint-rs/pull/1111))
- Release v0.23.5 ([#1079](https://github.com/informalsystems/tendermint-rs/pull/1079))
- Split out verifier parts of tendermint-light-client to tendermint-light-client-verifier (Backported to v0.23) ([#1072](https://github.com/informalsystems/tendermint-rs/pull/1072))
- Release v0.23.4 ([#1073](https://github.com/informalsystems/tendermint-rs/pull/1073))
- Release v0.23.3 ([#1066](https://github.com/informalsystems/tendermint-rs/pull/1066))
- Add temporary fix and tests for `block_results` serialization ([#1061](https://github.com/informalsystems/tendermint-rs/pull/1061))
- Release v0.23.2 ([#1043](https://github.com/informalsystems/tendermint-rs/pull/1043))
- Replace chrono with time 0.3 (backport to 0.23.x) ([#1036](https://github.com/informalsystems/tendermint-rs/pull/1036))
- Release v0.23.1 ([#1019](https://github.com/informalsystems/tendermint-rs/pull/1019))
- Fix proto compiler ([#1015](https://github.com/informalsystems/tendermint-rs/pull/1015))
- Release v0.23.0 ([#1013](https://github.com/informalsystems/tendermint-rs/pull/1013))
- Use `core` and `alloc` crates for `no_std` compatibility (Take 2) ([#993](https://github.com/informalsystems/tendermint-rs/pull/993))
- Update to official Prost v0.9 ([#1011](https://github.com/informalsystems/tendermint-rs/pull/1011))
- Bump version to 0.23.0-internal ([#1009](https://github.com/informalsystems/tendermint-rs/pull/1009))
- Move out `tendermint::config` to `tendermint-config` crate ([#986](https://github.com/informalsystems/tendermint-rs/pull/986))
- Adopt forked prost crates ([#1005](https://github.com/informalsystems/tendermint-rs/pull/1005))
- Use chrono::DateTime instead of std::time::SystemTime ([#994](https://github.com/informalsystems/tendermint-rs/pull/994))
- Release v0.22.0 ([#987](https://github.com/informalsystems/tendermint-rs/pull/987))
- Use flex-error for tendermint-rs errors ([#923](https://github.com/informalsystems/tendermint-rs/pull/923))
- Release v0.21.0 ([#935](https://github.com/informalsystems/tendermint-rs/pull/935))
- Temporarily revert [#926](https://github.com/informalsystems/tendermint-rs/pull/926) ([#928](https://github.com/informalsystems/tendermint-rs/pull/928))
- Update `prost` and `prost-types` to version 0.8 ([#926](https://github.com/informalsystems/tendermint-rs/pull/926))
- Rebuild Protobuf data structures for Tendermint v0.34.9 ([#920](https://github.com/informalsystems/tendermint-rs/pull/920))
- Release v0.20.0 ([#912](https://github.com/informalsystems/tendermint-rs/pull/912))
- Fix latest clippy assertion failures ([#910](https://github.com/informalsystems/tendermint-rs/pull/910))
- Clippy fixes ([#868](https://github.com/informalsystems/tendermint-rs/pull/868))
- Release v0.19.0 ([#854](https://github.com/informalsystems/tendermint-rs/pull/854))
- Fix Light Client validator set hash calculation ([#834](https://github.com/informalsystems/tendermint-rs/pull/834))
- Release v0.18.1 ([#808](https://github.com/informalsystems/tendermint-rs/pull/808))
- Fix rendering of documentation on docs.rs ([#807](https://github.com/informalsystems/tendermint-rs/pull/807))
- Fix panic in evidence serialization ([#798](https://github.com/informalsystems/tendermint-rs/pull/798))
- Release v0.18.0 ([#796](https://github.com/informalsystems/tendermint-rs/pull/796))
- Update Tokio to 1.0, Hyper to 0.14, Prost to 0.7 and Bytes to 1.0 ([#783](https://github.com/informalsystems/tendermint-rs/pull/783))
- Release v0.17.1 ([#778](https://github.com/informalsystems/tendermint-rs/pull/778))
- Fix formatting of tendermint::Time ([#775](https://github.com/informalsystems/tendermint-rs/pull/775))
- Release v0.17.0 ([#751](https://github.com/informalsystems/tendermint-rs/pull/751))
- Add support for consensus_state endpoint ([#719](https://github.com/informalsystems/tendermint-rs/pull/719))
- Final protobuf for 0.17.0 / Go 0.34.0 ([#737](https://github.com/informalsystems/tendermint-rs/pull/737))
- Added version information to protobuf structs ([#733](https://github.com/informalsystems/tendermint-rs/pull/733))
- Automatically de/serialize ABCI event attributes from/to base64 ([#718](https://github.com/informalsystems/tendermint-rs/pull/718))
- Add tx_search endpoint for RPC client ([#701](https://github.com/informalsystems/tendermint-rs/pull/701))
- Release v0.17.0-rc3 ([#684](https://github.com/informalsystems/tendermint-rs/pull/684))
- :State deserialization fixes ([#680](https://github.com/informalsystems/tendermint-rs/pull/680))
- Rename DomainType trait to Protobuf ([#672](https://github.com/informalsystems/tendermint-rs/pull/672))
- Automatic protobuf module structure creation ([#678](https://github.com/informalsystems/tendermint-rs/pull/678))
- Release v0.17.0-rc2 ([#668](https://github.com/informalsystems/tendermint-rs/pull/668))
- rfc3339 direct ser/deser fix for protobuf Timestamp ([#666](https://github.com/informalsystems/tendermint-rs/pull/666))
- Re-built tendermint-proto with serialization annotations ([#639](https://github.com/informalsystems/tendermint-rs/pull/639))
- Update Tendermint/Rust versions in READMEs ([#642](https://github.com/informalsystems/tendermint-rs/pull/642))
- Release v0.17.0 ([#624](https://github.com/informalsystems/tendermint-rs/pull/624))
- docs update ([#581](https://github.com/informalsystems/tendermint-rs/pull/581))
- Blanket implementation for DomainType ([#571](https://github.com/informalsystems/tendermint-rs/pull/571))
- Tendermint JSON test case fixes ([#563](https://github.com/informalsystems/tendermint-rs/pull/563))
- Replace amino with protobuf types ([#527](https://github.com/informalsystems/tendermint-rs/pull/527))
- Proto update ([#528](https://github.com/informalsystems/tendermint-rs/pull/528))
- bootstrap crate ([#508](https://github.com/informalsystems/tendermint-rs/pull/508))
