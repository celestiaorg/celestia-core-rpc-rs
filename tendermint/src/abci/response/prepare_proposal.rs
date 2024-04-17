use crate::block::Data;
use crate::prelude::*;

#[doc = include_str!("../doc/response-prepareproposal.md")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PrepareProposal {
    pub block_data: Data,
}

// =============================================================================
// Protobuf conversions
// =============================================================================

mod v0_34 {
    use super::PrepareProposal;
    use crate::{prelude::*, Error};
    use celestia_core_proto::v0_34::abci as pb;
    use celestia_core_proto::Protobuf;

    impl From<PrepareProposal> for pb::ResponsePrepareProposal {
        fn from(value: PrepareProposal) -> Self {
            Self {
                block_data: Some(value.block_data.into()),
            }
        }
    }

    impl TryFrom<pb::ResponsePrepareProposal> for PrepareProposal {
        type Error = Error;

        fn try_from(message: pb::ResponsePrepareProposal) -> Result<Self, Self::Error> {
            Ok(Self {
                block_data: message
                    .block_data
                    .ok_or_else(Error::missing_data)?
                    .try_into()?,
            })
        }
    }

    impl Protobuf<pb::ResponsePrepareProposal> for PrepareProposal {}
}
