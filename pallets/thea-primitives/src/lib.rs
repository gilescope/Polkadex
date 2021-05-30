#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]
// NOTE: needed to silence warnings about generated code in `decl_runtime_apis`
#![allow(clippy::too_many_arguments, clippy::unnecessary_mut_passed)]

use codec::{Codec, Decode, Encode};
use sp_core::H256;
use sp_std::prelude::*;

/// Key type for THEA module.
pub const KEY_TYPE: sp_application_crypto::KeyTypeId = sp_application_crypto::KeyTypeId(*b"thea");

/// THEA application-specific crypto types using ECDSA.
pub mod ecdsa {
    mod app_ecdsa {
        use sp_application_crypto::{app_crypto, ecdsa};
        app_crypto!(ecdsa, crate::KEY_TYPE);
    }

    sp_application_crypto::with_pair! {
        /// A THEA authority keypair using ECDSA as its crypto.
        pub type AuthorityPair = app_ecdsa::Pair;
    }

    /// Identity of a THEA authority using ECDSA as its crypto.
    pub type AuthorityId = app_ecdsa::Public;

    /// Signature for a THEA authority using ECDSA as its crypto.
    pub type AuthoritySignature = app_ecdsa::Signature;
}

/// The `ConsensusEngineId` of THEA.
pub const THEA_ENGINE_ID: sp_runtime::ConsensusEngineId = *b"THEA";

/// Authority set id starts with zero at genesis
pub const GENESIS_AUTHORITY_SET_ID: u64 = 0;

/// A typedef for validator set id.
pub type ValidatorSetId = u64;

/// A set of THEA authorities, a.k.a. validators.
#[derive(Decode, Encode, Debug, PartialEq, Clone)]
pub struct ValidatorSet<AuthorityId> {
    /// Public keys of the validator set elements
    pub validators: Vec<AuthorityId>,
    /// Identifier of the validator set
    pub id: ValidatorSetId,
}

impl<AuthorityId> ValidatorSet<AuthorityId> {
    /// Return an empty validator set with id of 0.
    pub fn empty() -> Self {
        Self {
            validators: Default::default(),
            id: Default::default(),
        }
    }
}

/// The index of an authority.
pub type AuthorityIndex = u32;

/// A consensus log item for THEA.
#[derive(Decode, Encode)]
pub enum ConsensusLog<AuthorityId: Codec> {
    /// The authorities have changed.
    #[codec(index = 1)]
    AuthoritiesChange(ValidatorSet<AuthorityId>),
    /// Disable the authority with given index.
    #[codec(index = 2)]
    OnDisabled(AuthorityIndex),
    // /// MMR root hash.
    // #[codec(index = 3)]
    // MmrRoot(MmrRootHash),
}

sp_api::decl_runtime_apis! {
    /// API necessary for THEA voters.
    pub trait TheaApi<AuthorityId: Codec> {
        /// Return the current active THEA validator set
        fn validator_set() -> ValidatorSet<AuthorityId>;
    }
}
