use std::str::FromStr;

use serde::{Deserialize, Serialize};
use serde_diff::SerdeDiff;

use crate::utils::Subject;

/// Public Key type enum. Can also contain a custom key type specified by the CustomKey field.
#[derive(Debug, PartialEq, Clone, SerdeDiff, Deserialize, Serialize)]
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
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize, SerdeDiff)]
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
#[derive(Debug, Clone, Default, PartialEq, SerdeDiff, Deserialize, Serialize)]
pub struct PublicKey {
    pub id: Subject,
    #[serde(rename = "type")]
    pub key_type: PublicKeyTypes,
    pub controller: Subject,
    #[serde(flatten)]
    pub encoding_type: KeyData,
    #[serde(skip)]
    pub reference: bool,
}

impl PublicKey {
    /// creates a new public key instance using `id`, `key_type`, `controller`, and `key_data`. `reference` is
    /// set to false by default.
    pub fn new(id: String, key_type: String, controller: String, key_data: KeyData) -> crate::Result<Self> {
        Ok(PublicKey {
            id: Subject::new(id)?,
            key_type: PublicKeyTypes::from_str(&key_type)?,
            controller: Subject::new(controller)?,
            encoding_type: key_data,
            reference: false,
        })
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
        Ok(serde_json::from_str(s)?)
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_key() {
        let key_data = KeyData::Base58("ass".into());

        let key = PublicKey::new(
            "did:into:123".into(),
            "RsaVerificationKey2018".into(),
            "did:into:123".into(),
            key_data,
        )
        .unwrap();

        println!("{}", serde_json::to_string(&key).unwrap());
    }
}
