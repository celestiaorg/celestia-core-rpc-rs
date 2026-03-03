//! `/num_unconfirmed_txs` endpoint JSON-RPC wrapper

use serde::{Deserialize, Serialize};

use crate::prelude::*;
use crate::{dialect::Dialect, request::RequestMessage, serializers};

/// Get the number of unconfirmed transactions
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Request;

impl RequestMessage for Request {
    fn method(&self) -> crate::Method {
        crate::Method::NumUnconfirmedTxs
    }
}

impl<S: Dialect> crate::Request<S> for Request {
    type Response = Response;
}

impl<S: Dialect> crate::SimpleRequest<S> for Request {
    type Output = Response;
}

/// NumUnconfirmedTxs response
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
    /// Transactions (empty for this endpoint)
    #[serde(default, with = "serializers::bytes::vec_base64string")]
    pub txs: Vec<Vec<u8>>,
}

impl crate::Response for Response {}
