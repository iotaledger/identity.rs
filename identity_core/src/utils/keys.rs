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
