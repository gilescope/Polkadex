use sp_std::prelude::*;
use frame_system::Origin;
use polkadex_primitives::assets::AssetId;
use frame_support::sp_runtime::SaturatedConversion;
use orml_traits::{MultiCurrency, MultiCurrencyExtended};
use crate::{*, Module as PalletModule};
use frame_benchmarking::{benchmarks, account, impl_benchmark_test_suite};
use frame_system::RawOrigin;
use sp_core::H160;
use std::str::FromStr;
benchmarks! {
        create_token {
                let origin : T::AccountId = account("user-0",0,5);
                let main = origin.clone();
                let max_supply = (1_000_000_000_000_000_000_000_000 as u128).saturated_into();
                let existenial_deposit = (1_000_000_000_000_000_000_000 as u128).saturated_into();
                let mint_account : T::AccountId = account("mint",1,5);
                let burn_account : T::AccountId = account("burn",2,5);
                let new_asset: AssetId =  AssetId::CHAINSAFE(H160::from_low_u64_be(24));
        }: _(RawOrigin::Signed(origin),new_asset,max_supply,Some(mint_account), Some(burn_account),existenial_deposit)
}