use super::super::Event;
use crate::{consensus, prelude::*, validator};

#[doc = include_str!("../doc/response-endblock.md")]
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct EndBlock {
    /// Changes to the validator set, if any.
    ///
    /// Setting the voting power to 0 removes a validator.
    pub validator_updates: Vec<validator::Update>,
    /// Changes to consensus parameters (optional).
    pub consensus_param_updates: Option<consensus::Params>,
    /// Events that occurred while ending the block.
    pub events: Vec<Event>,
}

// =============================================================================
// Protobuf conversions
// =============================================================================

tendermint_pb_modules! {
    use super::EndBlock;

    impl From<EndBlock> for pb::abci::ResponseEndBlock {
        fn from(end_block: EndBlock) -> Self {
            Self {
                validator_updates: end_block
                    .validator_updates
                    .into_iter()
                    .map(Into::into)
                    .collect(),
                consensus_param_updates: end_block.consensus_param_updates.map(Into::into),
                events: end_block.events.into_iter().map(Into::into).collect(),
            }
        }
    }

    impl TryFrom<pb::abci::ResponseEndBlock> for EndBlock {
        type Error = crate::Error;

        fn try_from(end_block: pb::abci::ResponseEndBlock) -> Result<Self, Self::Error> {
            Ok(Self {
                validator_updates: end_block
                    .validator_updates
                    .into_iter()
                    .map(TryInto::try_into)
                    .collect::<Result<_, _>>()?,
                consensus_param_updates: end_block
                    .consensus_param_updates
                    .map(TryInto::try_into)
                    .transpose()?,
                events: end_block
                    .events
                    .into_iter()
                    .map(TryInto::try_into)
                    .collect::<Result<_, _>>()?,
            })
        }
    }

    impl Protobuf<pb::abci::ResponseEndBlock> for EndBlock {}
}
