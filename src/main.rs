mod balances;
mod system;
use balances::Pallet as BalancesPallet;
use system::Pallet as SystemPallet;

fn main() {
    println!("Vai que é tua...");

    let mut balance = BalancesPallet::new();
    let mut system = SystemPallet::new();

    balance.set_balance(&"alice".to_string(), 100);
    balance.set_balance(&"bob".to_string(), 50);

    println!("Saldo inicial de Alice: {}", balance.balance("alice"));
    println!("Saldo inicial de Bob: {}", balance.balance("bob"));

    match balance.transfer("alice".to_string(), "bob".to_string(), 30) {
        Ok(_) => println!("Transferência bem-sucedida!"),
        Err(err) => println!("Erro na transferência: {}", err),
    }

    println!("Novo saldo de Alice: {}", balance.balance("alice"));
    println!("Novo saldo de Bob: {}", balance.balance("bob"));

    system.increment_block();
    println!("Número do bloco atualizado: {}", system.get_block_number());

    // Usando o nonce para evitar o aviso
    println!("Nonce inicial: {}", system.get_nonce());
    system.increment_nonce();
    println!("Nonce atualizado: {}", system.get_nonce());
}
