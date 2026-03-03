//! `/unconfirmed_txs` endpoint JSON-RPC wrapper

use serde::{Deserialize, Serialize};

use crate::prelude::*;
use crate::{dialect::Dialect, request::RequestMessage, serializers};

/// Get unconfirmed transactions from the mempool
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Request {
    /// Optional limit
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "serializers::optional_from_str"
    )]
    pub limit: Option<i64>,
}

impl Request {
    /// Create a new unconfirmed_txs request
    pub fn new(limit: Option<i64>) -> Self {
        Self { limit }
    }
}

impl RequestMessage for Request {
    fn method(&self) -> crate::Method {
        crate::Method::UnconfirmedTxs
    }
}

impl<S: Dialect> crate::Request<S> for Request {
    type Response = Response;
}

impl<S: Dialect> crate::SimpleRequest<S> for Request {
    type Output = Response;
}

/// Unconfirmed transactions response
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Response {
    /// Number of unconfirmed transactions
    #[serde(rename = "n_txs", with = "serializers::from_str")]
    pub count: i32,
    /// Total number of unconfirmed transactions
    #[serde(with = "serializers::from_str")]
    pub total: i32,
    /// Total bytes in the mempool
    #[serde(with = "serializers::from_str")]
    pub total_bytes: i64,
    /// Transactions
    #[serde(with = "serializers::bytes::vec_base64string")]
    pub txs: Vec<Vec<u8>>,
}

impl crate::Response for Response {}
