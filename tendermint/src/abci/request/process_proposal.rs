use crate::block::{Data, Header};
use crate::prelude::*;

#[doc = include_str!("../doc/request-processproposal.md")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProcessProposal {
    pub header: Header,
    pub block_data: Data,
}

// =============================================================================
// Protobuf conversions
// =============================================================================

mod v0_34 {
    use super::ProcessProposal;
    use crate::{prelude::*, Error};
    use celestia_core_proto::v0_34::abci as pb;
    use celestia_core_proto::Protobuf;

    impl From<ProcessProposal> for pb::RequestProcessProposal {
        fn from(value: ProcessProposal) -> Self {
            Self {
                header: Some(value.header.into()),
                block_data: Some(value.block_data.into()),
            }
        }
    }

    impl TryFrom<pb::RequestProcessProposal> for ProcessProposal {
        type Error = Error;

        fn try_from(message: pb::RequestProcessProposal) -> Result<Self, Self::Error> {
            Ok(ProcessProposal {
                header: message
                    .header
                    .ok_or_else(Error::missing_header)?
                    .try_into()?,
                block_data: message
                    .block_data
                    .ok_or_else(Error::missing_data)?
                    .try_into()?,
            })
        }
    }

    impl Protobuf<pb::RequestProcessProposal> for ProcessProposal {}
}
