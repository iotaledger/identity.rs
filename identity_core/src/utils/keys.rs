use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PublicKeyTypes {
    UnknownKey,
    Ed25519Key,
    RSAKey,
    EcdsaSecp265K1Key,
}

pub enum KeyEncoding {
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

pub struct PublicKey {
    key_type: PublicKeyTypes,
    encoding_type: KeyEncoding,
    key_data: Vec<u8>,
    reference: bool,
}

impl Default for PublicKeyTypes {
    fn default() -> Self {
        PublicKeyTypes::UnknownKey
    }
}

impl Default for KeyEncoding {
    fn default() -> Self {
        KeyEncoding::Unknown
    }
}

impl FromStr for KeyEncoding {
    type Err = crate::Error;

    fn from_str(s: &str) -> crate::Result<KeyEncoding> {
        match s {
            "unknown" => Ok(KeyEncoding::Unknown),
            "pem" => Ok(KeyEncoding::Pem),
            "jwk" => Ok(KeyEncoding::Jwk),
            "hex" => Ok(KeyEncoding::Hex),
            "base64" => Ok(KeyEncoding::Base64),
            "base58" => Ok(KeyEncoding::Base58),
            "multibase" => Ok(KeyEncoding::Multibase),
            "iota" => Ok(KeyEncoding::IotaAddress),
            "ethereum" => Ok(KeyEncoding::EthereumAddress),
            _ => Err(crate::Error::KeyFormatError),
        }
    }
}
