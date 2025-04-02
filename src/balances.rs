// src/balances.rs

use macros::call;

/// Trait de configuração para o pallet Balances
pub trait Config {
    type AccountId: Ord + Clone;
    type Balance: Default + std::ops::AddAssign + std::cmp::PartialOrd + std::ops::SubAssign + Copy;
}

#[derive(Default)]
pub struct Balances<T: Config> {
    pub contas: std::collections::BTreeMap<T::AccountId, T::Balance>,
}

// Macro que gera o enum Call e a lógica de Dispatch
#[call]
impl<T: Config> Balances<T> {
    pub fn transfer(
        &mut self,
        caller: T::AccountId,
        to: T::AccountId,
        amount: T::Balance,
    ) -> Result<(), &'static str> {
        let caller_balance = self.contas.entry(caller.clone()).or_default();
        if *caller_balance < amount {
            return Err("Saldo insuficiente");
        }

        *caller_balance -= amount;
        let to_balance = self.contas.entry(to).or_default();
        *to_balance += amount;

        Ok(())
    }
}
