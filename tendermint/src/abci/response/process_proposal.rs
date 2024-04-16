use crate::prelude::*;

use bytes::Bytes;

#[doc = include_str!("../doc/response-processproposal.md")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProcessProposal {
    pub result: ProcessProposalResult,
    pub evidence: Vec<Bytes>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(i32)]
#[derive(Default)]
pub enum ProcessProposalResult {
    #[default]
    Unknown = 0,
    Accept = 1,
    Reject = 2,
}

// =============================================================================
// Protobuf conversions
// =============================================================================

mod v0_34 {
    use super::{ProcessProposal, ProcessProposalResult};
    use crate::Error;
    use celestia_tendermint_proto::v0_34::abci as pb;
    use celestia_tendermint_proto::Protobuf;

    impl From<ProcessProposal> for pb::ResponseProcessProposal {
        fn from(value: ProcessProposal) -> pb::ResponseProcessProposal {
            pb::ResponseProcessProposal {
                result: value.result as i32,
                evidence: value.evidence,
            }
        }
    }

    impl TryFrom<pb::ResponseProcessProposal> for ProcessProposal {
        type Error = Error;

        fn try_from(message: pb::ResponseProcessProposal) -> Result<Self, Self::Error> {
            use pb::response_process_proposal::Result;

            let result = Result::try_from(message.result)
                .map_err(|_| Error::unsupported_process_proposal_result())?;

            let result = match result {
                Result::Unknown => ProcessProposalResult::Unknown,
                Result::Accept => ProcessProposalResult::Accept,
                Result::Reject => ProcessProposalResult::Reject,
            };

            Ok(ProcessProposal {
                result,
                evidence: message.evidence,
            })
        }
    }

    impl Protobuf<pb::ResponseProcessProposal> for ProcessProposal {}
}
