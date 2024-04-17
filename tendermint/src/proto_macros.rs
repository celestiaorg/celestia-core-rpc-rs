//! Macros to facilitate protobuf conversions

macro_rules! tendermint_pb_modules {
    {
        $($contents:item)*
    } => {
        mod v0_34 {
            use celetia_core_proto::v0_34 as pb;
            #[allow(unused_imports)]
            use celetia_core_proto::Protobuf;

            $($contents)*
        }
    };
}
