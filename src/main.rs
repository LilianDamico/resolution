mod balances;
mod system;
mod support;
mod proof_of_existence;


use balances::{Call as BalancesCall, Pallet as BalancesPallet};
use support::{Block, Dispatch, DispatchResult, Extrinsic, Header};
use system::Pallet as SystemPallet;

// Todas as chamadas expostas pelo runtime.
#[derive(Debug)]
pub enum RuntimeCall {
    Balances(BalancesCall<Runtime>),
}

// Estrutura principal da runtime.
#[derive(Debug)]
pub struct Runtime {
    pub system: SystemPallet<Self>,
    pub balances: BalancesPallet<Self>,
}

// Implementações de configuração dos pallets
impl system::Config for Runtime {
    type AccountId = String;
    type BlockNumber = u32;
    type Nonce = u32;
}

impl balances::Config for Runtime {
    type Balance = u128;
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            system: SystemPallet::new(),
            balances: BalancesPallet::new(),
        }
    }

    pub fn execute_block(
        &mut self,
        block: Block<Header<u32>, Extrinsic<String, RuntimeCall>>,
    ) -> DispatchResult {
        self.system.inc_block_number();

        if block.header.block_number != self.system.block_number() {
            return Err("Block number mismatch");
        }

        for (i, Extrinsic { caller, call }) in block.extrinsics.into_iter().enumerate() {
            self.system.inc_nonce(&caller);

            let result = self.dispatch(caller.clone(), call);
            if let Err(e) = result {
                eprintln!(
                    "Extrinsic Error\n\tBlock Number: {}\n\tExtrinsic Index: {}\n\tError: {}",
                    block.header.block_number,
                    i,
                    e
                );
            }
        }

        Ok(())
    }
}

// Delegação de dispatch do runtime para os pallets.
impl Dispatch for Runtime {
    type Caller = String;
    type Call = RuntimeCall;

    fn dispatch(
        &mut self,
        caller: Self::Caller,
        runtime_call: Self::Call,
    ) -> DispatchResult {
        match runtime_call {
            RuntimeCall::Balances(call) => {
                self.balances.dispatch(caller, call)?;
            }
        }
        Ok(())
    }
}

fn main() {
    let mut runtime = Runtime::new();

    let alice = String::from("alice");
    let bob = String::from("bob");
    let charlie = String::from("charlie");

    runtime.balances.set_balance(&alice, 100);

    let block = Block {
        header: Header { block_number: 1 },
        extrinsics: vec![
            Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::Balances(BalancesCall::Transfer {
                    to: bob.clone(),
                    amount: 20,
                }),
            },
            Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::Balances(BalancesCall::Transfer {
                    to: charlie.clone(),
                    amount: 20,
                }),
            },
        ],
    };

    runtime.execute_block(block).expect("Invalid block");

    println!("{:#?}", runtime);
}
