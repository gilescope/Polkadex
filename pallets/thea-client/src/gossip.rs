// Copyright (C) 2020-2021 Polkadex OU
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

// This is file is modified from beefy-gadget from Parity Technologies (UK) Ltd.
use std::marker::PhantomData;

use codec::Decode;
use parking_lot::RwLock;
use sc_network::{ObservedRole, PeerId};
use sc_network_gossip::{
    MessageIntent, ValidationResult as GossipValidationResult, ValidationResult,
    Validator as GossipValidator, ValidatorContext as GossipValidatorContext, ValidatorContext,
};
use sp_core::Pair;
use sp_runtime::traits::{Block, Hash, Header, NumberFor};
use std::fmt::Debug;

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
    fn validate(
        &self,
        context: &mut dyn GossipValidatorContext<B>,
        sender: &PeerId,
        data: &[u8],
    ) -> GossipValidationResult<<B as Block>::Hash> {
        todo!()
    }

    fn message_expired<'a>(&'a self) -> Box<dyn FnMut(<B as Block>::Hash, &[u8]) -> bool> {
        let live_rounds = self.live_rounds.read();
        Box::new(move |_topic, mut data| todo!())
    }

    fn message_allowed<'a>(
        &'a self,
    ) -> Box<dyn FnMut(&PeerId, MessageIntent, &<B as Block>::Hash, &[u8]) -> bool> {
        todo!()
    }
}
