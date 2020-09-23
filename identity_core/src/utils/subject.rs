use crate::did::DID;

use identity_diff::Diff;
use serde::{Deserialize, Serialize};

use std::{hash::Hash, str::FromStr};

/// A wrapped `DID` type called a subject.  
#[derive(Eq, PartialEq, Debug, Default, Clone, Serialize, Deserialize, Diff, Hash, PartialOrd, Ord)]
#[serde(transparent)]
#[diff(from_into)]
pub struct Subject(DID);

impl Subject {
    /// creates a new `Subject` given a `DID` string with proper format.
    pub fn new(s: String) -> crate::Result<Self> {
        let did = DID::parse_from_str(&s)?;

        Ok(Subject(did))
    }

    /// converts a `DID` into a `Subject`.
    pub fn from_did(did: DID) -> crate::Result<Self> {
        Ok(Subject(did))
    }

    /// retrieves the `DID` from the `Subject`.
    pub fn to_did(&self) -> crate::Result<DID> {
        Ok(self.0.clone())
    }
}

impl FromStr for Subject {
    type Err = crate::Error;

    fn from_str(s: &str) -> crate::Result<Self> {
        Ok(Subject(DID::parse_from_str(s)?))
    }
}

/// Allows type conversion from the `DID` type to the `Subject` type.
impl From<DID> for Subject {
    fn from(did: DID) -> Self {
        Subject::from_did(did).expect("unable to convert Did to Subject")
    }
}

impl From<&str> for Subject {
    fn from(s: &str) -> Self {
        Subject::from_str(s).expect("unable to convert Did to Subject")
    }
}

impl From<String> for Subject {
    fn from(s: String) -> Self {
        Subject::from_str(&s).expect("Unable to convert to Subject")
    }
}
