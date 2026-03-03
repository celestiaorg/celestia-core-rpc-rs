//! `/data_commitment` endpoint JSON-RPC wrapper

use serde::{Deserialize, Serialize};

use crate::prelude::*;
use crate::{dialect::Dialect, request::RequestMessage, serializers};

/// Generate a data commitment for a range of blocks.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Request {
    /// Start of range (inclusive)
    #[serde(with = "serializers::from_str")]
    pub start: u64,
    /// End of range (exclusive)
    #[serde(with = "serializers::from_str")]
    pub end: u64,
}

impl Request {
    /// Create a data commitment request
    pub fn new(start: u64, end: u64) -> Self {
        Self { start, end }
    }
}

impl RequestMessage for Request {
    fn method(&self) -> crate::Method {
        crate::Method::DataCommitment
    }
}

impl<S: Dialect> crate::Request<S> for Request {
    type Response = Response;
}

impl<S: Dialect> crate::SimpleRequest<S> for Request {
    type Output = Response;
}

/// Data commitment response wrapper
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Response {
    /// Data commitment
    #[serde(with = "serializers::bytes::hexstring")]
    pub data_commitment: Vec<u8>,
}

impl crate::Response for Response {}
