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
use pallet_eth_dispatch::Origin as EthereumOrigin;
benchmarks! {
    mint {
        let origin = T::CallOrigin::successful_origin();
        <Address>::put(H160::repeat_byte(2));
        let sender = H160::from_low_u64_be(60);
        let to : T::AccountId = account("reciever-2", 5, 8);
        let recipient: <T::Lookup as StaticLookup>::Source = T::Lookup::unlookup(to);
        let amount = U256::from(100000000_u128);
        let token = H160::from_low_u64_be(5);
        let call = Call::<T>::mint(token, sender, recipient, amount);
    }: {
        call.dispatch_bypass_filter(origin)?
    }

}
