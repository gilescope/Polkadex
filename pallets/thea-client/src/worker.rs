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
use std::{
    convert::{TryFrom, TryInto},
    fmt::Debug,
    marker::PhantomData,
    sync::Arc,
};

use codec::{Codec, Decode, Encode};
use futures::{future, FutureExt, StreamExt};
use hex::ToHex;
use log::{debug, error, trace, warn};
use parking_lot::Mutex;
use round_based::Msg;
use sc_client_api::{Backend, FinalityNotification, FinalityNotifications};
use sc_network_gossip::GossipEngine;
use sp_api::BlockId;
use sp_application_crypto::{AppPublic, Public};
use sp_arithmetic::traits::AtLeast32Bit;
use sp_core::Pair;
use sp_keystore::{SyncCryptoStore, SyncCryptoStorePtr};
use sp_runtime::{
    generic::OpaqueDigestItemId,
    traits::{Block, Header, NumberFor},
    SaturatedConversion,
};

use thea_primitives::{
    ConsensusLog, TheaApi, ValidatorSet, GENESIS_AUTHORITY_SET_ID, KEY_TYPE, THEA_ENGINE_ID,
};

use crate::{
    gossip::{topic, TheaGossipValidator},
    metric_inc, metric_set,
    metrics::Metrics,
    mpc::ProtocolMessage,
    Client,
};

pub(crate) struct WorkerParams<B, P, BE, C>
where
    B: Block,
    P: sp_core::Pair,
    P::Signature: Clone + Codec + Debug + PartialEq + TryFrom<Vec<u8>>,
{
    pub client: Arc<C>,
    pub backend: Arc<BE>,
    pub key_store: Option<SyncCryptoStorePtr>,
    pub gossip_engine: GossipEngine<B>,
    pub gossip_validator: Arc<TheaGossipValidator<B, P>>,
    pub party_idx: u16,
    pub threshold: u16,
    pub party_count: u16,
    pub metrics: Option<Metrics>,
}

/// A THEA worker plays the BEEFY protocol
pub(crate) struct TheaWorker<B, C, BE, P>
where
    B: Block,
    BE: Backend<B>,
    P: Pair,
    P::Public: AppPublic + Codec,
    P::Signature: Clone + Codec + Debug + PartialEq + TryFrom<Vec<u8>>,
    C: Client<B, BE, P>,
{
    client: Arc<C>,
    backend: Arc<BE>,
    key_store: Option<SyncCryptoStorePtr>,
    gossip_engine: Arc<Mutex<GossipEngine<B>>>,
    gossip_validator: Arc<TheaGossipValidator<B, P>>,
    /// Index of this worker
    party_idx: u16,
    /// Threshold of the protocol for signing
    threshold: u16,
    /// Total number of parties
    party_count: u16,
    metrics: Option<Metrics>,
    finality_notifications: FinalityNotifications<B>,
    /// Best block we received a GRANDPA notification for
    best_grandpa_block: NumberFor<B>,
    /// Validator set id for the last signed commitment
    last_signed_id: u64,
    // keep rustc happy
    _backend: PhantomData<BE>,
    _pair: PhantomData<P>,
}

impl<B, C, BE, P> TheaWorker<B, C, BE, P>
where
    B: Block,
    BE: Backend<B>,
    P: Pair,
    P::Public: AppPublic,
    P::Signature: Clone + Codec + Debug + PartialEq + TryFrom<Vec<u8>>,
    C: Client<B, BE, P>,
    C::Api: TheaApi<B, P::Public>,
{
    /// Return a new Thea worker instance.
    ///
    /// Note that a Thea worker is only fully functional if a corresponding
    /// Thea pallet has been deployed on-chain.
    ///
    /// The Thea pallet is needed in order to keep track of the Thea authority set.
    pub(crate) fn new(worker_params: WorkerParams<B, P, BE, C>) -> Self {
        let WorkerParams {
            client,
            backend,
            key_store,
            gossip_engine,
            gossip_validator,
            party_idx,
            threshold,
            party_count,
            metrics,
        } = worker_params;

        TheaWorker {
            client: client.clone(),
            backend,
            key_store,
            gossip_engine: Arc::new(Mutex::new(gossip_engine)),
            gossip_validator,
            party_idx,
            threshold,
            party_count,
            metrics,
            // rounds: round::Rounds::new(ValidatorSet::empty()),
            finality_notifications: client.finality_notification_stream(),
            best_grandpa_block: client.info().finalized_number,
            // best_beefy_block: None,
            last_signed_id: 0,
            _backend: PhantomData,
            _pair: PhantomData,
        }
    }
}

impl<B, C, BE, P> TheaWorker<B, C, BE, P>
where
    B: Block,
    BE: Backend<B>,
    P: Pair,
    P::Public: AppPublic,
    P::Signature: Clone + Codec + Debug + PartialEq + TryFrom<Vec<u8>>,
    C: Client<B, BE, P>,
    C::Api: TheaApi<B, P::Public>,
{
    // TODO: Implement the threshold ecdsa logic here

    pub fn handle_finality_notification(&mut self, notification: FinalityNotification<B>) {
        trace!(target: "thea", "🥩 Got New Finality notification: {:?}", notification.header.number());
    }

    pub fn handle_protocol_message(&mut self, message: Msg<ProtocolMessage>) {
        trace!(target: "thea", "🥩 Got New Protocol Message: Sender {:?}, Receiver: {:?}", message.sender,message.receiver);
    }
    pub(crate) async fn run(mut self) {
        let mut thea_protocol_messages = Box::pin(
            self.gossip_engine
                .lock()
                .messages_for(topic::<B>())
                .filter_map(|notification| async move {
                    trace!(target: "thea", "🥩 Got Protocol message on wire: {:?}", notification);

                    // VoteMessage::<MmrRootHash, NumberFor<B>, P::Public, P::Signature>::decode(
                    //     &mut &notification.message[..],
                    // )
                    // .ok();
                    None
                }),
        );

        loop {
            let engine = self.gossip_engine.clone();
            let gossip_engine = future::poll_fn(|cx| engine.lock().poll_unpin(cx));

            futures::select! {
                notification = self.finality_notifications.next().fuse() => {
                    if let Some(notification) = notification {
                        self.handle_finality_notification(notification);
                    } else {
                        return;
                    }
                },
                thea_protocol_message = thea_protocol_messages.next().fuse() => {
                    if let Some(message) = thea_protocol_message {
                        self.handle_protocol_message(message);
                    } else {
                        return;
                    }
                },
                _ = gossip_engine.fuse() => {
                    error!(target: "thea", "🥩 Gossip engine has terminated.");
                    return;
                }
            }
        }
    }
}

/// Scan the `header` digest log for a THEA validator set change. Return either the new
/// validator set or `None` in case no validator set change has been signaled.
fn find_authorities_change<B, Id>(header: &B::Header) -> Option<ValidatorSet<Id>>
where
    B: Block,
    Id: Codec,
{
    let id = OpaqueDigestItemId::Consensus(&THEA_ENGINE_ID);

    let filter = |log: ConsensusLog<Id>| match log {
        ConsensusLog::AuthoritiesChange(validator_set) => Some(validator_set),
        _ => None,
    };

    header
        .digest()
        .convert_first(|l| l.try_to(id).and_then(filter))
}
