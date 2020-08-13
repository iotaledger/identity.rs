use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum PublicKeyTypes {
    UnknownKey,
    Ed25519VerificationKey2018,
    RsaVerificationKey2018,
    EcdsaSecp256k1VerificationKey2018,
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicKey {
    key_type: PublicKeyTypes,
    encoding_type: KeyEncodingType,
    key_data: Vec<u8>,
    reference: bool,
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
