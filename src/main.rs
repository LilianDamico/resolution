mod balances;
mod system;

// Tipos concretos para esta máquina de estado simples.
mod types {
	pub type AccountId = String;
	pub type Balance = u128;
	pub type BlockNumber = u32;
	pub type Nonce = u32;
}

use types::*;

#[derive(Debug)]
pub struct Runtime {
	system: system::Pallet<Self>,
	balances: balances::Pallet<Self>,
}

// Implementando o trait Config do módulo System
impl system::Config for Runtime {
	type AccountId = AccountId;
	type BlockNumber = BlockNumber;
	type Nonce = Nonce;
}

// Implementando o trait Config do módulo Balances
impl balances::Config for Runtime {
	type Balance = Balance;
}

impl Runtime {
	pub fn new() -> Self {
		Self {
			system: system::Pallet::new(),
			balances: balances::Pallet::new(),
		}
	}
}

fn main() {
	let mut runtime = Runtime::new();

	let alice = "alice".to_string();
	let bob = "bob".to_string();
	let charlie = "charlie".to_string();

	runtime.balances.set_balance(&alice, 100);

	runtime.system.inc_block_number();
	assert_eq!(runtime.system.block_number(), 1);

	runtime.system.inc_nonce(&alice);
	let _ = runtime.balances.transfer(alice.clone(), bob.clone(), 30)
		.map_err(|e| eprintln!("Erro ao transferir: {}", e));

	runtime.system.inc_nonce(&alice);
	let _ = runtime.balances.transfer(alice.clone(), charlie.clone(), 20)
		.map_err(|e| eprintln!("Erro ao transferir: {}", e));

		println!("Nonce final de Alice: {}", runtime.system.get_nonce(&alice));

}
