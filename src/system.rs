// src/system.rs

use core::ops::AddAssign;
use num::traits::{One, Zero};
use std::collections::BTreeMap;

pub trait Config {
    type AccountId: Ord + Clone;
    type BlockNumber: Copy + Zero + One + AddAssign + PartialEq;
    type Nonce: Copy + Zero + One;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
    block_number: T::BlockNumber,
    nonce: BTreeMap<T::AccountId, T::Nonce>,
}

impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Self {
            block_number: T::BlockNumber::zero(),
            nonce: BTreeMap::new(),
        }
    }

    pub fn block_number(&self) -> T::BlockNumber {
        self.block_number
    }

    pub fn inc_block_number(&mut self) {
        self.block_number += T::BlockNumber::one();
    }

    pub fn inc_nonce(&mut self, who: &T::AccountId) {
        let count = self.nonce.get(who).copied().unwrap_or_else(T::Nonce::zero);
        self.nonce.insert(who.clone(), count + T::Nonce::one());
    }
}
