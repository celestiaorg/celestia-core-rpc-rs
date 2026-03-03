//! `/dump_consensus_state` endpoint JSON-RPC wrapper

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::prelude::*;
use crate::{dialect::Dialect, request::RequestMessage, Method};

/// Dump the current consensus state.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Request;

impl RequestMessage for Request {
    fn method(&self) -> Method {
        Method::DumpConsensusState
    }
}

impl<S: Dialect> crate::Request<S> for Request {
    type Response = Response;
}

impl<S: Dialect> crate::SimpleRequest<S> for Request {
    type Output = Response;
}

/// DumpConsensusState response
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Response {
    /// Consensus round state
    pub round_state: Value,
    /// Peer states
    pub peers: Vec<PeerStateInfo>,
}

impl crate::Response for Response {}

/// Peer state information
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PeerStateInfo {
    /// Peer address
    pub node_address: String,
    /// Peer consensus state
    pub peer_state: Value,
}
