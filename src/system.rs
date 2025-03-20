pub struct Pallet {
    block_number: u64,
    nonce: u32,
}

impl Pallet {
    pub fn new() -> Self {
        Pallet {
            block_number: 0,
            nonce: 0,
        }
    }

    pub fn increment_block(&mut self) {
        self.block_number += 1;
    }

    pub fn get_block_number(&self) -> u64 {
        self.block_number
    }

    /// Nova função para ler o `nonce` e evitar o aviso
    pub fn get_nonce(&self) -> u32 {
        self.nonce
    }

    /// Nova função para incrementar o `nonce`
    pub fn increment_nonce(&mut self) {
        self.nonce += 1;
    }
}
