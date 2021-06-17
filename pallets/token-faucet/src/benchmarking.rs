use sp_std::prelude::*;
use frame_system::Origin;

use crate::{*, Module as PalletModule};
use frame_benchmarking::{benchmarks, account, impl_benchmark_test_suite};
use frame_system::RawOrigin;
benchmarks! {
    credit_account_with_tokens_unsigned {
        let origin = RawOrigin::Root;
        let to = account("me",0,0);
    }: _(RawOrigin::None,to);
}