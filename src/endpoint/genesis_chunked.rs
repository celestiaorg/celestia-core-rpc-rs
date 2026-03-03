//! `/genesis_chunked` endpoint JSON-RPC wrapper

use serde::{Deserialize, Serialize};

use crate::prelude::*;
use crate::{dialect::Dialect, request::RequestMessage, serializers};

/// Get the genesis file in chunks
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Request {
    /// Chunk number
    #[serde(with = "serializers::from_str")]
    pub chunk: u32,
}

impl Request {
    /// Create a new request for a genesis chunk
    pub fn new(chunk: u32) -> Self {
        Self { chunk }
    }
}

impl RequestMessage for Request {
    fn method(&self) -> crate::Method {
        crate::Method::GenesisChunked
    }
}

impl<S: Dialect> crate::Request<S> for Request {
    type Response = Response;
}

impl<S: Dialect> crate::SimpleRequest<S> for Request {
    type Output = Response;
}

/// Genesis chunk response
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Response {
    /// Chunk number
    #[serde(with = "serializers::from_str")]
    pub chunk: u32,
    /// Total chunks
    #[serde(with = "serializers::from_str")]
    pub total: u32,
    /// Base64-encoded chunk data
    pub data: String,
}

impl crate::Response for Response {}
