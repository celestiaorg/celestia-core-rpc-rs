//! Hash serialization with validation

use serde::{Deserialize, Deserializer, Serializer};
use subtle_encoding::hex;

use crate::serializers::cow_str::CowStr;
use crate::{hash::Algorithm, prelude::*, Hash};

/// Deserialize hexstring into Hash
pub fn deserialize<'de, D>(deserializer: D) -> Result<Hash, D::Error>
where
    D: Deserializer<'de>,
{
    let hexstring = Option::<CowStr>::deserialize(deserializer)?.unwrap_or_default();
    Hash::from_hex_upper(Algorithm::Sha256, &hexstring).map_err(serde::de::Error::custom)
}

/// Serialize from Hash into hexstring
pub fn serialize<S>(value: &Hash, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let hex_bytes = hex::encode_upper(value.as_bytes());
    let hex_string = String::from_utf8(hex_bytes).map_err(serde::ser::Error::custom)?;
    serializer.serialize_str(&hex_string)
}
