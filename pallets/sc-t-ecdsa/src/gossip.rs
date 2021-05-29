use std::marker::PhantomData;

use parking_lot::RwLock;
use sc_network::{PeerId, ObservedRole};
use sc_network_gossip::{MessageIntent, ValidationResult as GossipValidationResult, Validator as GossipValidator, ValidatorContext as GossipValidatorContext, ValidatorContext, ValidationResult};
use sp_core::Pair;
use sp_runtime::traits::{Block, Hash, Header, NumberFor};
use std::fmt::Debug;
use codec::Decode;

// Limit THEA gossip by keeping only a bound number of voting rounds alive.
const MAX_LIVE_GOSSIP_ROUNDS: usize = 5;

/// Gossip engine messages topic
pub(crate) fn topic<B: Block>() -> B::Hash
    where
        B: Block,
{
    <<B::Header as Header>::Hashing as Hash>::hash(b"thea")
}

/// THEA gossip validator
///
/// Validate THEA gossip messages and limit the number of live BEEFY voting rounds.
///
/// Allows messages from last [`MAX_LIVE_GOSSIP_ROUNDS`] to flow, everything else gets
/// rejected/expired.
///
///All messaging is handled in a single THEA global topic.
/// TODO: Should we need different messaging for keygen, reshare and signgen?
pub(crate) struct TheaGossipValidator<B, P>
    where
        B: Block,
{
    topic: B::Hash,
    live_rounds: RwLock<Vec<NumberFor<B>>>,
    _pair: PhantomData<P>,
}

impl<B, P> TheaGossipValidator<B, P>
    where
        B: Block,
{
    pub fn new() -> TheaGossipValidator<B, P> {
        TheaGossipValidator {
            topic: topic::<B>(),
            live_rounds: RwLock::new(Vec::new()),
            _pair: PhantomData,
        }
    }

    pub(crate) fn note_round(&self, round: NumberFor<B>) {
        let mut live_rounds = self.live_rounds.write();

        // NOTE: ideally we'd use a VecDeque here, but currently binary search is only available on
        // nightly for `VecDeque`.
        while live_rounds.len() > MAX_LIVE_GOSSIP_ROUNDS {
            let _ = live_rounds.remove(0);
        }

        if let Some(idx) = live_rounds.binary_search(&round).err() {
            live_rounds.insert(idx, round);
        }
    }

    fn is_live(live_rounds: &[NumberFor<B>], round: NumberFor<B>) -> bool {
        live_rounds.binary_search(&round).is_ok()
    }
}


impl<B, P> GossipValidator<B> for TheaGossipValidator<B, P>
    where
        B: Block,
        P: Pair,
        P::Public: Debug + Decode,
        P::Signature: Debug + Decode,
{

    fn validate(&self, context: &mut dyn GossipValidatorContext<B>, sender: &PeerId, data: &[u8]) -> GossipValidationResult<<B as Block>::Hash> {
        todo!()
    }

    fn message_expired<'a>(&'a self) -> Box<dyn FnMut(<B as Block>::Hash, &[u8]) -> bool> {
        let live_rounds = self.live_rounds.read();
        Box::new(move |_topic, mut data| {
            todo!()
        })
    }

    fn message_allowed<'a>(&'a self) -> Box<dyn FnMut(&PeerId, MessageIntent, &<B as Block>::Hash, &[u8]) -> bool> {
        todo!()
    }
}