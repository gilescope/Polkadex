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

#![cfg_attr(not(feature = "std"), no_std)]

use codec::Encode;

use frame_support::{traits::OneSessionHandler, Parameter};

use sp_runtime::{
    generic::DigestItem,
    traits::{IsMember, Member},
    RuntimeAppPublic,
};
use sp_std::prelude::*;

use thea_primitives::{AuthorityIndex, ConsensusLog, ValidatorSet, THEA_ENGINE_ID};

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Authority identifier type
        type AuthorityId: Member
            + Parameter
            + RuntimeAppPublic
            + Default
            + MaybeSerializeDeserialize;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {}

    /// The current authorities set
    #[pallet::storage]
    #[pallet::getter(fn authorities)]
    pub(super) type Authorities<T: Config> = StorageValue<_, Vec<T::AuthorityId>, ValueQuery>;

    /// Flag to start THEA round
    #[pallet::storage]
    #[pallet::getter(fn can_start_flag)]
    pub(super) type TheaFlag<T: Config> = StorageValue<_, bool, ValueQuery>;

    /// The current validator set id
    #[pallet::storage]
    #[pallet::getter(fn validator_set_id)]
    pub(super) type ValidatorSetId<T: Config> =
        StorageValue<_, thea_primitives::ValidatorSetId, ValueQuery>;

    /// Authorities set scheduled to be used with the next session
    #[pallet::storage]
    #[pallet::getter(fn next_authorities)]
    pub(super) type NextAuthorities<T: Config> = StorageValue<_, Vec<T::AuthorityId>, ValueQuery>;

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub authorities: Vec<T::AuthorityId>,
        pub can_start: bool,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                authorities: Vec::new(),
                can_start: false,
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            Pallet::<T>::initialize_authorities(&self.authorities);
            Pallet::<T>::initialize_can_start(&self.can_start);
        }
    }
}

impl<T: Config> Pallet<T> {
    /// Return the current active THEA validator set.
    pub fn validator_set() -> ValidatorSet<T::AuthorityId> {
        ValidatorSet::<T::AuthorityId> {
            validators: Self::authorities(),
            id: Self::validator_set_id(),
        }
    }

    pub fn can_start() -> bool {
        Self::can_start_flag()
    }

    fn change_authorities(new: Vec<T::AuthorityId>, queued: Vec<T::AuthorityId>) {
        // As in GRANDPA, we trigger a validator set change only if the the validator
        // set has actually changed.
        if new != Self::authorities() {
            <Authorities<T>>::put(&new);

            let next_id = Self::validator_set_id() + 1u64;
            <ValidatorSetId<T>>::put(next_id);

            let log: DigestItem<T::Hash> = DigestItem::Consensus(
                THEA_ENGINE_ID,
                ConsensusLog::AuthoritiesChange(ValidatorSet {
                    validators: new,
                    id: next_id,
                })
                .encode(),
            );
            <frame_system::Pallet<T>>::deposit_log(log);
        }

        <NextAuthorities<T>>::put(&queued);
    }

    fn initialize_authorities(authorities: &[T::AuthorityId]) {
        if authorities.is_empty() {
            return;
        }

        assert!(
            <Authorities<T>>::get().is_empty(),
            "Authorities are already initialized!"
        );

        <Authorities<T>>::put(authorities);
        <ValidatorSetId<T>>::put(0);
        // Like `pallet_session`, initialize the next validator set as well.
        <NextAuthorities<T>>::put(authorities);
    }
    fn initialize_can_start(flag: &bool) {
        <TheaFlag<T>>::put(flag);
    }
}

impl<T: Config> sp_runtime::BoundToRuntimeAppPublic for Pallet<T> {
    type Public = T::AuthorityId;
}

impl<T: Config> OneSessionHandler<T::AccountId> for Pallet<T> {
    type Key = T::AuthorityId;

    fn on_genesis_session<'a, I: 'a>(validators: I)
    where
        I: Iterator<Item = (&'a T::AccountId, T::AuthorityId)>,
    {
        let authorities = validators.map(|(_, k)| k).collect::<Vec<_>>();
        Self::initialize_authorities(&authorities);
    }

    fn on_new_session<'a, I: 'a>(changed: bool, validators: I, queued_validators: I)
    where
        I: Iterator<Item = (&'a T::AccountId, T::AuthorityId)>,
    {
        if changed {
            let next_authorities = validators.map(|(_, k)| k).collect::<Vec<_>>();
            let next_queued_authorities = queued_validators.map(|(_, k)| k).collect::<Vec<_>>();

            Self::change_authorities(next_authorities, next_queued_authorities);
        }
    }

    fn on_disabled(i: usize) {
        let log: DigestItem<T::Hash> = DigestItem::Consensus(
            THEA_ENGINE_ID,
            ConsensusLog::<T::AuthorityId>::OnDisabled(i as AuthorityIndex).encode(),
        );

        <frame_system::Pallet<T>>::deposit_log(log);
    }
}

impl<T: Config> IsMember<T::AuthorityId> for Pallet<T> {
    fn is_member(authority_id: &T::AuthorityId) -> bool {
        Self::authorities().iter().any(|id| id == authority_id)
    }
}
