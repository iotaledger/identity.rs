// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use strum::EnumString;

use fastcrypto::{
    bls12381::min_sig::{
        BLS12381AggregateSignature, BLS12381AggregateSignatureAsBytes, BLS12381KeyPair,
        BLS12381PrivateKey, BLS12381PublicKey, BLS12381Signature,
    },
    ed25519::{
        Ed25519KeyPair, Ed25519PrivateKey, Ed25519PublicKey, Ed25519PublicKeyAsBytes,
        Ed25519Signature
    },
    hash::{Blake2b256, HashFunction},
    traits::{VerifyingKey, Authenticator},
    encoding::{Base64},
    secp256k1::{
        Secp256k1PublicKeyAsBytes
    },
    secp256r1::{
        Secp256r1PublicKeyAsBytes
    }
};
use fastcrypto_zkp::{zk_login_utils::Bn254FrElement};

use serde::Deserialize;
use serde::Serialize;
use serde_with::{serde_as, Bytes};

use super::{
    iota_serde::{Readable},
    error::{IotaResult, IotaError},
};

// Authority Objects
pub type AuthorityKeyPair = BLS12381KeyPair;
pub type AuthorityPublicKey = BLS12381PublicKey;
pub type AuthorityPrivateKey = BLS12381PrivateKey;
pub type AuthoritySignature = BLS12381Signature;
pub type AggregateAuthoritySignature = BLS12381AggregateSignature;
pub type AggregateAuthoritySignatureAsBytes = BLS12381AggregateSignatureAsBytes;

// TODO(joyqvq): prefix these types with Default, DefaultAccountKeyPair etc
pub type AccountKeyPair = Ed25519KeyPair;
pub type AccountPublicKey = Ed25519PublicKey;
pub type AccountPrivateKey = Ed25519PrivateKey;

pub type NetworkKeyPair = Ed25519KeyPair;
pub type NetworkPublicKey = Ed25519PublicKey;
pub type NetworkPrivateKey = Ed25519PrivateKey;

pub type DefaultHash = Blake2b256;


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PublicKey {
    Ed25519(Ed25519PublicKeyAsBytes),
    Secp256k1(Secp256k1PublicKeyAsBytes),
    Secp256r1(Secp256r1PublicKeyAsBytes),
    ZkLogin(ZkLoginPublicIdentifier),
}
/// A wrapper struct to retrofit in [enum PublicKey] for zkLogin.
/// Useful to construct [struct MultiSigPublicKey].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ZkLoginPublicIdentifier(pub Vec<u8>); // #[schemars(with = "Base64")]

impl ZkLoginPublicIdentifier {
    /// Consists of iss_bytes_len || iss_bytes || padded_32_byte_address_seed.
    pub fn new(iss: &str, address_seed: &Bn254FrElement) -> IotaResult<Self> {
        let mut bytes = Vec::new();
        let iss_bytes = iss.as_bytes();
        bytes.extend([iss_bytes.len() as u8]);
        bytes.extend(iss_bytes);
        bytes.extend(address_seed.padded());

        Ok(Self(bytes))
    }
}
impl AsRef<[u8]> for PublicKey {
    fn as_ref(&self) -> &[u8] {
        match self {
            PublicKey::Ed25519(pk) => &pk.0,
            PublicKey::Secp256k1(pk) => &pk.0,
            PublicKey::Secp256r1(pk) => &pk.0,
            PublicKey::ZkLogin(z) => &z.0,
        }
    }
}

impl PublicKey {
    pub fn scheme(&self) -> SignatureScheme {
        match self {
            PublicKey::Ed25519(_) => SignatureScheme::ED25519,
            PublicKey::Secp256k1(_) => SignatureScheme::Secp256k1,
            PublicKey::Secp256r1(_) => SignatureScheme::Secp256r1,
            PublicKey::ZkLogin(_) => SignatureScheme::ZkLoginAuthenticator,
        }
    }
}

/// Defines the compressed version of the public key that we pass around
/// in Iota
#[serde_as]
#[derive(
Copy,
Clone,
PartialEq,
Eq,
Hash,
PartialOrd,
Ord,
Serialize,
Deserialize,
Debug
)]
pub struct AuthorityPublicKeyBytes(
    #[serde_as(as = "Readable<Base64, Bytes>")]
    pub [u8; AuthorityPublicKey::LENGTH],
);

// BLS Port
//

impl IotaPublicKey for BLS12381PublicKey {
    const SIGNATURE_SCHEME: SignatureScheme = SignatureScheme::BLS12381;
}

// Ed25519 Iota Signature port
//

#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Ed25519IotaSignature(
    #[serde_as(as = "Readable<Base64, Bytes>")]
    [u8; Ed25519PublicKey::LENGTH + Ed25519Signature::LENGTH + 1],
);

// Implementation useful for simplify testing when mock signature is needed
impl Default for Ed25519IotaSignature {
    fn default() -> Self {
        Self([0; Ed25519PublicKey::LENGTH + Ed25519Signature::LENGTH + 1])
    }
}

impl IotaPublicKey for Ed25519PublicKey {
    const SIGNATURE_SCHEME: SignatureScheme = SignatureScheme::ED25519;
}

pub trait IotaPublicKey: VerifyingKey {
    const SIGNATURE_SCHEME: SignatureScheme;
}


#[derive(Clone, Copy, Deserialize, Serialize, Debug, EnumString, strum::Display)]
#[strum(serialize_all = "lowercase")]
pub enum SignatureScheme {
    ED25519,
    Secp256k1,
    Secp256r1,
    BLS12381, // This is currently not supported for user Iota Address.
    MultiSig,
    ZkLoginAuthenticator,
}

impl SignatureScheme {
    pub fn flag(&self) -> u8 {
        match self {
            SignatureScheme::ED25519 => 0x00,
            SignatureScheme::Secp256k1 => 0x01,
            SignatureScheme::Secp256r1 => 0x02,
            SignatureScheme::MultiSig => 0x03,
            SignatureScheme::BLS12381 => 0x04, // This is currently not supported for user Iota
            // Address.
            SignatureScheme::ZkLoginAuthenticator => 0x05,
        }
    }

    /// Takes as input an hasher and updates it with a flag byte if the input
    /// scheme is not ED25519; it does nothing otherwise.
    pub fn update_hasher_with_flag(&self, hasher: &mut DefaultHash) {
        match self {
            SignatureScheme::ED25519 => (),
            _ => hasher.update([self.flag()]),
        };
    }

    pub fn from_flag(flag: &str) -> Result<SignatureScheme, IotaError> {
        let byte_int = flag
            .parse::<u8>()
            .map_err(|_| IotaError::KeyConversion("Invalid key scheme".to_string()))?;
        Self::from_flag_byte(&byte_int)
    }

    pub fn from_flag_byte(byte_int: &u8) -> Result<SignatureScheme, IotaError> {
        match byte_int {
            0x00 => Ok(SignatureScheme::ED25519),
            0x01 => Ok(SignatureScheme::Secp256k1),
            0x02 => Ok(SignatureScheme::Secp256r1),
            0x03 => Ok(SignatureScheme::MultiSig),
            0x04 => Ok(SignatureScheme::BLS12381),
            0x05 => Ok(SignatureScheme::ZkLoginAuthenticator),
            _ => Err(IotaError::KeyConversion("Invalid key scheme".to_string())),
        }
    }
}