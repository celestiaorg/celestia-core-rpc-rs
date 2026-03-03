//! `/unsafe_flush_mempool` endpoint JSON-RPC wrapper

use serde::{Deserialize, Serialize};

use crate::prelude::*;
use crate::{dialect::Dialect, request::RequestMessage};

/// Request to flush the mempool (unsafe).
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Request;

impl RequestMessage for Request {
    fn method(&self) -> crate::Method {
        crate::Method::UnsafeFlushMempool
    }
}

impl<S: Dialect> crate::Request<S> for Request {
    type Response = Response;
}

impl<S: Dialect> crate::SimpleRequest<S> for Request {
    type Output = Response;
}

/// Empty response
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Response {}

impl crate::Response for Response {}
