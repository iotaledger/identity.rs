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
pub enum KeyType {
    JsonWebKey2020,
    EcdsaSecp256k1VerificationKey2019,
    Ed25519VerificationKey2018,
    GpgVerificationKey2020,
    RsaVerificationKey2018,
    X25519KeyAgreementKey2019,
    SchnorrSecp256k1VerificationKey2019,
    EcdsaSecp256k1RecoveryMethod2020,
}

impl KeyType {
    pub fn try_from_str(string: &str) -> Result<Self> {
        match string {
            "JsonWebKey2020" => Ok(Self::JsonWebKey2020),
            "Secp256k1VerificationKey2018" => Ok(Self::EcdsaSecp256k1VerificationKey2019),
            "Ed25519VerificationKey2018" => Ok(Self::Ed25519VerificationKey2018),
            "GpgVerificationKey2020" => Ok(Self::GpgVerificationKey2020),
            "RsaVerificationKey2018" => Ok(Self::RsaVerificationKey2018),
            "X25519KeyAgreementKey2019" => Ok(Self::X25519KeyAgreementKey2019),
            "SchnorrSecp256k1VerificationKey2019" => Ok(Self::SchnorrSecp256k1VerificationKey2019),
            "EcdsaSecp256k1RecoveryMethod2020" => Ok(Self::EcdsaSecp256k1RecoveryMethod2020),
            _ => Err(Error::InvalidKeyType),
        }
    }

    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::JsonWebKey2020 => "JsonWebKey2020",
            Self::EcdsaSecp256k1VerificationKey2019 => "Secp256k1VerificationKey2018",
            Self::Ed25519VerificationKey2018 => "Ed25519VerificationKey2018",
            Self::GpgVerificationKey2020 => "GpgVerificationKey2020",
            Self::RsaVerificationKey2018 => "RsaVerificationKey2018",
            Self::X25519KeyAgreementKey2019 => "X25519KeyAgreementKey2019",
            Self::SchnorrSecp256k1VerificationKey2019 => "SchnorrSecp256k1VerificationKey2019",
            Self::EcdsaSecp256k1RecoveryMethod2020 => "X25519KeyAgreementKey2019",
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
