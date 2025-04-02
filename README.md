# 🧠 Resolution - Máquina de Estados em Rust

Este projeto é uma implementação educacional de uma **máquina de estados** em Rust, inspirada na arquitetura de runtime do [Polkadot SDK (Substrate)](https://substrate.dev). O objetivo é aprender os fundamentos de sistemas modulares e extensíveis usando **traits**, **enums**, **macros** e **despacho dinâmico de chamadas**.

---

## 📦 Estrutura do Projeto

```
resolution/
├── Cargo.toml
├── macros/                  # crate auxiliar para macros customizadas
│   ├── src/
│   │   ├── lib.rs
│   │   ├── call/
│   │   │   ├── mod.rs
│   │   │   ├── parse.rs
│   │   │   └── expand.rs
│   │   └── runtime/
│   │       ├── mod.rs
│   │       ├── parse.rs
│   │       └── expand.rs
├── src/
│   ├── balances.rs          # Pallet de saldo (Balance Pallet)
│   ├── proof_of_existence.rs # Pallet de prova de existência (POE Pallet)
│   ├── runtime.rs           # Runtime principal que agrega os pallets
│   ├── types.rs             # Tipos comuns (AccountId, Block, etc.)
│   ├── system.rs            # Pallet de sistema base (nonce, block number)
│   ├── support.rs           # Traits e helpers para Dispatch e Pallets
│   └── main.rs              # Execução e simulação de chamadas
```

---

## ⚙️ Objetivo do Projeto

Este projeto tem como propósito:

- ✅ Compreender **arquiteturas de runtime modulares** como Substrate.
- ✅ Criar **pallets customizados** que implementam funcionalidades isoladas.
- ✅ Simular chamadas entre pallets via `dispatch`.
- ✅ Usar **macros do Rust** para reduzir repetição de código.
- ✅ Preparar para usar o Polkadot SDK com confiança.

---

## 🛠️ Pallets Criados

### 📁 `balances.rs`
Um pallet responsável por:
- Armazenar saldos de contas.
- Permitir transferência de tokens entre contas.
- Registrar os dados em uma `BTreeMap`.

### 📁 `proof_of_existence.rs`
Um pallet responsável por:
- Registrar a prova de um dado (`Vec<u8>`) como autoria do remetente.
- Verificar a existência e o dono da prova.

### 📁 `system.rs`
Responsável por:
- Controlar o `nonce` de cada conta.
- Gerenciar o número do bloco.
- Emitir eventos ou metadados do sistema.

---

## 🧠 Uso das Macros

### Macros implementadas:

- `#[macros::call]`: Gera automaticamente o enum `Call` para o pallet.
- `#[macros::runtime]`: Gera a enum `RuntimeCall` e implementa o `dispatch`.

Essas macros leem a estrutura dos pallets e do runtime e geram automaticamente código como:

```rust
pub enum RuntimeCall {
  Balances(balances::Call),
  ProofOfExistence(proof_of_existence::Call),
}
```

---

## ▶️ Como Rodar

1. Clone o repositório:
```bash
git clone https://github.com/LilianDamico/resolution.git
cd resolution
```

2. Compile e execute:
```bash
cargo run
```

3. Veja o `main.rs` para simulações de chamadas nos pallets!

---

## 📚 Aprendizados Esperados

- Domínio de `traits`, `generics` e `enums` em Rust.
- Uso avançado de macros (`proc-macro-attribute`).
- Criação e despacho de chamadas personalizadas.
- Organização modular de código e crates.

---

## 🚀 Próximos Passos

- [ ] Adicionar mais pallets (Governança, Staking, etc).
- [ ] Serialização com `serde`.
- [ ] Interface CLI ou Web para interagir com runtime.

---

## 🧑‍💻 Créditos

Este projeto segue os passos do tutorial [w3b3d3v/rust-state-machine](https://github.com/w3b3d3v/rust-state-machine) com adaptação e explicações detalhadas para fins educacionais.

---

## Licença

MIT © 2025 Lílian Maria Damico Fonseca
