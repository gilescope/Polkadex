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
use log::{debug, error, trace, warn};
use parking_lot::Mutex;
use sc_client_api::{Backend, FinalityNotification, FinalityNotifications};
use sc_network_gossip::GossipEngine;
use sp_api::BlockId;
use sp_application_crypto::{AppPublic, Public};
use sp_core::Pair;
use sp_keystore::{SyncCryptoStore, SyncCryptoStorePtr};
use sp_runtime::{
    generic::OpaqueDigestItemId,
    traits::{Block, Header, NumberFor},
    SaturatedConversion,
};

use hex::ToHex;
use sp_arithmetic::traits::AtLeast32Bit;

use crate::{
    gossip::{topic, TheaGossipValidator},
    metric_inc, metric_set,
    metrics::Metrics,
    Client,
};

use thea_primitives::{TheaApi, ValidatorSet, GENESIS_AUTHORITY_SET_ID, KEY_TYPE, THEA_ENGINE_ID};

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
