use num::traits::{CheckedAdd, CheckedSub, Zero};
use std::collections::BTreeMap;

/// Trait de configuração para o módulo de saldos.
/// Herdando de `system::Config`, temos acesso ao tipo `AccountId`.
pub trait Config: crate::system::Config {
	type Balance: Zero + CheckedSub + CheckedAdd + Copy + PartialOrd;
}

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
	) -> Result<(), &'static str> {
		let from_balance = self.balance(&from);

		if from_balance < amount {
			return Err("Insufficient balance");
		}

		let to_balance = self.balance(&to);

		let updated_from = from_balance.checked_sub(&amount).ok_or("Underflow")?;
		let updated_to = to_balance.checked_add(&amount).ok_or("Overflow")?;

		self.balances.insert(from, updated_from);
		self.balances.insert(to, updated_to);

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	struct TestConfig;

	impl crate::system::Config for TestConfig {
		type AccountId = String;
		type BlockNumber = u32;
		type Nonce = u32;
	}

	impl Config for TestConfig {
		type Balance = u128;
	}

	#[test]
	fn transfer_works() {
		let mut pallet = Pallet::<TestConfig>::new();

		let alice = "alice".to_string();
		let bob = "bob".to_string();

		pallet.set_balance(&alice, 100);
		assert_eq!(pallet.balance(&alice), 100);
		assert_eq!(pallet.balance(&bob), 0);

		assert!(pallet.transfer(alice.clone(), bob.clone(), 30).is_ok());
		assert_eq!(pallet.balance(&alice), 70);
		assert_eq!(pallet.balance(&bob), 30);
	}
}
