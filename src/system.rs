use std::collections::BTreeMap;

type AccountId = String;
type BlockNumber = u32;
type Nonce = u32;


#[derive(Debug)]
pub struct Pallet {

    block_number: BlockNumber,
    
    nonce: BTreeMap<AccountId, Nonce>,
}

impl Pallet {
    pub fn new() -> Self {
        Self {
            block_number: 0,
            nonce: BTreeMap::new(),
        }
    }

    pub fn block_number(&self) -> BlockNumber {
        self.block_number
    }

    pub fn inc_block_number(&mut self) {
        self.block_number += 1;
    }

    pub fn inc_nonce(&mut self, who: &AccountId) {
        let current_nonce = self.nonce.get(who).copied().unwrap_or(0);
        self.nonce.insert(who.clone(), current_nonce + 1);
    }

    pub fn get_nonce(&self, who: &AccountId) -> Nonce {
        *self.nonce.get(who).unwrap_or(&0)
    }
}
