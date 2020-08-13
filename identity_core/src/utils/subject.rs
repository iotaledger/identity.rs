use crate::did::DID;

use serde::{Deserialize, Serialize};

use std::{
    hash::{Hash, Hasher},
    str::FromStr,
};

#[derive(Eq, PartialEq, Debug, Default, Serialize, Deserialize)]
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

impl FromStr for Subject {
    type Err = crate::Error;

    fn from_str(s: &str) -> crate::Result<Self> {
        Ok(Subject(DID::parse_from_str(s)?))
    }
}
