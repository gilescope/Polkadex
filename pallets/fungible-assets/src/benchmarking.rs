use sp_std::prelude::*;
use frame_system::Origin;
use polkadex_primitives::assets::AssetId;
use frame_support::sp_runtime::SaturatedConversion;
use orml_traits::{MultiCurrency};
use crate::{*, Module as PalletModule};
use frame_benchmarking::{benchmarks, account, impl_benchmark_test_suite};
use frame_system::RawOrigin;
use codec::{Decode, Encode};
use sp_io::hashing::blake2_256;
use sp_core::H160;

pub fn currency<CurrencyId: Decode>() -> CurrencyId {
    let new_asset_chainsafe: AssetId = AssetId::DOT;
    let currency_raw = (new_asset_chainsafe).encode();
    CurrencyId::decode(&mut &currency_raw[..]).unwrap()
}
benchmarks! {
    create_token {
        let origin : T::AccountId = account("user-0",0,5);
        let main = origin.clone();
        let max_supply = (1_000_000_000_000_000_000_000_000 as u128).saturated_into();
        let existenial_deposit = (1_000_000_000_000_000_000_000 as u128).saturated_into();
        let mint_account : T::AccountId = account("mint",1,5);
        let burn_account : T::AccountId = account("burn",2,5);
        let new_asset: T::CurrencyId = currency();
    }: _(RawOrigin::Signed(origin),new_asset,max_supply,Some(mint_account), Some(burn_account),existenial_deposit)

    set_vesting_info {
            //Create token
        let origin : T::AccountId = account("user-0",0,5);
        let main = origin.clone();
        let max_supply = (1_000_000_000_000_000_000_000_000 as u128).saturated_into();
        let existenial_deposit = (1_000_000_000_000_000_000_000 as u128).saturated_into();
        let mint_account : T::AccountId = account("mint",1,5);
        let burn_account : T::AccountId = account("burn",2,5);
        let new_asset: T::CurrencyId = currency();
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
        let mint_account : T::AccountId = account("mint",1,5);
        let burn_account : T::AccountId = account("burn",2,5);
        let new_asset: T::CurrencyId = currency();
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
        let mint_account : T::AccountId = account("mint",1,5);
        let burn_account : T::AccountId = account("burn",2,5);
        let new_asset: T::CurrencyId = currency();
        PalletModule::<T>::create_token(RawOrigin::Signed(origin.clone()).into(),new_asset.clone(),max_supply,Some(mint_account.clone()), Some(burn_account),existenial_deposit);

            //
        let to : T::AccountId = account("investor",1000,10000);
        let amount = (10_000_000_000_000_000_000_000 as u128).saturated_into();
    }: _(RawOrigin::Signed(mint_account),to, new_asset, amount)

    burn_fungible {
            //Create token
        let origin : T::AccountId = account("user-0",0,5);
        let main = origin.clone();
        let max_supply = (1_000_000_000_000_000_000_000_000 as u128).saturated_into();
        let existenial_deposit = (1_000_000_000_000_000_000_000 as u128).saturated_into();
        let mint_account : T::AccountId = account("mint",1,5);
        let burn_account : T::AccountId = account("burn",2,5);
        let new_asset: T::CurrencyId = currency();
        PalletModule::<T>::create_token(RawOrigin::Signed(origin.clone()).into(),new_asset.clone(),max_supply,Some(mint_account.clone()), Some(burn_account.clone()),existenial_deposit);
        let amount = (10_000_000_000_000_000_000_000 as u128).saturated_into();
    }: _(RawOrigin::Signed(burn_account), new_asset, amount)
}