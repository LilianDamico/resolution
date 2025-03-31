pub trait Config {
    type AccountId: Ord;
    type Balance: Default + std::ops::AddAssign + std::ops::SubAssign + PartialOrd;
}

#[derive(Default)]
pub struct Balances<T: Config> {
    pub contas: std::collections::BTreeMap<T::AccountId, T::Balance>,
}

impl<T: Config> Balances<T>
where
    T::AccountId: Ord,
{
    pub fn creditar(&mut self, quem: T::AccountId, valor: T::Balance) {
        let saldo = self.contas.entry(quem).or_default();
        *saldo += valor;
    }

    pub fn debitar(&mut self, quem: T::AccountId, valor: T::Balance) -> bool
    where
        T::Balance: PartialOrd,
    {
        if let Some(saldo) = self.contas.get_mut(&quem) {
            if *saldo >= valor {
                *saldo -= valor;
                return true;
            }
        }
        false
    }

    pub fn consultar_saldo(&self, quem: &T::AccountId) -> Option<&T::Balance> {
        self.contas.get(quem)
    }
}
