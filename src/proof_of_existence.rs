pub trait Config {
    type AccountId;
    type Event;
}

#[derive(Default)]
pub struct Proofs<T: Config> {
    pub dados: std::collections::BTreeMap<Vec<u8>, T::AccountId>,
}

impl<T: Config> Proofs<T>
where
    T::AccountId: Ord + Clone,
{
    pub fn registrar(&mut self, dado: Vec<u8>, quem: T::AccountId) -> bool {
        if self.dados.contains_key(&dado) {
            false
        } else {
            self.dados.insert(dado, quem);
            true
        }
    }

    pub fn verificar(&self, dado: &Vec<u8>) -> Option<&T::AccountId> {
        self.dados.get(dado)
    }
}
