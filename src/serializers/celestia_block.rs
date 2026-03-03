//! Serde helpers for Celestia block JSON.

use celestia_types::block::Block;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;
use tendermint::block::{Commit, Header};
use tendermint::evidence;
use tendermint_proto::v0_38::types::EvidenceList as RawEvidenceList;

use super::celestia_block_data;
use crate::prelude::*;

/// Deserialize Celestia block from JSON.
pub fn deserialize<'de, D>(deserializer: D) -> Result<Block, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;
    from_value(value).map_err(serde::de::Error::custom)
}

/// Serialize Celestia block to JSON.
pub fn serialize<S>(value: &Block, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    value.serialize(serializer)
}

pub(crate) fn from_value(value: Value) -> Result<Block, String> {
    let obj = value
        .as_object()
        .ok_or_else(|| "expected block object".to_string())?;

    let header_value = obj
        .get("header")
        .ok_or_else(|| "missing header".to_string())?
        .clone();
    let header: Header = serde_json::from_value(header_value).map_err(|e| e.to_string())?;

    let data_value = obj
        .get("data")
        .ok_or_else(|| "missing data".to_string())?
        .clone();
    let data = celestia_block_data::from_value(data_value)?;

    let evidence = match obj.get("evidence") {
        Some(value) => {
            let raw = serde_json::from_value::<RawEvidenceList>(value.clone())
                .map_err(|e| e.to_string())?;
            evidence::List::try_from(raw).map_err(|e| e.to_string())?
        }
        None => evidence::List::default(),
    };

    let last_commit = match obj.get("last_commit") {
        None => None,
        Some(value) if value.is_null() => None,
        Some(value) => {
            Some(serde_json::from_value::<Commit>(value.clone()).map_err(|e| e.to_string())?)
        }
    };

    Ok(Block::new(header, data, evidence, last_commit))
}

pub mod option {
    use super::*;

    /// Deserialize an optional Celestia block from JSON.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Block>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Option::<Value>::deserialize(deserializer)?;
        match value {
            Some(value) => super::from_value(value)
                .map(Some)
                .map_err(serde::de::Error::custom),
            None => Ok(None),
        }
    }

    /// Serialize an optional Celestia block to JSON.
    pub fn serialize<S>(value: &Option<Block>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(value) => super::serialize(value, serializer),
            None => serializer.serialize_none(),
        }
    }
}
