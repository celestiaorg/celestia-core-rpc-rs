//! `/tx_status` endpoint JSON-RPC wrapper

use serde::{Deserialize, Serialize};
use tendermint::Hash;

use crate::prelude::*;
use crate::{dialect::Dialect, request::RequestMessage, serializers};

/// Get transaction status by hash
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Request {
    /// Transaction hash
    #[serde(with = "serializers::tx_hash_base64")]
    pub hash: Hash,
}

impl Request {
    /// Create a new tx_status request
    pub fn new(hash: Hash) -> Self {
        Self { hash }
    }
}

impl RequestMessage for Request {
    fn method(&self) -> crate::Method {
        crate::Method::TxStatus
    }
}

impl<S: Dialect> crate::Request<S> for Request {
    type Response = Response;
}

impl<S: Dialect> crate::SimpleRequest<S> for Request {
    type Output = Response;
}

/// Transaction status response
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Response {
    /// Block height
    #[serde(default, with = "serializers::from_str")]
    pub height: i64,
    /// Transaction index
    #[serde(default, with = "serializers::from_str")]
    pub index: u32,
    /// Execution code
    #[serde(rename = "execution_code", default, with = "serializers::from_str")]
    pub execution_code: u32,
    /// Error log
    #[serde(default)]
    pub error: String,
    /// Transaction status
    pub status: String,
    /// Codespace for the error
    #[serde(default)]
    pub codespace: String,
    /// Gas wanted
    #[serde(rename = "gas_wanted", default, with = "serializers::from_str")]
    pub gas_wanted: i64,
    /// Gas used
    #[serde(rename = "gas_used", default, with = "serializers::from_str")]
    pub gas_used: i64,
    /// Signers
    #[serde(default)]
    pub signers: Vec<String>,
}

impl crate::Response for Response {}
