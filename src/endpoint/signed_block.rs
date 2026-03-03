//! `/signed_block` endpoint JSON-RPC wrapper

use celestia_types::block::Data;
use serde::{Deserialize, Serialize};
use tendermint::{block, validator};

use crate::prelude::*;
use crate::{dialect::Dialect, request::RequestMessage, serializers};

/// Fetch a signed block at a given height
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Request {
    /// Height of the block to request.
    pub height: Option<block::Height>,
}

impl Request {
    /// Create a new request for a signed block
    pub fn new(height: block::Height) -> Self {
        Self {
            height: Some(height),
        }
    }
}

impl RequestMessage for Request {
    fn method(&self) -> crate::Method {
        crate::Method::SignedBlock
    }
}

impl<S: Dialect> crate::Request<S> for Request {
    type Response = Response;
}

impl<S: Dialect> crate::SimpleRequest<S> for Request {
    type Output = Response;
}

/// Signed block response
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Response {
    /// Block header
    pub header: block::Header,
    /// Block commit
    pub commit: block::Commit,
    /// Block data
    #[serde(with = "serializers::celestia_block_data")]
    pub data: Data,
    /// Validator set
    pub validator_set: validator::Set,
}

impl crate::Response for Response {}
