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

// This is pallet is modified beefy-primitives from Parity Technologies (UK) Ltd.
#![cfg_attr(not(feature = "std"), no_std)]
// NOTE: needed to silence warnings about generated code in `decl_runtime_apis`
#![allow(
    clippy::too_many_arguments,
    clippy::unnecessary_mut_passed,
    clippy::redundant_slicing
)]


use codec::{Codec, Decode, Encode};
use sp_inherents::{InherentIdentifier, IsFatalError};
use sp_std::prelude::*;

use sp_runtime::{
    traits::{IdentifyAccount, Verify},
    MultiSignature,
};

/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = MultiSignature;

/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

/// Key type for IPFS module.
pub const KEY_TYPE: sp_application_crypto::KeyTypeId = sp_application_crypto::KeyTypeId(*b"ipfs");

/// IPFS Inherents
pub const INHERENT_IDENTIFIER: InherentIdentifier = *b"ipfsreqt";
pub type InherentType = Vec<u8>;

/// Errors that can occur while checking the Thea inherent.
#[derive(Encode, sp_runtime::RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Decode, thiserror::Error))]
pub enum InherentError {
    /// This is a fatal-error and will stop block import.
    #[cfg_attr(feature = "std", error("The inserted shared public key is invalid."))]
    InvalidPublicKey(InherentType),
    /// This is a fatal-error and will stop block import.
    #[cfg_attr(feature = "std", error("Wrong Inherent Call in Block"))]
    WrongInherentCall,
}

impl IsFatalError for InherentError {
    fn is_fatal_error(&self) -> bool {
        match self {
            InherentError::InvalidPublicKey(_) => true,
            InherentError::WrongInherentCall => true,
        }
    }
}

impl InherentError {
    /// Try to create an instance ouf of the given identifier and data.
    #[cfg(feature = "std")]
    pub fn try_from(id: &InherentIdentifier, data: &[u8]) -> Option<Self> {
        if id == &INHERENT_IDENTIFIER {
            <InherentError as codec::Decode>::decode(&mut &data[..]).ok()
        } else {
            None
        }
    }
}

/// THEA application-specific crypto types using ECDSA.
mod app {
    use sp_application_crypto::{app_crypto, sr25519};

    app_crypto!(sr25519, crate::KEY_TYPE);
}

sp_application_crypto::with_pair! {
        /// A THEA authority keypair using ECDSA as its crypto.
        pub type AuthorityPair = app::Pair;
}

/// Identity of a THEA authority using ECDSA as its crypto.
pub type AuthorityId = app::Public;

/// Signature for a THEA authority using ECDSA as its crypto.
pub type AuthoritySignature = app::Signature;

/// The `ConsensusEngineId` of THEA.
pub const IPFS_ENGINE_ID: sp_runtime::ConsensusEngineId = *b"IPFS";

/// Authority set id starts with zero at genesis
pub const GENESIS_AUTHORITY_SET_ID: u64 = 0;

/// A typedef for validator set id.
pub type ValidatorSetId = u64;


/// A consensus log item for Offchain IPFS.
#[derive(Decode, Encode)]
pub enum ConsensusLog<AuthorityId: Codec> {
    /// The authorities have changed.
    #[codec(index = 1)]
    AuthoritiesChange(ValidatorSet<AuthorityId>),
    /// Disable the authority with given index.
    #[codec(index = 2)]
    OnDisabled(AuthorityIndex),
}


sp_api::decl_runtime_apis! {
    /// API necessary for THEA voters.
    pub trait OffchainIPFSApi<AuthorityId: Codec> {
        /// Return the current active THEA validator set
        fn validator_set() -> Vec<AuthorityId>;
    }
}
