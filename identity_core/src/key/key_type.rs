use core::{
    convert::TryFrom,
    fmt::{Display, Formatter, Result as FmtResult},
    str::FromStr,
};
use identity_diff::Diff;
use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};

/// Possible key types within a DID document.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize, Diff)]
#[non_exhaustive]
pub enum KeyType {
    Ed25519VerificationKey2018,
}

impl KeyType {
    pub fn try_from_str(string: &str) -> Result<Self> {
        match string {
            "Ed25519VerificationKey2018" => Ok(Self::Ed25519VerificationKey2018),
            _ => Err(Error::InvalidKeyType),
        }
    }

    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Ed25519VerificationKey2018 => "Ed25519VerificationKey2018",
        }
    }
}

impl Display for KeyType {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.write_str(self.as_str())
    }
}

impl FromStr for KeyType {
    type Err = Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        Self::try_from_str(string)
    }
}

impl TryFrom<&'_ str> for KeyType {
    type Error = Error;

    fn try_from(other: &'_ str) -> Result<Self, Self::Error> {
        Self::try_from_str(other)
    }
}
