mod balances;
mod system;


#[derive(Debug)]
pub struct Runtime {
    system: system::Pallet,
    balances: balances::Pallet,
}

impl Runtime {

    fn new() -> Self {
        Self {
            system: system::Pallet::new(),
            balances: balances::Pallet::new(),
        }
    }
}

fn main() {
    let mut runtime = Runtime::new();

    let alice = "alice".to_string();
    let bob = "bob".to_string();
    let charlie = "charlie".to_string();

    runtime.balances.set_balance(&alice, 100);
    println!("Saldo inicial de Alice: {}", runtime.balances.balance("alice"));

    runtime.system.inc_block_number();
    assert_eq!(runtime.system.block_number(), 1);
    println!("Número do bloco: {}", runtime.system.block_number());

    runtime.system.inc_nonce(&alice);
    let _res = runtime.balances.transfer(alice.clone(), bob.clone(), 30).map_err(|e| eprintln!("{}", e));
    println!("Saldo de Alice após transação 1: {}", runtime.balances.balance("alice"));
    println!("Saldo de Bob após transação 1: {}", runtime.balances.balance("bob"));

    runtime.system.inc_nonce(&alice);
    let _res = runtime.balances.transfer(alice.clone(), charlie.clone(), 20).map_err(|e| eprintln!("{}", e));
    println!("Saldo de Alice após transação 2: {}", runtime.balances.balance("alice"));
    println!("Saldo de Charlie após transação 2: {}", runtime.balances.balance("charlie"));

    println!("Nonce final de Alice: {}", runtime.system.get_nonce(&alice));

    println!("{:#?}", runtime);
}
