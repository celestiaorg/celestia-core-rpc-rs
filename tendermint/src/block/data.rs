use celestia_tendermint_proto::v0_34::types::Data as RawData;
use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
#[serde(try_from = "RawData", into = "RawData")]
pub struct Data {
    pub txs: Vec<Vec<u8>>,
    pub square_size: u64,
    pub hash: Vec<u8>,
}

mod v0_34 {
    use super::Data;
    use crate::{prelude::*, Error};
    use celestia_tendermint_proto::v0_34::types::Data as RawData;
    use celestia_tendermint_proto::Protobuf;

    impl Protobuf<RawData> for Data {}

    impl TryFrom<RawData> for Data {
        type Error = Error;

        fn try_from(value: RawData) -> Result<Self, Self::Error> {
            Ok(Data {
                txs: value.txs,
                square_size: value.square_size,
                hash: value.hash,
            })
        }
    }

    impl From<Data> for RawData {
        fn from(value: Data) -> RawData {
            RawData {
                txs: value.txs,
                square_size: value.square_size,
                hash: value.hash,
            }
        }
    }
}
