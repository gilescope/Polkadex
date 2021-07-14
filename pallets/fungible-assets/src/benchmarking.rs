use sp_std::prelude::*;
use frame_system::Origin;
use polkadex_primitives::assets::AssetId;
use frame_support::sp_runtime::SaturatedConversion;
use frame_support::{
    traits::{Get, EnsureOrigin, UnfilteredDispatchable},
};
use orml_traits::{MultiCurrency};
use crate::{*, Module as PalletModule};
use frame_benchmarking::{benchmarks, account, impl_benchmark_test_suite};
use frame_system::{RawOrigin, EventRecord};
use codec::{Decode, Encode};
use sp_io::hashing::blake2_256;
use sp_core::H160;

pub fn currency<CurrencyId: Decode>(id : u64) -> CurrencyId {
    let new_asset_chainsafe: AssetId = AssetId::CHAINSAFE(H160::from_low_u64_be(id));
    let currency_raw = (new_asset_chainsafe).encode();
    CurrencyId::decode(&mut &currency_raw[..]).unwrap()
}

fn assert_last_event<T: Config>(generic_event: <T as Config>::Event) {
    let events = frame_system::Pallet::<T>::events();
    let system_event: <T as frame_system::Config>::Event = generic_event.into();
    // compare to the last event record
    let EventRecord { event, .. } = &events[events.len() - 1];
    assert_eq!(event, &system_event);
}

fn create_asset_token<T: Config>(max_supply : u128, existenial_deposit : u128 ) {
    let origin : T::AccountId = account("user-0",0,5);
    let max_supply = max_supply.saturated_into();
    let existenial_deposit = existenial_deposit.saturated_into();
    let mint_account : T::AccountId = origin.clone();
    let burn_account : T::AccountId = origin.clone();
    let new_asset: T::CurrencyId = currency(567);
    PalletModule::<T>::create_token(RawOrigin::Signed(origin.clone()).into(),new_asset.clone(),max_supply,Some(mint_account.clone()), Some(burn_account.clone()),existenial_deposit);
}

benchmarks! {
    create_token {
        let origin : T::AccountId = account("user-0",0,5);
        let main = origin.clone();
        let max_supply = (1_000_000_000_000_000_000_000_000 as u128).saturated_into();
        let existenial_deposit = (1_000_000_000_000_000_000_000 as u128).saturated_into();
        let mint_account : T::AccountId = origin.clone();
        let burn_account : T::AccountId = origin.clone();
        let new_asset: T::CurrencyId = currency(20);
    }: _(RawOrigin::Signed(origin),new_asset,max_supply,Some(mint_account), Some(burn_account),existenial_deposit)

    set_vesting_info {
            //Create token
        let origin : T::AccountId = account("user-0",0,5);
        let main = origin.clone();
        let max_supply = (1_000_000_000_000_000_000_000_000 as u128).saturated_into();
        let existenial_deposit = (1_000_000_000_000_000_000_000 as u128).saturated_into();
        let mint_account : T::AccountId = origin.clone();
        let burn_account : T::AccountId = origin.clone();
        let new_asset: T::CurrencyId = currency(10);
        PalletModule::<T>::create_token(RawOrigin::Signed(origin.clone()).into(),new_asset.clone(),max_supply,Some(mint_account), Some(burn_account),existenial_deposit);

            //
        let investor = account("investor",1000,10000);
        let amount = (10_000_000_000_000_000_000_000 as u128).saturated_into();
        let rate = (1_000_000_000_000_000_000 as u128).saturated_into();
    }: _(RawOrigin::Signed(origin),amount,new_asset,rate, investor)


    claim_vesting {
            //Create token
        let origin : T::AccountId = account("user-0",0,5);
        let main = origin.clone();
        let max_supply = (1_000_000_000_000_000_000_000_000 as u128).saturated_into();
        let existenial_deposit = (1_000_000_000_000_000_000_000 as u128).saturated_into();
        let mint_account : T::AccountId = origin.clone();
        let burn_account : T::AccountId = origin.clone();
        let new_asset: T::CurrencyId = currency(60);
        PalletModule::<T>::create_token(RawOrigin::Signed(origin.clone()).into(),new_asset.clone(),max_supply,Some(mint_account), Some(burn_account),existenial_deposit);

            //
        let investor : T::AccountId = account("investor",1000,10000);
        let amount = (10_000_000_000_000_000_000_000 as u128).saturated_into();
        let rate = (1_000_000_000_000_000_000 as u128).saturated_into();
        PalletModule::<T>::set_vesting_info(RawOrigin::Signed(origin).into(),amount,new_asset.clone(),rate, investor.clone());

        let current_block_no = <system::Module<T>>::block_number();
        let vesting_info : VestingInfo<T> = VestingInfo::from(amount, rate, current_block_no);

        let identifier = (&current_block_no, &vesting_info).using_encoded(T::Hashing::hash);

    }: _(RawOrigin::Signed(investor),identifier, new_asset)

    mint_fungible {
            //Create token
        let origin : T::AccountId = account("user-0",0,5);
        let main = origin.clone();
        let max_supply = (1_000_000_000_000_000_000_000_000 as u128).saturated_into();
        let existenial_deposit = (1_000_000_000_000_000_000_000 as u128).saturated_into();
        let mint_account : T::AccountId = origin.clone();
        let burn_account : T::AccountId = origin.clone();
        let new_asset: T::CurrencyId = currency(15);
        PalletModule::<T>::create_token(RawOrigin::Signed(origin.clone()).into(),new_asset.clone(),max_supply,Some(mint_account.clone()), Some(burn_account),existenial_deposit);

            //
        let to : T::AccountId = account("investor",1000,10000);
        let amount = (10_000_000_000_000_000_000_000 as u128).saturated_into();
    }: _(RawOrigin::Signed(mint_account),to, new_asset, amount)

    burn_fungible {
            //Create token
        let origin : T::AccountId = account("user-0",0,5);
        let main = origin.clone();
        let max_supply = (1_000_000_000_000_000_000_000_u128 * 1000_u128).saturated_into();
        let existenial_deposit = (1_000_000_000_000_000_000_000 as u128).saturated_into();
        let mint_account : T::AccountId = origin.clone();
        let burn_account : T::AccountId = origin.clone();
        let new_asset: T::CurrencyId = currency(156);
        PalletModule::<T>::create_token(RawOrigin::Signed(origin.clone()).into(),new_asset.clone(),max_supply,Some(mint_account.clone()), Some(burn_account.clone()),existenial_deposit);

    }: {
        //burns all tokens
        for _ in 0..999 {
            let amount = (1_000_000_000_000_000_000_000 as u128).saturated_into();
            PalletModule::<T>::burn_fungible(RawOrigin::Signed(burn_account.clone()).into(), new_asset, amount);
        }
    }

    set_metadata_fungible {
            //Create token
        let origin : T::AccountId = account("user-0",0,5);
        let main = origin.clone();
        let max_supply = (1_000_000_000_000_000_000_000_000 as u128).saturated_into();
        let existenial_deposit = (1_000_000_000_000_000_000_000 as u128).saturated_into();
        let mint_account : T::AccountId = origin.clone();
        let burn_account : T::AccountId = origin.clone();
        let new_asset: T::CurrencyId = currency(15);
        PalletModule::<T>::create_token(RawOrigin::Signed(origin.clone()).into(),new_asset.clone(),max_supply,Some(mint_account.clone()), Some(burn_account),existenial_deposit);

        let metadata = AssetMetadata{
            name : "asset".as_bytes().to_vec(),
            website : "asset.example.com".as_bytes().to_vec(),
            team : "Team A".as_bytes().to_vec(),
        };
    }: _(RawOrigin::Signed(origin),new_asset, metadata)
    verify {
		assert_last_event::<T>(Event::<T>::MetadataAdded(new_asset, main).into());
	}


    attest_token {
        create_asset_token::<T>(1_000_000_000_000_000_000_000,1_000_000_000_000_000_000_000);
        let origin = T::GovernanceOrigin::successful_origin();
        let asset_id : T::CurrencyId = currency(567);
        let call = Call::<T>::attest_token(asset_id.clone());
    }: {
        call.dispatch_bypass_filter(origin)?
    }
    verify {
		assert_last_event::<T>(Event::<T>::TokenVerified(asset_id).into());
	}

    modify_token_deposit_amount {
        let origin = T::GovernanceOrigin::successful_origin();
        let pdx_amount : T::Balance = (1_000_000_000_000_000_000_000 as u128).saturated_into();
        let call = Call::<T>::modify_token_deposit_amount(pdx_amount.clone());
    }:{
        call.dispatch_bypass_filter(origin)?
    }
    verify {
		assert_last_event::<T>(Event::<T>::TokenDepositModified(pdx_amount).into());
	}
}