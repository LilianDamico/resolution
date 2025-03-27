use num::traits::{CheckedAdd, CheckedSub, Zero};
use std::collections::BTreeMap;

use crate::support::{Dispatch, DispatchResult};
use crate::system;

/// Trait de configuração para o pallet de saldos.
pub trait Config: system::Config {
	type Balance: Zero + CheckedAdd + CheckedSub + Copy + PartialOrd;
}

/// Estrutura do pallet de saldos.
#[derive(Debug)]
pub struct Pallet<T: Config> {
	balances: BTreeMap<T::AccountId, T::Balance>,
}

impl<T: Config> Pallet<T> {
	pub fn new() -> Self {
		Self {
			balances: BTreeMap::new(),
		}
	}

	pub fn set_balance(&mut self, who: &T::AccountId, amount: T::Balance) {
		self.balances.insert(who.clone(), amount);
	}

	pub fn balance(&self, who: &T::AccountId) -> T::Balance {
		*self.balances.get(who).unwrap_or(&T::Balance::zero())
	}

	pub fn transfer(
		&mut self,
		from: T::AccountId,
		to: T::AccountId,
		amount: T::Balance,
	) -> DispatchResult {
		let from_balance = self.balance(&from);
		let to_balance = self.balance(&to);

		if amount > from_balance {
			return Err("Insufficient Balance");
		}

		let new_from_balance = from_balance.checked_sub(&amount).ok_or("Underflow")?;
		let new_to_balance = to_balance.checked_add(&amount).ok_or("Overflow")?;

		self.set_balance(&from, new_from_balance);
		self.set_balance(&to, new_to_balance);

		Ok(())
	}
}

/// Enum de chamadas possíveis do pallet de Balances.
#[derive(Debug)]
pub enum Call<T: Config> {
	Transfer {
		to: T::AccountId,
		amount: T::Balance,
	},
}

/// Implementação de Dispatch no nível do pallet.
impl<T: Config> Dispatch for Pallet<T> {
	type Caller = T::AccountId;
	type Call = Call<T>;

	fn dispatch(
		&mut self,
		caller: Self::Caller,
		call: Self::Call,
	) -> DispatchResult {
		match call {
			Call::Transfer { to, amount } => self.transfer(caller, to, amount),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	struct TestConfig;

	impl system::Config for TestConfig {
		type AccountId = String;
		type BlockNumber = u32;
		type Nonce = u32;
	}

	impl Config for TestConfig {
		type Balance = u128;
	}

	#[test]
	fn test_transfer() {
		let mut balances = Pallet::<TestConfig>::new();
		let alice = "alice".to_string();
		let bob = "bob".to_string();

		balances.set_balance(&alice, 100);

		let call = Call::<TestConfig>::Transfer {
			to: bob.clone(),
			amount: 30,
		};

		let result = balances.dispatch(alice.clone(), call);
		assert!(result.is_ok());

		assert_eq!(balances.balance(&alice), 70);
		assert_eq!(balances.balance(&bob), 30);
	}
}
