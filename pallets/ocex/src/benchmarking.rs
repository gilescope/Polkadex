use sp_std::prelude::*;
use frame_system::Origin;
use polkadex_primitives::assets::AssetId;
use frame_support::sp_runtime::SaturatedConversion;
use orml_traits::{MultiCurrency, MultiCurrencyExtended};
use crate::{*, Module as PalletModule};
use frame_benchmarking::{benchmarks, account, impl_benchmark_test_suite};
use frame_system::RawOrigin;

benchmarks! {
    deposit {
        let main : T::AccountId = account("user-0",0,5);
        let to = main.clone();
        let amount = (2000000000000000000 as u128).saturated_into();
        <T as Config>::Currency::deposit(AssetId::POLKADEX,&main,amount);
        PalletModule::<T>::register(RawOrigin::Signed(main.clone()).into(),main.clone());
    }: _(RawOrigin::Signed(main),to,AssetId::POLKADEX, (1000000000000000000 as u128).saturated_into())

    withdraw {
        let origin : T::AccountId = account("user-0",0,5);
        let main = origin.clone();
        let amount = (2000000000000000000 as u128).saturated_into();
        <T as Config>::Currency::deposit(AssetId::POLKADEX,&main,amount);
        PalletModule::<T>::deposit(RawOrigin::Signed(main.clone()).into(),main.clone(),AssetId::POLKADEX, (1000000000000000000 as u128).saturated_into());
    }: _(RawOrigin::Signed(origin),main,AssetId::POLKADEX, (1000000000000000000 as u128).saturated_into())

    register {
        let origin : T::AccountId = account("user-0",0,5);
        let main = origin.clone();
    }: _(RawOrigin::Signed(origin),main)

    release {
        let origin : T::AccountId = account("user-0",0,5);
        let to : T::AccountId = account("user-4",6,7);
        let main = origin.clone();
        let amount = (4000000000000000000 as u128).saturated_into();
        <T as Config>::Currency::deposit(AssetId::POLKADEX,&main,amount);
        PalletModule::<T>::deposit(RawOrigin::Signed(main.clone()).into(),main.clone(),AssetId::POLKADEX, (2000000000000000000 as u128).saturated_into());
        pallet_substratee_registry::EnclaveIndex::<T>::insert(main.clone(), 1_u64);
    }: _(RawOrigin::Signed(origin),AssetId::POLKADEX, (1000000000000000000 as u128).saturated_into(), to)

    add_proxy {
        let main : T::AccountId = account("user-1",6,500);
        let to = main.clone();
        let amount = (2000000000000000000 as u128).saturated_into();
        <T as Config>::Currency::deposit(AssetId::POLKADEX,&main,amount);
        PalletModule::<T>::register(RawOrigin::Signed(main.clone()).into(),main.clone());
        let proxies = vec!["proxy-1","proxy-2","proxy-3","proxy-4","proxy-5","proxy-6","proxy-7","proxy-8","proxy-9"];
        for (i,proxy_name) in proxies.iter().enumerate() {
            let proxy_account : T::AccountId = account(proxy_name, i as u32, 500);
            PalletModule::<T>::add_proxy(RawOrigin::Signed(main.clone()).into(),main.clone(), proxy_account);
        }
        let proxy_account : T::AccountId = account("proxy", 100, 500);
    }: _(RawOrigin::Signed(main), to, proxy_account)

    remove_proxy {
        let main : T::AccountId = account("user-1",6,500);
        let to = main.clone();
        let amount = (2000000000000000000 as u128).saturated_into();
        <T as Config>::Currency::deposit(AssetId::POLKADEX,&main,amount);
        PalletModule::<T>::register(RawOrigin::Signed(main.clone()).into(),main.clone());
        let proxies = vec!["proxy-1","proxy-2","proxy-3","proxy-4","proxy-5","proxy-6","proxy-7","proxy-8","proxy-9"];
        for (i,proxy_name) in proxies.iter().enumerate() {
            let proxy_account : T::AccountId = account(proxy_name, i as u32, 500);
            PalletModule::<T>::add_proxy(RawOrigin::Signed(main.clone()).into(),main.clone(), proxy_account.clone());
            PalletModule::<T>::remove_proxy(RawOrigin::Signed(main.clone()).into(),main.clone(), proxy_account);
        }
        let proxy_account : T::AccountId = account("proxy", 100, 500);
        PalletModule::<T>::add_proxy(RawOrigin::Signed(main.clone()).into(),main.clone(), proxy_account.clone());
    }: _(RawOrigin::Signed(main), to, proxy_account)
}
