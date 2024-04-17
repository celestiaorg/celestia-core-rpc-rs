//! Helper types to generalize differences in serialization between
//! Tendermint RPC protocol versions.

pub mod v0_34;

mod begin_block;
mod check_tx;
mod deliver_tx;
mod end_block;

pub use begin_block::BeginBlock;
pub use check_tx::CheckTx;
pub use deliver_tx::DeliverTx;
pub use end_block::EndBlock;

use serde::{de::DeserializeOwned, Serialize};

use celestia_core::{abci, evidence};

pub trait Dialect: sealed::Sealed + Default + Clone + Send + Sync {
    type Event: Into<abci::Event> + Serialize + DeserializeOwned;
    type Evidence: From<evidence::Evidence> + Serialize + DeserializeOwned + Send;
}

pub type LatestDialect = v0_34::Dialect;

mod sealed {
    pub trait Sealed {}

    impl Sealed for super::v0_34::Dialect {}
}
