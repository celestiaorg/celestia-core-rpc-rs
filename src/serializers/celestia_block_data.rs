//! Serde helpers for Celestia block data JSON.

use celestia_types::block::Data;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;
use subtle_encoding::{base64, hex};

use crate::prelude::*;
/// Deserialize Celestia block data from JSON.
pub fn deserialize<'de, D>(deserializer: D) -> Result<Data, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;
    from_value(value).map_err(serde::de::Error::custom)
}

/// Serialize Celestia block data to JSON.
pub fn serialize<S>(value: &Data, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    value.serialize(serializer)
}

pub(crate) fn from_value(value: Value) -> Result<Data, String> {
    let obj = value
        .as_object()
        .ok_or_else(|| "expected block data object".to_string())?;

    let txs = parse_txs(obj.get("txs"))?;
    let square_size = parse_u64(obj.get("square_size").or_else(|| obj.get("squareSize")))?;
    let hash = parse_hash_bytes(obj.get("hash"))?;

    Ok(Data {
        txs,
        square_size,
        hash,
    })
}

fn parse_txs(value: Option<&Value>) -> Result<Vec<Vec<u8>>, String> {
    let Some(value) = value else {
        return Ok(Vec::new());
    };
    if value.is_null() {
        return Ok(Vec::new());
    }
    let list = value
        .as_array()
        .ok_or_else(|| "expected txs array".to_string())?;
    list.iter()
        .map(|item| {
            let s = item
                .as_str()
                .ok_or_else(|| "expected base64 string tx".to_string())?;
            base64::decode(s).map_err(|e| format!("invalid base64 tx: {e}"))
        })
        .collect()
}

fn parse_u64(value: Option<&Value>) -> Result<u64, String> {
    let Some(value) = value else {
        return Ok(0);
    };
    match value {
        Value::Number(number) => number
            .as_u64()
            .ok_or_else(|| "expected u64 number".to_string()),
        Value::String(s) => s
            .parse::<u64>()
            .map_err(|e| format!("invalid u64 string: {e}")),
        Value::Null => Ok(0),
        _ => Err("expected u64 or string".to_string()),
    }
}

fn parse_hash_bytes(value: Option<&Value>) -> Result<Vec<u8>, String> {
    let Some(value) = value else {
        return Ok(Vec::new());
    };
    if value.is_null() {
        return Ok(Vec::new());
    }
    let s = value
        .as_str()
        .ok_or_else(|| "expected hash string".to_string())?;
    if s.is_empty() {
        return Ok(Vec::new());
    }
    if is_hex_string(s) {
        return hex::decode_upper(s)
            .or_else(|_| hex::decode(s))
            .map_err(|e| format!("invalid hash hex encoding: {e}"));
    }
    base64::decode(s).map_err(|e| format!("invalid hash base64 encoding: {e}"))
}

fn is_hex_string(value: &str) -> bool {
    !value.is_empty()
        && value.len() % 2 == 0
        && value.as_bytes().iter().all(|b| b.is_ascii_hexdigit())
}
