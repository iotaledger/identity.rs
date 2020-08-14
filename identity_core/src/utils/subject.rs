use crate::did::DID;

use serde::{Deserialize, Serialize};

use std::str::FromStr;

#[derive(Eq, PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Subject(DID);

impl Subject {
    pub fn new(s: String) -> crate::Result<Self> {
        let did = DID::parse_from_str(&s)?;

        Ok(Subject(did))
    }

    pub fn from_did(did: DID) -> crate::Result<Self> {
        Ok(Subject(did))
    }
}

impl FromStr for Subject {
    type Err = crate::Error;

    fn from_str(s: &str) -> crate::Result<Self> {
        Ok(Subject(DID::parse_from_str(s)?))
    }
}
