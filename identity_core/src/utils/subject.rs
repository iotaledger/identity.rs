use crate::did::DID;

use std::hash::{Hash, Hasher};

#[derive(Eq, PartialEq, Debug)]
pub struct Subject(DID);

impl Subject {
    pub fn new(s: String) -> crate::Result<Self> {
        let did = DID::parse_from_str(&s)?;

        Ok(Subject(did))
    }
}

impl Hash for Subject {
    fn hash<T>(&self, state: &mut T)
    where
        T: Hasher,
    {
        let s = self.0.to_string();
        s.hash(state);
    }
}
