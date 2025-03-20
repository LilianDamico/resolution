use std::collections::BTreeMap;

/// Este é o System Pallet.
/// Ele gerencia o estado de baixo nível necessário para o blockchain.
pub struct Pallet {
    /// O número atual do bloco.
    block_number: u32,
    /// Um mapa de uma conta para seu nonce.
    nonce: BTreeMap<String, u32>,
}

impl Pallet {
    /// Cria uma nova instância do System Pallet.
    pub fn new() -> Self {
        Self {
            block_number: 0,
            nonce: BTreeMap::new(),
        }
    }

    /// Retorna o número atual do bloco.
    pub fn block_number(&self) -> u32 {
        self.block_number
    }

    /// Incrementa o número do bloco em um.
    pub fn inc_block_number(&mut self) {
        self.block_number += 1;
    }

    pub fn inc_nonce(&mut self, who: &String) {
        let current_nonce = self.nonce.get(who).copied().unwrap_or(0);
        self.nonce.insert(who.clone(), current_nonce + 1);
    }
    pub fn get_nonce(&self, who: &String) -> u32 {
        *self.nonce.get(who).unwrap_or(&0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_system() {
        let mut system = Pallet::new();

        let alice = "alice".to_string();

        system.inc_block_number();
        system.inc_block_number();

        system.inc_nonce(&alice);
        system.inc_nonce(&alice);

        assert_eq!(system.block_number(), 2);

        assert_eq!(system.get_nonce(&alice), 2);
    }
}
