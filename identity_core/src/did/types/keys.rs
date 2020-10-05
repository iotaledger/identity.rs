use identity_diff::Diff;
use serde::{Deserialize, Serialize};
use std::{hash::Hash, str::FromStr};

use crate::did::{utils::HasId, DID};

/// Public Key type enum. Can also contain a custom key type specified by the CustomKey field.
#[derive(Debug, PartialEq, Clone, Diff, Deserialize, Serialize, Eq, Hash, Ord, PartialOrd)]
pub enum PublicKeyTypes {
    Ed25519VerificationKey2018,
    RsaVerificationKey2018,
    EcdsaSecp256k1VerificationKey2019,
    JsonWebKey2020,
    GpgVerificationKey2020,
    X25519KeyAgreementKey2019,
    EcdsaSecp256k1RecoveryMethod2020,
    SchnorrSecp256k1VerificationKey2019,
    UnknownKey,
}

/// Encoding method used for the specified public key.
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize, Diff, Eq, Hash, Ord, PartialOrd)]
pub enum KeyData {
    #[serde(rename = "publicKeyUnknown")]
    Unknown(String),
    #[serde(rename = "publicKeyPem")]
    Pem(String),
    #[serde(rename = "publicKeyJwk")]
    Jwk(String),
    #[serde(rename = "publicKeyHex")]
    Hex(String),
    #[serde(rename = "publicKeyBase64")]
    Base64(String),
    #[serde(rename = "publicKeyBase58")]
    Base58(String),
    #[serde(rename = "publicKeyMultibase")]
    Multibase(String),
    #[serde(rename = "iotaAddress")]
    IotaAddress(String),
    #[serde(rename = "ethereumAddress")]
    EthereumAddress(String),
}

/// Public key struct that contains `id`, `key_type`, `controller`, `encoding_type`, `key_data` and `reference`.
/// `reference` defines whether or not the PublicKey is a reference.
#[derive(Debug, Clone, Default, PartialEq, Diff, Deserialize, Serialize, Eq, Hash, Ord, PartialOrd)]
#[diff(from_into)]
pub struct PublicKey {
    pub id: DID,
    #[serde(rename = "type")]
    pub key_type: PublicKeyTypes,
    pub controller: DID,
    #[serde(flatten)]
    pub key_data: KeyData,
    #[serde(skip)]
    pub reference: bool,
}

impl PublicKey {
    pub fn init(self) -> Self {
        Self {
            id: self.id,
            key_type: self.key_type,
            controller: self.controller,
            key_data: self.key_data,
            reference: self.reference,
        }
    }
}

impl HasId for PublicKey {
    type Id = DID;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

impl Default for PublicKeyTypes {
    fn default() -> Self {
        PublicKeyTypes::UnknownKey
    }
}

impl Default for KeyData {
    fn default() -> Self {
        KeyData::Unknown(String::from(""))
    }
}

impl FromStr for PublicKey {
    type Err = crate::Error;

    fn from_str(s: &str) -> crate::Result<PublicKey> {
        serde_json::from_str(s).map_err(crate::Error::DecodeJSON)
    }
}

impl ToString for PublicKey {
    fn to_string(&self) -> String {
        serde_json::to_string(self).expect("Unable to serialize Public Key")
    }
}

impl FromStr for PublicKeyTypes {
    type Err = crate::Error;

    fn from_str(s: &str) -> crate::Result<Self> {
        match s {
            "RsaVerificationKey2018" => Ok(Self::RsaVerificationKey2018),
            "Ed25519VerificationKey2018" => Ok(Self::Ed25519VerificationKey2018),
            "Secp256k1VerificationKey2018" => Ok(Self::EcdsaSecp256k1VerificationKey2019),
            "JsonWebKey2020" => Ok(Self::JsonWebKey2020),
            "GpgVerificationKey2020" => Ok(Self::GpgVerificationKey2020),
            "X25519KeyAgreementKey2019" => Ok(Self::X25519KeyAgreementKey2019),
            "EcdsaSecp256k1RecoveryMethod2020" => Ok(Self::EcdsaSecp256k1RecoveryMethod2020),
            "SchnorrSecp256k1VerificationKey2019" => Ok(Self::SchnorrSecp256k1VerificationKey2019),
            _ => Ok(Self::UnknownKey),
        }
    }
}

impl ToString for PublicKeyTypes {
    fn to_string(&self) -> String {
        match self {
            PublicKeyTypes::RsaVerificationKey2018 => "RsaVerificationKey2018".into(),
            PublicKeyTypes::Ed25519VerificationKey2018 => "Ed25519VerificationKey2018".into(),
            PublicKeyTypes::EcdsaSecp256k1VerificationKey2019 => "Secp256k1VerificationKey2018".into(),
            PublicKeyTypes::JsonWebKey2020 => "JsonWebKey2020".into(),
            PublicKeyTypes::GpgVerificationKey2020 => "GpgVerificationKey2020".into(),
            PublicKeyTypes::X25519KeyAgreementKey2019 => "X25519KeyAgreementKey2019".into(),
            PublicKeyTypes::EcdsaSecp256k1RecoveryMethod2020 => "X25519KeyAgreementKey2019".into(),
            PublicKeyTypes::SchnorrSecp256k1VerificationKey2019 => "SchnorrSecp256k1VerificationKey2019".into(),
            PublicKeyTypes::UnknownKey => "".into(),
        }
    }
}

impl From<&str> for PublicKeyTypes {
    fn from(s: &str) -> Self {
        match s {
            "RsaVerificationKey2018" => Self::RsaVerificationKey2018,
            "Ed25519VerificationKey2018" => Self::Ed25519VerificationKey2018,
            "Secp256k1VerificationKey2018" => Self::EcdsaSecp256k1VerificationKey2019,
            "JsonWebKey2020" => Self::JsonWebKey2020,
            "GpgVerificationKey2020" => Self::GpgVerificationKey2020,
            "X25519KeyAgreementKey2019" => Self::X25519KeyAgreementKey2019,
            "EcdsaSecp256k1RecoveryMethod2020" => Self::EcdsaSecp256k1RecoveryMethod2020,
            "SchnorrSecp256k1VerificationKey2019" => Self::SchnorrSecp256k1VerificationKey2019,
            _ => Self::UnknownKey,
        }
    }
}
