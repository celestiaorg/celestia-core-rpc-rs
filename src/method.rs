//! JSON-RPC request methods

use core::{
    fmt::{self, Display},
    str::FromStr,
};

use serde::{de::Error as _, Deserialize, Deserializer, Serialize, Serializer};

use crate::{prelude::*, Error};

/// JSON-RPC request methods.
///
/// Serialized as the "method" field of JSON-RPC/HTTP requests.
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum Method {
    /// Get ABCI info
    AbciInfo,

    /// Get ABCI query
    AbciQuery,

    /// Get block info
    Block,

    /// Get block info by hash
    BlockByHash,

    /// Get ABCI results for a particular block
    BlockResults,

    /// Search for blocks by their BeginBlock and EndBlock events
    BlockSearch,

    /// Get blockchain info
    Blockchain,

    /// Broadcast transaction asynchronously
    BroadcastTxAsync,

    /// Broadcast transaction synchronously
    BroadcastTxSync,

    /// Broadcast transaction commit
    BroadcastTxCommit,

    /// Get commit info for a block
    Commit,

    /// Get consensus parameters
    ConsensusParams,

    /// Get consensus state
    ConsensusState,

    /// Dump consensus state
    DumpConsensusState,

    /// Generate data root inclusion proof
    DataRootInclusionProof,

    /// Get genesis file
    Genesis,

    /// Get genesis file in multiple chunks
    GenesisChunked,

    /// Get block header
    Header,

    /// Get block header by hash
    HeaderByHash,

    /// Get health info
    Health,

    /// Get network info
    NetInfo,

    /// Check transaction without executing it
    CheckTx,

    /// Get unconfirmed transactions
    UnconfirmedTxs,

    /// Get number of unconfirmed transactions
    NumUnconfirmedTxs,

    /// Get node status
    Status,

    /// Find transaction by hash
    Tx,

    /// Search for transactions with their results
    TxSearch,

    /// Get validator info for a block
    Validators,

    /// Subscribe to events
    Subscribe,

    /// Unsubscribe from events
    Unsubscribe,

    /// Unsubscribe from all events
    UnsubscribeAll,

    /// Broadcast evidence
    BroadcastEvidence,

    /// Broadcast shares proof (deprecated)
    ProveShares,

    /// Broadcast shares proof (v2)
    ProveSharesV2,

    /// Fetch signed block data
    SignedBlock,

    /// Fetch data commitment
    DataCommitment,

    /// Fetch tx status
    TxStatus,

    /// Fetch tx status batch
    TxStatusBatch,

    /// Dial seeds (unsafe)
    DialSeeds,

    /// Dial peers (unsafe)
    DialPeers,

    /// Flush mempool (unsafe)
    UnsafeFlushMempool,
}

impl Method {
    /// Get a static string which represents this method name
    pub fn as_str(self) -> &'static str {
        match self {
            Method::AbciInfo => "abci_info",
            Method::AbciQuery => "abci_query",
            Method::Block => "block",
            Method::BlockByHash => "block_by_hash",
            Method::BlockResults => "block_results",
            Method::BlockSearch => "block_search",
            Method::Blockchain => "blockchain",
            Method::BroadcastEvidence => "broadcast_evidence",
            Method::BroadcastTxAsync => "broadcast_tx_async",
            Method::BroadcastTxSync => "broadcast_tx_sync",
            Method::BroadcastTxCommit => "broadcast_tx_commit",
            Method::Commit => "commit",
            Method::ConsensusParams => "consensus_params",
            Method::ConsensusState => "consensus_state",
            Method::DumpConsensusState => "dump_consensus_state",
            Method::DataRootInclusionProof => "data_root_inclusion_proof",
            Method::Genesis => "genesis",
            Method::GenesisChunked => "genesis_chunked",
            Method::Header => "header",
            Method::HeaderByHash => "header_by_hash",
            Method::Health => "health",
            Method::NetInfo => "net_info",
            Method::CheckTx => "check_tx",
            Method::UnconfirmedTxs => "unconfirmed_txs",
            Method::NumUnconfirmedTxs => "num_unconfirmed_txs",
            Method::ProveShares => "prove_shares",
            Method::ProveSharesV2 => "prove_shares_v2",
            Method::Status => "status",
            Method::SignedBlock => "signed_block",
            Method::DataCommitment => "data_commitment",
            Method::Subscribe => "subscribe",
            Method::Tx => "tx",
            Method::TxSearch => "tx_search",
            Method::TxStatus => "tx_status",
            Method::TxStatusBatch => "tx_status_batch",
            Method::Unsubscribe => "unsubscribe",
            Method::UnsubscribeAll => "unsubscribe_all",
            Method::Validators => "validators",
            Method::DialSeeds => "dial_seeds",
            Method::DialPeers => "dial_peers",
            Method::UnsafeFlushMempool => "unsafe_flush_mempool",
        }
    }
}

impl FromStr for Method {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        Ok(match s {
            "abci_info" => Method::AbciInfo,
            "abci_query" => Method::AbciQuery,
            "block" => Method::Block,
            "block_by_hash" => Method::BlockByHash,
            "block_results" => Method::BlockResults,
            "header" => Method::Header,
            "header_by_hash" => Method::HeaderByHash,
            "block_search" => Method::BlockSearch,
            "blockchain" => Method::Blockchain,
            "broadcast_evidence" => Method::BroadcastEvidence,
            "broadcast_tx_async" => Method::BroadcastTxAsync,
            "broadcast_tx_sync" => Method::BroadcastTxSync,
            "broadcast_tx_commit" => Method::BroadcastTxCommit,
            "commit" => Method::Commit,
            "consensus_params" => Method::ConsensusParams,
            "data_root_inclusion_proof" => Method::DataRootInclusionProof,
            "consensus_state" => Method::ConsensusState,
            "dump_consensus_state" => Method::DumpConsensusState,
            "genesis" => Method::Genesis,
            "genesis_chunked" => Method::GenesisChunked,
            "health" => Method::Health,
            "net_info" => Method::NetInfo,
            "check_tx" => Method::CheckTx,
            "unconfirmed_txs" => Method::UnconfirmedTxs,
            "num_unconfirmed_txs" => Method::NumUnconfirmedTxs,
            "prove_shares" => Method::ProveShares,
            "prove_shares_v2" => Method::ProveSharesV2,
            "status" => Method::Status,
            "signed_block" => Method::SignedBlock,
            "data_commitment" => Method::DataCommitment,
            "subscribe" => Method::Subscribe,
            "tx" => Method::Tx,
            "tx_search" => Method::TxSearch,
            "tx_status" => Method::TxStatus,
            "tx_status_batch" => Method::TxStatusBatch,
            "unsubscribe" => Method::Unsubscribe,
            "unsubscribe_all" => Method::UnsubscribeAll,
            "validators" => Method::Validators,
            "dial_seeds" => Method::DialSeeds,
            "dial_peers" => Method::DialPeers,
            "unsafe_flush_mempool" => Method::UnsafeFlushMempool,
            other => return Err(Error::method_not_found(other.to_string())),
        })
    }
}

impl Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Serialize for Method {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.as_str().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Method {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::from_str(&String::deserialize(deserializer)?)
            .map_err(|e| D::Error::custom(format!("{e}")))
    }
}
