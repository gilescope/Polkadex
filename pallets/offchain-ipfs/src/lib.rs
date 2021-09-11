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
use frame_support::{traits::OneSessionHandler, PalletId};
use sp_runtime::{generic::DigestItem, traits::IsMember, RuntimeAppPublic};
use sp_std::{prelude::*, result};

pub use pallet::*;
use sp_runtime::traits::AccountIdConversion;
use thea_primitives::{AuthorityIndex, ConsensusLog, ValidatorSet, THEA_ENGINE_ID};


#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        pallet_prelude::*,
        sp_runtime::traits::{AtLeast32BitUnsigned, Saturating},
    };
    use frame_system::pallet_prelude::*;
    use orml_traits::{currency::MultiCurrencyExtended, MultiCurrency};
    use polkadex_primitives::assets::AssetId;

    use thea_primitives::{
        address::RecipientAddress, types::Network, InherentError, InherentType, INHERENT_IDENTIFIER,
    };

    use crate::types::{FeeStructure, WithdrawalBatch, WithdrawalRequest};

    use super::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        /// Authority identifier type
        type AuthorityId: Member
            + Parameter
            + RuntimeAppPublic
            + Default
            + MaybeSerializeDeserialize;
        /// Balance Type
        type Balance: Parameter
            + Member
            + AtLeast32BitUnsigned
            + Default
            + Copy
            + MaybeSerializeDeserialize;
        /// Module that handles tokens
        type Currency: MultiCurrencyExtended<
            Self::AccountId,
            CurrencyId = AssetId,
            Balance = Self::Balance,
        >;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    /// The current authorities set
    #[pallet::storage]
    #[pallet::getter(fn authorities)]
    pub(super) type Authorities<T: Config> = StorageValue<_, Vec<T::AuthorityId>, ValueQuery>;

    /// The Verified CID
    #[pallet::storage]
    #[pallet::getter(fn authorities)]
    pub(super) type VerifiedCID<T: Config> = StorageValue<_, T::Hash, ValueQuery>;

    /// Authorities set scheduled to be used with the next session
    #[pallet::storage]
    #[pallet::getter(fn next_authorities)]
    pub(super) type NextAuthorities<T: Config> = StorageValue<_, Vec<T::AuthorityId>, ValueQuery>;

    /// The current validator set id
    #[pallet::storage]
    #[pallet::getter(fn validator_set_id)]
    pub(super) type ValidatorSetId<T: Config> =
    StorageValue<_, thea_primitives::ValidatorSetId, ValueQuery>;

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub authorities: Vec<T::AuthorityId>,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                authorities: Vec::new(),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            Pallet::<T>::initialize_authorities(&self.authorities);
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {

        #[pallet::weight((10_000, DispatchClass::Mandatory))]
        pub fn set_new_ipfs_cid(
            origin: OriginFor<T>,
            cid: T::Hash,
        ) -> DispatchResult {
            ensure_none(origin)?;

            // TODO: Add some checks

            <VerifiedCID<T>>::put(cid);
            Self::deposit_event(Event::CIDSet(cid));
            Ok(())
        }
    }

    #[pallet::inherent]
    impl<T: Config> ProvideInherent for Pallet<T> {
        type Call = Call<T>;
        type Error = InherentError;
        const INHERENT_IDENTIFIER: InherentIdentifier = INHERENT_IDENTIFIER;

        fn create_inherent(data: &InherentData) -> Option<Self::Call> {
            let inherent_data = data
                .get_data::<InherentType>(&INHERENT_IDENTIFIER)
                .expect("Thea inherent data not correctly encoded")
                .expect("Thea inherent data must be provided");
            let uncompressed_public_key = (*inherent_data).to_vec();

            Some(Call::set_new_ipfs_cid(uncompressed_public_key))
        }

        fn check_inherent(
            call: &Self::Call,
            data: &InherentData,
        ) -> result::Result<(), Self::Error> {
            let imported_public_key: Vec<u8> = match call {
                Call::set_new_ipfs_cid(ref t) => t.to_vec(),
                _ => return Err(InherentError::WrongInherentCall),
            };

            let data = data
                .get_data::<InherentType>(&INHERENT_IDENTIFIER)
                .expect("Thea inherent data not correctly encoded")
                .expect("Thea inherent data must be provided");

            let local_uncompressed_public_key = (*data).to_vec();

            if local_uncompressed_public_key != imported_public_key {
                return Err(InherentError::InvalidPublicKey(imported_public_key));
            }

            Ok(())
        }

        fn is_inherent(call: &Self::Call) -> bool {
            matches!(call, Call::set_new_ipfs_cid(_))
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    #[pallet::metadata(T::AccountId = "AccountId", T::Balance = "Balance")]
    pub enum Event<T: Config> {
        /// Thea Flag changed to \[status\]
        TheaFlagChanged(bool),
        CanSignFlagChanged(bool),
        SharedPublicKeyIsNotAvailable,
        NewIPFSCIDSet(T::Hash),
        AssetWithdrawInitiated(AssetId, T::AccountId, T::Balance, RecipientAddress),
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Withdrawal not supported for given asset id
        UnSupportedNetwork,
        /// Current Batch is full
        WithdrawalBatchFull,
        /// Amount less than minimum withdrawal amount
        LowAmount,
        /// Not enough free balance to withdraw and pay for fees
        NotEnoughBalance,
    }
}

impl<T: Config> Pallet<T> {
    /// Provides the thea account
    pub fn thea_account() -> T::AccountId {
        PalletId(*b"thea/acc").into_account()
    }

    /// Return the current active THEA validator set.
    pub fn validator_set() -> ValidatorSet<T::AuthorityId> {
        ValidatorSet::<T::AuthorityId> {
            validators: Self::authorities(),
            id: Self::validator_set_id(),
        }
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

        // assert!(
        //     <Authorities<T>>::get().is_empty(),
        //     "Authorities are already initialized!"
        // );

        <Authorities<T>>::put(authorities);
        <ValidatorSetId<T>>::put(0);
        // Like `pallet_session`, initialize the next validator set as well.
        <NextAuthorities<T>>::put(authorities);
    }

    fn initialize_can_start(flag: &bool) {
        <TheaFlag<T>>::put(flag);
    }

    fn initialize_can_sign(flag: &bool) {
        <CanSignFlag<T>>::put(flag);
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
