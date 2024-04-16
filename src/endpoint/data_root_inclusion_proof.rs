//! `/data_root_inclusion_proof` endpoint JSON-RPC wrapper

use serde::{Deserialize, Serialize};
use tendermint::merkle;

use crate::prelude::*;
use crate::{dialect::Dialect, request::RequestMessage, serializers};

/// Generate an inclusion proof for a data root at a given height inside of a range
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Request {
    /// Height to prove
    #[serde(with = "serializers::from_str")]
    pub height: u64,
    /// Start of range
    #[serde(with = "serializers::from_str")]
    pub start: u64,
    /// End of range
    #[serde(with = "serializers::from_str")]
    pub end: u64,
}

impl Request {
    /// Create a DataRootInclusionProof request
    pub fn new(height: u64, start: u64, end: u64) -> Self {
        Self { height, start, end }
    }
}

impl RequestMessage for Request {
    fn method(&self) -> crate::Method {
        crate::Method::DataRootInclusionProof
    }
}

impl<S: Dialect> crate::Request<S> for Request {
    type Response = Response;
}

impl<S: Dialect> crate::SimpleRequest<S> for Request {
    type Output = Response;
}

/// DataRootInclusionProof query response wrapper
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Response {
    /// Data Root Inclusion Proof response
    pub proof: Option<merkle::Proof>,
}

impl crate::Response for Response {}
