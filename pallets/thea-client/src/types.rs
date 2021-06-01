// use codec::{Decode, Encode};
// use round_based::Msg;
//
// use crate::mpc::ProtocolMessage;
// use thea_primitives::ValidatorSetId;
//
// #[derive(Decode, Encode, Copy, Clone)]
// pub struct TheaRoundMessage<TBlockNumber> {
//     /// The serialized payload of t-ECDSA protocol Message
//     /// this is also used for signing
//     protocol_payload: Vec<u8>,
//
//     /// The round
//     block_number: TBlockNumber,
//
//     validator_set_id: ValidatorSetId,
// }
//
// pub struct SignedTheaRoundMessage<TBlockNumber> {}
