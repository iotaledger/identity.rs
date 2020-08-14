use std::str::FromStr;

use serde::{Deserialize as DeriveD, Serialize as DeriveS};

use crate::utils::Subject;

#[derive(Debug, PartialEq, Clone, Copy, DeriveD, DeriveS)]
#[serde(rename_all = "PascalCase")]
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

#[derive(Debug, PartialEq, Clone, Copy, DeriveD, DeriveS)]
pub enum KeyEncodingType {
    Unknown,
    Pem,
    Jwk,
    Hex,
    Base64,
    Base58,
    Multibase,
    IotaAddress,
    EthereumAddress,
}

#[derive(Debug, Clone, Default)]
pub struct PublicKey {
    pub id: Subject,
    pub key_type: PublicKeyTypes,
    pub controller: Subject,
    pub encoding_type: KeyEncodingType,
    pub key_data: String,
    pub reference: bool,
}

impl Default for PublicKeyTypes {
    fn default() -> Self {
        PublicKeyTypes::UnknownKey
    }
}

impl Default for KeyEncodingType {
    fn default() -> Self {
        KeyEncodingType::Unknown
    }
}

impl FromStr for KeyEncodingType {
    type Err = crate::Error;

    fn from_str(s: &str) -> crate::Result<KeyEncodingType> {
        match s {
            "publicKeyUnknown" => Ok(KeyEncodingType::Unknown),
            "publicKeyPem" => Ok(KeyEncodingType::Pem),
            "publicKeyJwk" => Ok(KeyEncodingType::Jwk),
            "publicKeyHex" => Ok(KeyEncodingType::Hex),
            "publicKeyBase64" => Ok(KeyEncodingType::Base64),
            "publicKeyBase58" => Ok(KeyEncodingType::Base58),
            "publicKeyMultibase" => Ok(KeyEncodingType::Multibase),
            "iotaAdress" => Ok(KeyEncodingType::IotaAddress),
            "ethereumAdress" => Ok(KeyEncodingType::EthereumAddress),
            _ => Err(crate::Error::KeyFormatError),
        }
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
            _ => Err(crate::Error::KeyTypeError),
        }
    }
}

impl ToString for KeyEncodingType {
    fn to_string(&self) -> String {
        match self {
            KeyEncodingType::Unknown => "publicKeyUnknown".into(),
            KeyEncodingType::Pem => "publicKeyPem".into(),
            KeyEncodingType::Jwk => "publicKeyJwk".into(),
            KeyEncodingType::Base64 => "publicKeyBase64".into(),
            KeyEncodingType::Base58 => "publicKeyBase58".into(),
            KeyEncodingType::Hex => "publicKeyHex".into(),
            KeyEncodingType::IotaAddress => "iotaAdress".into(),
            KeyEncodingType::EthereumAddress => "ethereumAdress".into(),
            KeyEncodingType::Multibase => "publicKeyMultibase".into(),
            _ => String::new(),
        }
    }
}
