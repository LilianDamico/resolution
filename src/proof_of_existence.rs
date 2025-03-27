use crate::support::DispatchResult;
use core::fmt::Debug;
use std::collections::BTreeMap;

pub trait Config: crate::system::Config {
	
	type Content: Debug + Ord;
}


#[derive(Debug)]
pub struct Pallet<T: Config> {
	
	claims: BTreeMap<T::Content, T::AccountId>,
}

impl<T: Config> Pallet<T> {
	/// Create a new instance of the Proof of Existence Module.
	pub fn new() -> Self {
		Self { claims: BTreeMap::new() }
	}

	
	pub fn get_claim(&self, claim: &T::Content) -> Option<&T::AccountId> {
		self.claims.get(claim)
	}

	
	pub fn create_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
		if self.claims.contains_key(&claim) {
			return Err("this content is already claimed");
		}
		self.claims.insert(claim, caller);
		Ok(())
	}

	
	pub fn revoke_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
		let owner = self.get_claim(&claim).ok_or("no claim found")?;
		if owner != &caller {
			return Err("not the owner of the claim");
		}
		self.claims.remove(&claim);
		Ok(())
	}
}

#[cfg(test)]
mod test {
	use super::*;

	struct TestConfig;

	impl super::Config for TestConfig {
		type Content = &'static str;
	}

	impl crate::system::Config for TestConfig {
		type AccountId = &'static str;
		type BlockNumber = u32;
		type Nonce = u32;
	}

	#[test]
	fn basic_proof_of_existence() {
		let mut poe = Pallet::<TestConfig>::new();
		let alice = "alice";
		let bob = "bob";
		let claim = "claim_data";

		
		assert_eq!(poe.get_claim(&claim), None);

		
		assert!(poe.create_claim(alice, claim).is_ok());
		assert_eq!(poe.get_claim(&claim), Some(&"alice"));

	
		assert!(poe.create_claim(bob, claim).is_err());

		
		assert!(poe.revoke_claim(bob, claim).is_err());

		
		assert!(poe.revoke_claim(alice, claim).is_ok());
		assert_eq!(poe.get_claim(&claim), None);

		
		assert!(poe.revoke_claim(alice, claim).is_err());
	}
}
