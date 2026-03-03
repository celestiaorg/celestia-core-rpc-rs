//! `/dial_peers` endpoint JSON-RPC wrapper

use serde::{Deserialize, Serialize};

use crate::prelude::*;
use crate::{dialect::Dialect, request::RequestMessage, Method};

/// Request to dial peers (unsafe).
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Request {
    /// Peers to dial
    pub peers: Vec<String>,
    /// Persist peers
    pub persistent: bool,
    /// Dial peers unconditionally
    pub unconditional: bool,
    /// Add peers to private list
    pub private: bool,
}

impl Request {
    /// Create a new dial_peers request
    pub fn new(peers: Vec<String>, persistent: bool, unconditional: bool, private: bool) -> Self {
        Self {
            peers,
            persistent,
            unconditional,
            private,
        }
    }
}

impl RequestMessage for Request {
    fn method(&self) -> Method {
        Method::DialPeers
    }
}

impl<S: Dialect> crate::Request<S> for Request {
    type Response = Response;
}

impl<S: Dialect> crate::SimpleRequest<S> for Request {
    type Output = Response;
}

/// Dial peers response
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Response {
    /// Status log
    pub log: String,
}

impl crate::Response for Response {}
