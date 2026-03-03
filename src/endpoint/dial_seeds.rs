//! `/dial_seeds` endpoint JSON-RPC wrapper

use serde::{Deserialize, Serialize};

use crate::prelude::*;
use crate::{dialect::Dialect, request::RequestMessage, Method};

/// Request to dial seeds (unsafe).
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Request {
    /// Seeds to dial
    pub seeds: Vec<String>,
}

impl Request {
    /// Create a new dial_seeds request
    pub fn new(seeds: Vec<String>) -> Self {
        Self { seeds }
    }
}

impl RequestMessage for Request {
    fn method(&self) -> Method {
        Method::DialSeeds
    }
}

impl<S: Dialect> crate::Request<S> for Request {
    type Response = Response;
}

impl<S: Dialect> crate::SimpleRequest<S> for Request {
    type Output = Response;
}

/// Dial seeds response
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Response {
    /// Status log
    pub log: String,
}

impl crate::Response for Response {}
