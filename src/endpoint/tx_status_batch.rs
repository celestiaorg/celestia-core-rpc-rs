//! `/tx_status_batch` endpoint JSON-RPC wrapper

use serde::{Deserialize, Serialize};
use subtle_encoding::base64;
use tendermint::{hash::Algorithm, Hash};

use crate::prelude::*;
use crate::{dialect::Dialect, request::RequestMessage};

use super::tx_status;

/// Get transaction statuses by hash (batch)
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Request {
    /// Transaction hashes
    #[serde(
        serialize_with = "serialize_hashes",
        deserialize_with = "deserialize_hashes"
    )]
    pub hashes: Vec<Hash>,
}

impl Request {
    /// Create a new tx_status_batch request
    pub fn new(hashes: Vec<Hash>) -> Self {
        Self { hashes }
    }
}

impl RequestMessage for Request {
    fn method(&self) -> crate::Method {
        crate::Method::TxStatusBatch
    }
}

impl<S: Dialect> crate::Request<S> for Request {
    type Response = Response;
}

impl<S: Dialect> crate::SimpleRequest<S> for Request {
    type Output = Response;
}

/// Transaction status batch response
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Response {
    /// Statuses for each hash
    pub statuses: Vec<TxStatusResponse>,
}

impl crate::Response for Response {}

/// Transaction status response entry
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TxStatusResponse {
    /// Transaction hash
    pub hash: Hash,
    /// Transaction status result
    pub result: tx_status::Response,
}

fn deserialize_hashes<'de, D>(deserializer: D) -> Result<Vec<Hash>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let values = Option::<Vec<String>>::deserialize(deserializer)?.unwrap_or_default();
    values
        .into_iter()
        .map(|value| {
            let decoded = base64::decode(value).map_err(serde::de::Error::custom)?;
            Hash::from_bytes(Algorithm::Sha256, &decoded).map_err(serde::de::Error::custom)
        })
        .collect()
}

fn serialize_hashes<S>(hashes: &[Hash], serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let encoded = hashes
        .iter()
        .map(|hash| String::from_utf8(base64::encode(hash.as_bytes())))
        .collect::<Result<Vec<String>, _>>()
        .map_err(serde::ser::Error::custom)?;
    encoded.serialize(serializer)
}
