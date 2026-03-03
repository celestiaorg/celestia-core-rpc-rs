//! `/prove_shares_v2` endpoint JSON-RPC wrapper

use celestia_types::ShareProof;
use serde::{Deserialize, Serialize};

use crate::prelude::*;
use crate::{dialect::Dialect, request::RequestMessage, serializers};

/// Generate a shares proof (v2) for a data root at a given height inside of a range
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Request {
    /// Height for the shares to proof
    #[serde(with = "serializers::from_str")]
    pub height: u64,
    /// Start share for the span
    #[serde(rename = "startShare", with = "serializers::from_str")]
    pub start_share: u64,
    /// End share for the span
    #[serde(rename = "endShare", with = "serializers::from_str")]
    pub end_share: u64,
}

impl Request {
    /// Create a new prove_shares_v2 request
    pub fn new(height: u64, start_share: u64, end_share: u64) -> Self {
        Self {
            height,
            start_share,
            end_share,
        }
    }
}

impl RequestMessage for Request {
    fn method(&self) -> crate::Method {
        crate::Method::ProveSharesV2
    }
}

impl<S: Dialect> crate::Request<S> for Request {
    type Response = Response;
}

impl<S: Dialect> crate::SimpleRequest<S> for Request {
    type Output = Response;
}

/// ProveSharesV2 query response wrapper
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Response {
    /// Share proof response
    #[serde(rename = "share_proof")]
    pub share_proof: ShareProof,
}

impl crate::Response for Response {}
