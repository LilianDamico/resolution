// src/proof_of_existence.rs

use std::collections::BTreeMap;
use crate::support::DispatchResult;

pub trait Config {
    type AccountId: Eq + Clone;
    type Content: Ord + Clone;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
    pub claims: BTreeMap<T::Content, T::AccountId>,
}

impl<T: Config> Default for Pallet<T> {
    fn default() -> Self {
        Self {
            claims: BTreeMap::new(),
        }
    }
}

impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_claim(&self, claim: &T::Content) -> Option<&T::AccountId> {
        self.claims.get(claim)
    }
}

// Macro que gera o enum Call e a lógica de Dispatch
#[macros::call]
impl<T: Config> Pallet<T> {
    pub fn create_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
        if self.claims.contains_key(&claim) {
            return Err("this content is already claimed");
        }
        self.claims.insert(claim, caller);
        Ok(())
    }

    pub fn revoke_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
        let owner = self.get_claim(&claim).ok_or("claim does not exist")?;
        if caller != *owner {
            return Err("this content is owned by someone else");
        }
        self.claims.remove(&claim);
        Ok(())
    }
}
