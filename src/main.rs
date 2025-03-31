use resolution::{Balances, Proofs, Runtime};

fn main() {
    let mut saldo = Balances::<Runtime>::default();
    let mut provas = Proofs::<Runtime>::default();

    // Creditar
    saldo.creditar(1, 100);
    saldo.creditar(2, 50);

    // Debitar
    let ok = saldo.debitar(1, 30);
    println!("Débito de 30 da conta 1: {}", ok);
    println!("Saldo da conta 1: {:?}", saldo.consultar_saldo(&1));

    // Provas
    let dado = b"arquivo.pdf".to_vec();
    let registrado = provas.registrar(dado.clone(), 1);
    println!("Prova registrada? {}", registrado);

    if let Some(dono) = provas.verificar(&dado) {
        println!("Dado pertence a conta: {}", dono);
    }
}
