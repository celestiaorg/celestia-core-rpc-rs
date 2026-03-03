//! `/check_tx` endpoint JSON-RPC wrapper

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tendermint::abci;

use crate::dialect::{self, Dialect};
use crate::prelude::*;
use crate::{request::RequestMessage, serializers, Method};

/// Request to check a transaction without executing it.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Request {
    /// Transaction to check
    #[serde(with = "serializers::bytes::base64string")]
    pub tx: Vec<u8>,
}

impl Request {
    /// Create a new check_tx request
    pub fn new(tx: impl Into<Vec<u8>>) -> Self {
        Self { tx: tx.into() }
    }
}

impl RequestMessage for Request {
    fn method(&self) -> Method {
        Method::CheckTx
    }
}

impl<S: Dialect> crate::Request<S> for Request {
    type Response = DialectResponse<S::Event>;
}

impl<S: Dialect> crate::SimpleRequest<S> for Request {
    type Output = Response;
}

/// CheckTx response
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Response(pub abci::response::CheckTx);

impl crate::Response for Response {}

/// RPC dialect helper for serialization of the response.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DialectResponse<Ev>(pub dialect::CheckTx<Ev>);

impl<Ev> crate::Response for DialectResponse<Ev> where Ev: Serialize + DeserializeOwned {}

impl<Ev> From<DialectResponse<Ev>> for Response
where
    Ev: Into<abci::Event>,
{
    fn from(msg: DialectResponse<Ev>) -> Self {
        Self(msg.0.into())
    }
}
