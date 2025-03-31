use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct AccountInfo {
    pub nonce: u64,
    pub block_number: u64,
}

impl AccountInfo {
    pub fn new() -> Self {
        AccountInfo {
            nonce: 0,
            block_number: 0,
        }
    }
}

#[derive(Debug)]
pub struct System {
    pub accounts: HashMap<String, AccountInfo>,
    pub current_block: u64,
}

impl System {
    pub fn new() -> Self {
        System {
            accounts: HashMap::new(),
            current_block: 0,
        }
    }

    pub fn register_account(&mut self, account_id: String) {
        self.accounts
            .entry(account_id)
            .or_insert(AccountInfo::new());
    }

    pub fn increment_block(&mut self) {
        self.current_block += 1;
    }

    pub fn increment_nonce(&mut self, account_id: &str) -> Result<(), &'static str> {
        if let Some(account) = self.accounts.get_mut(account_id) {
            account.nonce += 1;
            Ok(())
        } else {
            Err("Conta não encontrada")
        }
    }

    pub fn get_account_info(&self, account_id: &str) -> Option<&AccountInfo> {
        self.accounts.get(account_id)
    }
}
