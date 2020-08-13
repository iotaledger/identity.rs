use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum PublicKeyTypes {
    UnknownKey,
    Ed25519Key,
    RSAKey,
    Ed25519VerificationKey,
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
            "unknown" => Ok(KeyEncodingType::Unknown),
            "pem" => Ok(KeyEncodingType::Pem),
            "jwk" => Ok(KeyEncodingType::Jwk),
            "hex" => Ok(KeyEncodingType::Hex),
            "base64" => Ok(KeyEncodingType::Base64),
            "base58" => Ok(KeyEncodingType::Base58),
            "multibase" => Ok(KeyEncodingType::Multibase),
            "iota" => Ok(KeyEncodingType::IotaAddress),
            "ethereum" => Ok(KeyEncodingType::EthereumAddress),
            _ => Err(crate::Error::KeyFormatError),
        }
    }
}
