use crate::block::Data;
use crate::prelude::*;

#[doc = include_str!("../doc/request-prepareproposal.md")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PrepareProposal {
    pub block_data: Data,
    pub block_data_size: i64,
    pub chain_id: String,
    pub height: i64,
    pub time: celestia_tendermint_proto::google::protobuf::Timestamp,
}

// =============================================================================
// Protobuf conversions
// =============================================================================

mod v0_34 {
    use super::PrepareProposal;
    use crate::{prelude::*, Error};
    use celestia_tendermint_proto::v0_34::abci as pb;
    use celestia_tendermint_proto::Protobuf;

    impl From<PrepareProposal> for pb::RequestPrepareProposal {
        fn from(value: PrepareProposal) -> Self {
            Self {
                block_data: Some(value.block_data.into()),
                block_data_size: value.block_data_size,
                chain_id: value.chain_id,
                height: value.height,
                time: Some(value.time),
            }
        }
    }

    impl TryFrom<pb::RequestPrepareProposal> for PrepareProposal {
        type Error = Error;

        fn try_from(message: pb::RequestPrepareProposal) -> Result<Self, Self::Error> {
            Ok(PrepareProposal {
                block_data: message
                    .block_data
                    .ok_or_else(Error::missing_data)?
                    .try_into()?,
                block_data_size: message.block_data_size,
                chain_id: message.chain_id,
                height: message.height,
                time: message.time.ok_or_else(Error::missing_data)?.try_into()?,
            })
        }
    }

    impl Protobuf<pb::RequestPrepareProposal> for PrepareProposal {}
}
