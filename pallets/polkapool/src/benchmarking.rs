//! Benchmarking setup for pallet-polkapool

use sp_std::prelude::*;
use frame_system::Origin;
use polkadex_primitives::assets::AssetId;
use frame_support::sp_runtime::SaturatedConversion;
use frame_support::{
    traits::{Get, EnsureOrigin, UnfilteredDispatchable},
};
use orml_traits::{MultiCurrency, MultiCurrencyExtended};
use crate::{*, Module as PalletModule};
use frame_benchmarking::{benchmarks, account, impl_benchmark_test_suite};
use frame_system::RawOrigin;
use sp_core::{H160, U256};


/// Claim feeless transaction bechmarking compiles
/**/



#[allow(unused)]
use crate::Pallet as Template;
benchmarks! {

    claim_feeless_transaction {
           let origin : T::AccountId = account("staker-0", 10, 20);
           let main = origin.clone();
           let amount = (2000000000000000000 as u128).saturated_into();
           <T as Config>::Currency::deposit(AssetId::POLKADEX,&main,amount);
           let stake_amount  = (1000000000000000000 as u128).saturated_into();
           let call: <T as Config>::Call = frame_system::Call::<T>::remark(vec![]).into();
    }: _(RawOrigin::Signed(origin),stake_amount,Box::new(call))

    unstake {
        let current_block_no: T::BlockNumber = <frame_system::Pallet<T>>::block_number();
        let origin : T::AccountId = account("staker-0", 10, 20);
        let staked_amount = StakeInfo {
             staked_amount: 0_u128.saturated_into(),
            unlocking_block: current_block_no.clone(),
        };
        StakedUsers::<T>::insert(origin.clone(),staked_amount);
    }: _(RawOrigin::Signed(origin))

    slash_stake {
        let current_block_no: T::BlockNumber = <frame_system::Pallet<T>>::block_number();
        let origin = T::GovernanceOrigin::successful_origin();
        let user : T::AccountId = account("staker-0", 10, 20);
        let amount = (200000000000000 as u128).saturated_into();
        <T as Config>::Currency::deposit(AssetId::POLKADEX,&user,amount);
        let staked_amount = StakeInfo {
             staked_amount: 100000000000000_u128.saturated_into(),
            unlocking_block: current_block_no.clone(),
        };
        StakedUsers::<T>::insert(user.clone(),staked_amount);
        let call = Call::<T>::slash_stake(user);
    }:{
        //let origin = <T as frame_system::Config>::Origin::from(origin);
        call.dispatch_bypass_filter(origin.into())?
    }
}