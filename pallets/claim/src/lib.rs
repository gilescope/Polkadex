#![cfg_attr(not(feature = "std"), no_std)]
use codec::{Decode, Encode};

use frame_support::pallet_prelude::*;
use frame_support::{
	decl_error, decl_event, decl_module, decl_storage, dispatch::DispatchResult, ensure,
	traits::Get, traits::Currency, RuntimeDebug, Parameter
};
#[cfg(feature = "std")]
use serde::{self, Deserialize, Deserializer, Serialize, Serializer};
use sp_io::{crypto::secp256k1_ecdsa_recover, hashing::keccak_256};
use sp_runtime::transaction_validity::{
	InvalidTransaction, TransactionLongevity, TransactionSource, TransactionValidity,
	ValidTransaction,
};
use polkadex_primitives::assets::AssetId;
use orml_traits::{MultiCurrency, MultiCurrencyExtended};
use frame_support::sp_runtime::traits::AtLeast32BitUnsigned;
use frame_system::{ensure_signed,ensure_none,ensure_root};
use sp_std::prelude::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[derive(Clone, Copy, PartialEq, Eq, Encode, Decode, Default, RuntimeDebug)]
pub struct EthereumAddress([u8; 20]);

#[cfg(feature = "std")]
impl Serialize for EthereumAddress {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
	{
		let hex: String = rustc_hex::ToHex::to_hex(&self.0[..]);
		serializer.serialize_str(&format!("0x{}", hex))
	}
}

#[cfg(feature = "std")]
impl<'de> Deserialize<'de> for EthereumAddress {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
	{
		let base_string = String::deserialize(deserializer)?;
		let offset = if base_string.starts_with("0x") { 2 } else { 0 };
		let s = &base_string[offset..];
		if s.len() != 40 {
			Err(serde::de::Error::custom(
				"Bad length of Ethereum address (should be 42 including '0x')",
			))?;
		}
		let raw: Vec<u8> = rustc_hex::FromHex::from_hex(s)
			.map_err(|e| serde::de::Error::custom(format!("{:?}", e)))?;
		let mut r = Self::default();
		r.0.copy_from_slice(&raw);
		Ok(r)
	}
}

#[derive(Encode, Decode, Clone)]
pub struct EcdsaSignature(pub [u8; 65]);

impl PartialEq for EcdsaSignature {
	fn eq(&self, other: &Self) -> bool {
		&self.0[..] == &other.0[..]
	}
}

impl sp_std::fmt::Debug for EcdsaSignature {
	fn fmt(&self, f: &mut sp_std::fmt::Formatter<'_>) -> sp_std::fmt::Result {
		write!(f, "EcdsaSignature({:?})", &self.0[..])
	}
}

#[derive(Clone, Copy, PartialEq, Eq, Encode, Decode, Default, RuntimeDebug)]
pub struct EthereumTxHash([u8; 32]);

#[cfg(feature = "std")]
impl Serialize for EthereumTxHash {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
	{
		let hex: String = rustc_hex::ToHex::to_hex(&self.0[..]);
		serializer.serialize_str(&format!("0x{}", hex))
	}
}

#[cfg(feature = "std")]
impl<'de> Deserialize<'de> for EthereumTxHash {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
	{
		let base_string = String::deserialize(deserializer)?;
		let offset = if base_string.starts_with("0x") { 2 } else { 0 };
		let s = &base_string[offset..];
		if s.len() != 64 {
			Err(serde::de::Error::custom(
				"Bad length of Ethereum tx hash (should be 66 including '0x')",
			))?;
		}
		let raw: Vec<u8> = rustc_hex::FromHex::from_hex(s)
			.map_err(|e| serde::de::Error::custom(format!("{:?}", e)))?;
		let mut r = Self::default();
		r.0.copy_from_slice(&raw);
		Ok(r)
	}
}

pub trait Config: frame_system::Config + orml_tokens::Config {
	/// Because this pallet emits events, it depends on the runtime's definition of an event.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;

	type Currency: MultiCurrencyExtended<
		Self::AccountId,
		CurrencyId = AssetId,
		Balance = Self::Balance,
	>;
}

decl_event! {
    pub enum Event<T>
    where
		AccountId = <T as frame_system::Config>::AccountId,
	 <T as orml_tokens::Config>::Balance
    {
        /// Event emitted when a transaction has been stored.
		ERC20TransactionStored(AccountId, EthereumTxHash, EthereumAddress, Balance),
		/// Event emitted when a transaction has been claimed.
		ERC20TokenClaimed(AccountId, EthereumTxHash, Balance),
		/// Event emitted when the relayer has been changed.
		RelayerChanged(AccountId),
    }
}

decl_error! {
    pub enum Error for Module<T: Config> {
        /// The transaction signature is invalid.
		InvalidSignature,
		/// The signer is not claim transaction sender.
		InvalidSigner,
		/// The transaction hash doesn't exist
		TxHashNotFound,
		/// The transaction hash already exist
		TxHashAlreadyExist,
		/// The transaction has been claimed
		TxAlreadyClaimed,
		/// The transaction height less than end height
		LessThanEndHeight,
		/// The relayer has not been changed
		RelayerNotChanged,
		/// The caller is not relayer
		CallerNotRelayer,
		/// The module doesn't set relyer
		NoRelayer,
    }
}

decl_storage! {
    trait Store for Module<T: Config> as PolkadexClaim {
        pub EndHeight get(fn end_height): u64;
        pub BurnedTransactions get(fn burned_transactions): map hasher(opaque_blake2_128) EthereumTxHash => (EthereumAddress, T::Balance);
        pub ClaimState get(fn claim_state): map hasher(opaque_blake2_128) EthereumTxHash => bool;
        pub Relayer get(fn relayer): T::AccountId;
    }
}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        type Error = Error<T>;

        fn deposit_event() = default;

        #[weight=0 + T::DbWeight::get().reads_writes(1,1)]
		pub fn change_relayer(
			origin,
			new_relayer: T::AccountId,
		) -> DispatchResult {
			ensure_root(origin)?;

			if Relayer::<T>::exists() {
				let old_relayer = Relayer::<T>::get();
				ensure!(
					Some(&new_relayer) != Some(&old_relayer),
					Error::<T>::RelayerNotChanged
				);
			}

			Relayer::<T>::put(&new_relayer);
			Self::deposit_event(RawEvent::RelayerChanged(new_relayer));
			Ok(())
		}

		#[weight=0 + T::DbWeight::get().reads_writes(1,1)]
		pub fn store_erc20_burned_transactions(
			origin,
			height: u64,
			claims: Vec<(EthereumTxHash, EthereumAddress,T::Balance)>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(Relayer::<T>::exists(), Error::<T>::NoRelayer);
			let relayer = Relayer::<T>::get();
			ensure!(
				Some(&who) == Some(&relayer),
				Error::<T>::CallerNotRelayer
			);
			// check first
			for (eth_tx_hash, _, _) in claims.iter() {
				ensure!(
					!BurnedTransactions::<T>::contains_key(&eth_tx_hash),
					Error::<T>::TxHashAlreadyExist
				);
			}
			for (eth_tx_hash, eth_address, erc20_amount) in claims.iter() {
				BurnedTransactions::<T>::insert(
					&eth_tx_hash,
					(eth_address.clone(), erc20_amount.clone()),
				);
				ClaimState::insert(&eth_tx_hash, false);
				Self::deposit_event(RawEvent::ERC20TransactionStored(
					who.clone(),
					*eth_tx_hash,
					*eth_address,
					*erc20_amount,
				));
			}
			let end_height = EndHeight::get();
			if height > end_height {
				EndHeight::put(height);
			}

			Ok(())
		}

		#[weight=0 + T::DbWeight::get().reads_writes(1,1)]
		pub fn claim_erc20_token(
			origin,
			account: T::AccountId,
			eth_tx_hash: EthereumTxHash,
			eth_signature: EcdsaSignature,
		) -> DispatchResult {
			let _ = ensure_none(origin)?;
			ensure!(
				BurnedTransactions::<T>::contains_key(&eth_tx_hash),
				Error::<T>::TxHashNotFound
			);
			ensure!(
				ClaimState::get(&eth_tx_hash),
				Error::<T>::TxAlreadyClaimed
			);
			let address = Encode::encode(&account);
			let signer = Self::eth_recover(&eth_signature, &address, &eth_tx_hash.0)
				.ok_or(Error::<T>::InvalidSignature)?;
			let tx = BurnedTransactions::<T>::get(&eth_tx_hash);
			ensure!(signer == tx.0, Error::<T>::InvalidSigner);
			ClaimState::insert(&eth_tx_hash, true);
			// mint coins
			// let imbalance = T::Currency::deposit_creating(&account, tx.1);
			// drop(imbalance);
            T::Currency::deposit(AssetId::POLKADEX, &account, tx.1)?;
			Self::deposit_event(RawEvent::ERC20TokenClaimed(account, eth_tx_hash, tx.1));

			Ok(())
		}
    }
}

impl<T: Config> Module<T> {
	// Constructs the message that Ethereum RPC's `personal_sign` and `eth_sign` would sign.
	fn ethereum_signable_message(what: &[u8], extra: &[u8]) -> Vec<u8> {
		let mut l = what.len() + extra.len();
		let mut rev = Vec::new();
		while l > 0 {
			rev.push(b'0' + (l % 10) as u8);
			l /= 10;
		}
		let mut v = b"\x19Ethereum Signed Message:\n".to_vec();
		v.extend(rev.into_iter().rev());
		v.extend_from_slice(what);
		v.extend_from_slice(extra);
		v
	}

	// Attempts to recover the Ethereum address from a message signature signed by using
	// the Ethereum RPC's `personal_sign` and `eth_sign`.
	fn eth_recover(s: &EcdsaSignature, what: &[u8], extra: &[u8]) -> Option<EthereumAddress> {
		let msg = keccak_256(&Self::ethereum_signable_message(what, extra));
		let mut res = EthereumAddress::default();
		res.0
			.copy_from_slice(&keccak_256(&secp256k1_ecdsa_recover(&s.0, &msg).ok()?[..])[12..]);
		Some(res)
	}
}

impl<T: Config> frame_support::unsigned::ValidateUnsigned for Module<T> {
	type Call = Call<T>;

	fn validate_unsigned(_source: TransactionSource, call: &Self::Call) -> TransactionValidity {
		const PRIORITY: u64 = 100;

		let (maybe_signer, tx_hash) = match call {
			Call::claim_erc20_token(account, eth_tx_hash, eth_signature) => {
				let address = Encode::encode(&account);
				(
					Self::eth_recover(&eth_signature, &address, &eth_tx_hash.0),
					eth_tx_hash,
				)
			}
			_ => return Err(InvalidTransaction::Call.into()),
		};

		let e = InvalidTransaction::Custom(ValidityError::TxHashNotFound.into());
		ensure!(<BurnedTransactions<T>>::contains_key(&tx_hash), e);

		let signer = maybe_signer.ok_or(InvalidTransaction::BadProof)?;

		let e = InvalidTransaction::Custom(ValidityError::InvalidSigner.into());
		let stored_tx = <BurnedTransactions<T>>::get(&tx_hash);
		let stored_signer = stored_tx.0;
		ensure!(signer == stored_signer, e);

		let e = InvalidTransaction::Custom(ValidityError::TxAlreadyClaimed.into());
		ensure!(!<ClaimState>::get(&tx_hash), e);

		Ok(ValidTransaction {
			priority: PRIORITY,
			requires: vec![],
			provides: vec![("claims", signer).encode()],
			longevity: TransactionLongevity::max_value(),
			propagate: true,
		})
	}
}

#[repr(u8)]
pub enum ValidityError {
	/// The transaction signature is invalid.
	InvalidSignature = 0,
	/// The transaction hash doesn't exist
	TxHashNotFound = 1,
	/// The transaction has been claimed
	TxAlreadyClaimed = 2,
	/// The signer is not claim transaction sender.
	InvalidSigner = 3,
}

impl From<ValidityError> for u8 {
	fn from(err: ValidityError) -> Self {
		err as u8
	}
}