use std::collections::BTreeMap;


type AccountId = String;
type Balance = u128;

#[derive(Debug)]
pub struct Pallet {

    balances: BTreeMap<AccountId, Balance>,
}

impl Pallet {
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }

    pub fn set_balance(&mut self, who: &AccountId, amount: Balance) {
        self.balances.insert(who.clone(), amount);
    }

    pub fn balance(&self, who: &str) -> Balance {
        *self.balances.get(who).unwrap_or(&0)
    }

    pub fn transfer(
        &mut self,
        caller: AccountId,
        to: AccountId,
        amount: Balance,
    ) -> Result<(), &'static str> {
        let caller_balance = self.balance(&caller);
        let to_balance = self.balance(&to);

        if caller_balance < amount {
            return Err("Saldo insuficiente para a transferência");
        }

        let new_caller_balance = caller_balance.checked_sub(amount)
            .ok_or("Erro ao calcular novo saldo do remetente")?;
        let new_to_balance = to_balance.checked_add(amount)
            .ok_or("Erro ao calcular novo saldo do destinatário")?;

        self.balances.insert(caller.clone(), new_caller_balance);
        self.balances.insert(to.clone(), new_to_balance);

        Ok(())
    }
}
