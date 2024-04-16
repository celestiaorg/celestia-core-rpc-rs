//! AppHash serialization with validation

use serde::{Deserialize, Deserializer, Serializer};
use subtle_encoding::hex;

use crate::serializers::cow_str::CowStr;
use crate::{prelude::*, AppHash};

/// Deserialize hexstring into AppHash
pub fn deserialize<'de, D>(deserializer: D) -> Result<AppHash, D::Error>
where
    D: Deserializer<'de>,
{
    let hexstring = Option::<CowStr>::deserialize(deserializer)?.unwrap_or_default();
    AppHash::from_hex_upper(&hexstring).map_err(serde::de::Error::custom)
}

/// Serialize from AppHash into hexstring
pub fn serialize<S>(value: &AppHash, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let hex_bytes = hex::encode_upper(value.as_ref());
    let hex_string = String::from_utf8(hex_bytes).map_err(serde::ser::Error::custom)?;
    serializer.serialize_str(&hex_string)
}
