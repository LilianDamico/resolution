use crate::balances::Config as BalancesConfig;
use crate::proof_of_existence::Config as ProofConfig;

#[derive(Default)]
pub struct Runtime;

impl BalancesConfig for Runtime {
    type AccountId = u64;
    type Balance = u64; // `u64` já implementa `Default`, `AddAssign`, `PartialOrd`
}

impl ProofConfig for Runtime {
    type AccountId = u64;
    type Event = ();
}
