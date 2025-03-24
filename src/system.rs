use core::ops::AddAssign;
use num::traits::{One, Zero};
use std::collections::BTreeMap;

/// Trait de configuração para o System Pallet.
pub trait Config {
    type AccountId: Ord + Clone;
    type BlockNumber: Copy + Zero + One + AddAssign + PartialEq;
    type Nonce: Copy + Zero + One;
}

/// Pallet do Sistema, responsável pelo número de bloco e nonces dos usuários.
#[derive(Debug)]
pub struct Pallet<T: Config> {
    block_number: T::BlockNumber,
    nonce: BTreeMap<T::AccountId, T::Nonce>,
}

impl<T: Config> Pallet<T> {
    /// Cria uma nova instância do System Pallet.
    pub fn new() -> Self {
        Self {
            block_number: T::BlockNumber::zero(),
            nonce: BTreeMap::new(),
        }
    }

    /// Retorna o número atual do bloco.
    pub fn block_number(&self) -> T::BlockNumber {
        self.block_number
    }

    /// Incrementa o número do bloco.
    pub fn inc_block_number(&mut self) {
        self.block_number += T::BlockNumber::one();
    }

    /// Incrementa o nonce de uma conta.
    pub fn inc_nonce(&mut self, who: &T::AccountId) {
        let count = self.nonce.get(who).copied().unwrap_or_else(T::Nonce::zero);
        self.nonce.insert(who.clone(), count + T::Nonce::one());
    }

    /// Retorna o nonce de uma conta.
    pub fn get_nonce(&self, who: &T::AccountId) -> T::Nonce {
        self.nonce.get(who).copied().unwrap_or_else(T::Nonce::zero)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::string::String;

    struct TestConfig;

    impl Config for TestConfig {
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    #[test]
    fn init_system() {
        let mut system = Pallet::<TestConfig>::new();

        let alice = "alice".to_string();

        system.inc_block_number();
        assert_eq!(system.block_number(), 1);

        system.inc_nonce(&alice);
        assert_eq!(system.get_nonce(&alice), 1);
    }
}
