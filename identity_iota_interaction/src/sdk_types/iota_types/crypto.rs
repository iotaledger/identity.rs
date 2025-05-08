// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
use std::hash::Hash;
use std::str::FromStr;

use enum_dispatch::enum_dispatch;
use strum::EnumString;
use schemars::JsonSchema;
use derive_more::{AsRef, AsMut, From};
use eyre::{eyre, Report};

use fastcrypto::{
    bls12381::min_sig::{
        BLS12381AggregateSignature, BLS12381AggregateSignatureAsBytes, BLS12381KeyPair,
        BLS12381PrivateKey, BLS12381PublicKey, BLS12381Signature,
    }, ed25519::{
        Ed25519KeyPair, Ed25519PrivateKey, Ed25519PublicKey, Ed25519PublicKeyAsBytes,
        Ed25519Signature
    }, encoding::{Base64, Bech32, Encoding}, error::{FastCryptoError, FastCryptoResult}, hash::{Blake2b256, HashFunction}, secp256k1::{Secp256k1KeyPair, Secp256k1PublicKey, Secp256k1PublicKeyAsBytes, Secp256k1Signature}, secp256r1::{Secp256r1KeyPair, Secp256r1PublicKey, Secp256r1PublicKeyAsBytes, Secp256r1Signature}, traits::{Authenticator, EncodeDecodeBase64, KeyPair as KeypairTraits, Signer, ToFromBytes, VerifyingKey}
};
use fastcrypto_zkp::zk_login_utils::Bn254FrElement;

use serde::{Deserialize, Deserializer, Serializer};
use serde::Serialize;
use serde_with::{serde_as, Bytes};

use crate::shared_crypto::intent::IntentMessage;

use super::{
    base_types::IotaAddress, error::{IotaError, IotaResult}, iota_serde::Readable
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

pub const DEFAULT_EPOCH_ID: EpochId = 0;
const IOTA_PRIV_KEY_PREFIX: &str = "iotaprivkey";

// Account Keys
//
// * The following section defines the keypairs that are used by
// * accounts to interact with Iota.
// * Currently we support eddsa and ecdsa on Iota.

#[expect(clippy::large_enum_variant)]
#[derive(Debug, From, PartialEq, Eq)]
pub enum IotaKeyPair {
    Ed25519(Ed25519KeyPair),
    Secp256k1(Secp256k1KeyPair),
    Secp256r1(Secp256r1KeyPair),
}

impl IotaKeyPair {
    pub fn public(&self) -> PublicKey {
        match self {
            IotaKeyPair::Ed25519(kp) => PublicKey::Ed25519(kp.public().into()),
            IotaKeyPair::Secp256k1(kp) => PublicKey::Secp256k1(kp.public().into()),
            IotaKeyPair::Secp256r1(kp) => PublicKey::Secp256r1(kp.public().into()),
        }
    }

    pub fn copy(&self) -> Self {
        match self {
            IotaKeyPair::Ed25519(kp) => kp.copy().into(),
            IotaKeyPair::Secp256k1(kp) => kp.copy().into(),
            IotaKeyPair::Secp256r1(kp) => kp.copy().into(),
        }
    }
}

impl EncodeDecodeBase64 for IotaKeyPair {
    fn encode_base64(&self) -> String {
        Base64::encode(self.to_bytes())
    }

    fn decode_base64(value: &str) -> FastCryptoResult<Self> {
        let bytes = Base64::decode(value)?;
        Self::from_bytes(&bytes).map_err(|_| FastCryptoError::InvalidInput)
    }
}
impl IotaKeyPair {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.push(self.public().flag());

        match self {
            IotaKeyPair::Ed25519(kp) => {
                bytes.extend_from_slice(kp.as_bytes());
            }
            IotaKeyPair::Secp256k1(kp) => {
                bytes.extend_from_slice(kp.as_bytes());
            }
            IotaKeyPair::Secp256r1(kp) => {
                bytes.extend_from_slice(kp.as_bytes());
            }
        }
        bytes
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, eyre::Report> {
        match SignatureScheme::from_flag_byte(bytes.first().ok_or_else(|| eyre!("Invalid length"))?)
        {
            Ok(x) => match x {
                SignatureScheme::ED25519 => Ok(IotaKeyPair::Ed25519(Ed25519KeyPair::from_bytes(
                    bytes.get(1..).ok_or_else(|| eyre!("Invalid length"))?,
                )?)),
                SignatureScheme::Secp256k1 => {
                    Ok(IotaKeyPair::Secp256k1(Secp256k1KeyPair::from_bytes(
                        bytes.get(1..).ok_or_else(|| eyre!("Invalid length"))?,
                    )?))
                }
                SignatureScheme::Secp256r1 => {
                    Ok(IotaKeyPair::Secp256r1(Secp256r1KeyPair::from_bytes(
                        bytes.get(1..).ok_or_else(|| eyre!("Invalid length"))?,
                    )?))
                }
                _ => Err(eyre!("Invalid flag byte")),
            },
            _ => Err(eyre!("Invalid bytes")),
        }
    }

    pub fn to_bytes_no_flag(&self) -> Vec<u8> {
        match self {
            IotaKeyPair::Ed25519(kp) => kp.as_bytes().to_vec(),
            IotaKeyPair::Secp256k1(kp) => kp.as_bytes().to_vec(),
            IotaKeyPair::Secp256r1(kp) => kp.as_bytes().to_vec(),
        }
    }

    /// Encode a IotaKeyPair as `flag || privkey` in Bech32 starting with
    /// "iotaprivkey" to a string. Note that the pubkey is not encoded.
    pub fn encode(&self) -> Result<String, eyre::Report> {
        Bech32::encode(self.to_bytes(), IOTA_PRIV_KEY_PREFIX).map_err(|e| eyre!(e))
    }

    /// Decode a IotaKeyPair from `flag || privkey` in Bech32 starting with
    /// "iotaprivkey" to IotaKeyPair. The public key is computed directly from
    /// the private key bytes.
    pub fn decode(value: &str) -> Result<Self, eyre::Report> {
        let bytes = Bech32::decode(value, IOTA_PRIV_KEY_PREFIX)?;
        Self::from_bytes(&bytes)
    }
}

impl Serialize for IotaKeyPair {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = self.encode_base64();
        serializer.serialize_str(&s)
    }
}

impl<'de> Deserialize<'de> for IotaKeyPair {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        let s = String::deserialize(deserializer)?;
        IotaKeyPair::decode_base64(&s).map_err(|e| Error::custom(e.to_string()))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PublicKey {
    Ed25519(Ed25519PublicKeyAsBytes),
    Secp256k1(Secp256k1PublicKeyAsBytes),
    Secp256r1(Secp256r1PublicKeyAsBytes),
    ZkLogin(ZkLoginPublicIdentifier),
    Passkey(Secp256r1PublicKeyAsBytes),
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
            PublicKey::Passkey(pk) => &pk.0,
        }
    }
}

impl EncodeDecodeBase64 for PublicKey {
    fn encode_base64(&self) -> String {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.extend_from_slice(&[self.flag()]);
        bytes.extend_from_slice(self.as_ref());
        Base64::encode(&bytes[..])
    }

    fn decode_base64(value: &str) -> FastCryptoResult<Self> {
        let bytes = Base64::decode(value)?;
        match bytes.first() {
            Some(x) => {
                if x == &SignatureScheme::ED25519.flag() {
                    let pk: Ed25519PublicKey =
                        Ed25519PublicKey::from_bytes(bytes.get(1..).ok_or(
                            FastCryptoError::InputLengthWrong(Ed25519PublicKey::LENGTH + 1),
                        )?)?;
                    Ok(PublicKey::Ed25519((&pk).into()))
                } else if x == &SignatureScheme::Secp256k1.flag() {
                    let pk = Secp256k1PublicKey::from_bytes(bytes.get(1..).ok_or(
                        FastCryptoError::InputLengthWrong(Secp256k1PublicKey::LENGTH + 1),
                    )?)?;
                    Ok(PublicKey::Secp256k1((&pk).into()))
                } else if x == &SignatureScheme::Secp256r1.flag() {
                    let pk = Secp256r1PublicKey::from_bytes(bytes.get(1..).ok_or(
                        FastCryptoError::InputLengthWrong(Secp256r1PublicKey::LENGTH + 1),
                    )?)?;
                    Ok(PublicKey::Secp256r1((&pk).into()))
                } else if x == &SignatureScheme::PasskeyAuthenticator.flag() {
                    let pk = Secp256r1PublicKey::from_bytes(bytes.get(1..).ok_or(
                        FastCryptoError::InputLengthWrong(Secp256r1PublicKey::LENGTH + 1),
                    )?)?;
                    Ok(PublicKey::Passkey((&pk).into()))
                } else {
                    Err(FastCryptoError::InvalidInput)
                }
            }
            _ => Err(FastCryptoError::InvalidInput),
        }
    }
}

impl PublicKey {
    pub fn flag(&self) -> u8 {
        self.scheme().flag()
    }

    pub fn try_from_bytes(
        curve: SignatureScheme,
        key_bytes: &[u8],
    ) -> Result<PublicKey, Report> {
        match curve {
            SignatureScheme::ED25519 => Ok(PublicKey::Ed25519(
                (&Ed25519PublicKey::from_bytes(key_bytes)?).into(),
            )),
            SignatureScheme::Secp256k1 => Ok(PublicKey::Secp256k1(
                (&Secp256k1PublicKey::from_bytes(key_bytes)?).into(),
            )),
            SignatureScheme::Secp256r1 => Ok(PublicKey::Secp256r1(
                (&Secp256r1PublicKey::from_bytes(key_bytes)?).into(),
            )),
            SignatureScheme::PasskeyAuthenticator => Ok(PublicKey::Passkey(
                (&Secp256r1PublicKey::from_bytes(key_bytes)?).into(),
            )),
            _ => Err(eyre!("Unsupported curve")),
        }
    }

    pub fn scheme(&self) -> SignatureScheme {
        match self {
            PublicKey::Ed25519(_) => SignatureScheme::ED25519,     // Equals Ed25519IotaSignature::SCHEME
            PublicKey::Secp256k1(_) => SignatureScheme::Secp256k1, // Equals Secp256k1IotaSignature::SCHEME
            PublicKey::Secp256r1(_) => SignatureScheme::Secp256r1, // Equals Secp256r1IotaSignature::SCHEME
            PublicKey::ZkLogin(_) => SignatureScheme::ZkLoginAuthenticator,
            PublicKey::Passkey(_) => SignatureScheme::PasskeyAuthenticator,
        }
    }
}

/// Defines the compressed version of the public key that we pass around
/// in IOTA.
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
Debug // schemars::JsonSchema and AsRef are omitted here, having Debug instead 
)]
pub struct AuthorityPublicKeyBytes(
    #[serde_as(as = "Readable<Base64, Bytes>")]
    pub [u8; AuthorityPublicKey::LENGTH],
);

// Enums for signature scheme signatures
#[enum_dispatch]
#[derive(Clone, JsonSchema, Debug, PartialEq, Eq, Hash)]
pub enum Signature {
    Ed25519IotaSignature,
    Secp256k1IotaSignature,
    Secp256r1IotaSignature,
}

impl Serialize for Signature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let bytes = self.as_ref();

        if serializer.is_human_readable() {
            let s = Base64::encode(bytes);
            serializer.serialize_str(&s)
        } else {
            serializer.serialize_bytes(bytes)
        }
    }
}

impl<'de> Deserialize<'de> for Signature {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;

        let bytes = if deserializer.is_human_readable() {
            let s = String::deserialize(deserializer)?;
            Base64::decode(&s).map_err(|e| Error::custom(e.to_string()))?
        } else {
            let data: Vec<u8> = Vec::deserialize(deserializer)?;
            data
        };

        Self::from_bytes(&bytes).map_err(|e| Error::custom(e.to_string()))
    }
}

impl AsRef<[u8]> for Signature {
    fn as_ref(&self) -> &[u8] {
        match self {
            Signature::Ed25519IotaSignature(sig) => sig.as_ref(),
            Signature::Secp256k1IotaSignature(sig) => sig.as_ref(),
            Signature::Secp256r1IotaSignature(sig) => sig.as_ref(),
        }
    }
}
impl AsMut<[u8]> for Signature {
    fn as_mut(&mut self) -> &mut [u8] {
        match self {
            Signature::Ed25519IotaSignature(sig) => sig.as_mut(),
            Signature::Secp256k1IotaSignature(sig) => sig.as_mut(),
            Signature::Secp256r1IotaSignature(sig) => sig.as_mut(),
        }
    }
}

impl ToFromBytes for Signature {
    fn from_bytes(bytes: &[u8]) -> Result<Self, FastCryptoError> {
        match bytes.first() {
            Some(x) => {
                if x == &Ed25519IotaSignature::SCHEME.flag() {
                    Ok(<Ed25519IotaSignature as ToFromBytes>::from_bytes(bytes)?.into())
                } else if x == &Secp256k1IotaSignature::SCHEME.flag() {
                    Ok(<Secp256k1IotaSignature as ToFromBytes>::from_bytes(bytes)?.into())
                } else if x == &Secp256r1IotaSignature::SCHEME.flag() {
                    Ok(<Secp256r1IotaSignature as ToFromBytes>::from_bytes(bytes)?.into())
                } else {
                    Err(FastCryptoError::InvalidInput)
                }
            }
            _ => Err(FastCryptoError::InvalidInput),
        }
    }
}

// Ed25519 Iota Signature port
//

#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Hash, AsRef, AsMut)]
#[as_ref(forward)]
#[as_mut(forward)]
pub struct Ed25519IotaSignature(
    #[schemars(with = "Base64")]
    #[serde_as(as = "Readable<Base64, Bytes>")]
    [u8; Ed25519PublicKey::LENGTH + Ed25519Signature::LENGTH + 1],
);

// Implementation useful for simplify testing when mock signature is needed
impl Default for Ed25519IotaSignature {
    fn default() -> Self {
        Self([0; Ed25519PublicKey::LENGTH + Ed25519Signature::LENGTH + 1])
    }
}

impl IotaSignatureInner for Ed25519IotaSignature {
    type Sig = Ed25519Signature;
    type PubKey = Ed25519PublicKey;
    type KeyPair = Ed25519KeyPair;
    const LENGTH: usize = Ed25519PublicKey::LENGTH + Ed25519Signature::LENGTH + 1;
}

impl IotaPublicKey for Ed25519PublicKey {
    const SIGNATURE_SCHEME: SignatureScheme = SignatureScheme::ED25519;
}

impl ToFromBytes for Ed25519IotaSignature {
    fn from_bytes(bytes: &[u8]) -> Result<Self, FastCryptoError> {
        if bytes.len() != Self::LENGTH {
            return Err(FastCryptoError::InputLengthWrong(Self::LENGTH));
        }
        let mut sig_bytes = [0; Self::LENGTH];
        sig_bytes.copy_from_slice(bytes);
        Ok(Self(sig_bytes))
    }
}

// Secp256k1 Iota Signature port
//
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Hash, AsRef, AsMut)]
#[as_ref(forward)]
#[as_mut(forward)]
pub struct Secp256k1IotaSignature(
    #[schemars(with = "Base64")]
    #[serde_as(as = "Readable<Base64, Bytes>")]
    [u8; Secp256k1PublicKey::LENGTH + Secp256k1Signature::LENGTH + 1],
);

impl IotaSignatureInner for Secp256k1IotaSignature {
    type Sig = Secp256k1Signature;
    type PubKey = Secp256k1PublicKey;
    type KeyPair = Secp256k1KeyPair;
    const LENGTH: usize = Secp256k1PublicKey::LENGTH + Secp256k1Signature::LENGTH + 1;
}

impl IotaPublicKey for Secp256k1PublicKey {
    const SIGNATURE_SCHEME: SignatureScheme = SignatureScheme::Secp256k1;
}

impl ToFromBytes for Secp256k1IotaSignature {
    fn from_bytes(bytes: &[u8]) -> Result<Self, FastCryptoError> {
        if bytes.len() != Self::LENGTH {
            return Err(FastCryptoError::InputLengthWrong(Self::LENGTH));
        }
        let mut sig_bytes = [0; Self::LENGTH];
        sig_bytes.copy_from_slice(bytes);
        Ok(Self(sig_bytes))
    }
}

// Secp256r1 Iota Signature port
//
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Hash, AsRef, AsMut)]
#[as_ref(forward)]
#[as_mut(forward)]
pub struct Secp256r1IotaSignature(
    #[schemars(with = "Base64")]
    #[serde_as(as = "Readable<Base64, Bytes>")]
    [u8; Secp256r1PublicKey::LENGTH + Secp256r1Signature::LENGTH + 1],
);

impl IotaSignatureInner for Secp256r1IotaSignature {
    type Sig = Secp256r1Signature;
    type PubKey = Secp256r1PublicKey;
    type KeyPair = Secp256r1KeyPair;
    const LENGTH: usize = Secp256r1PublicKey::LENGTH + Secp256r1Signature::LENGTH + 1;
}

impl IotaPublicKey for Secp256r1PublicKey {
    const SIGNATURE_SCHEME: SignatureScheme = SignatureScheme::Secp256r1;
}

impl ToFromBytes for Secp256r1IotaSignature {
    fn from_bytes(bytes: &[u8]) -> Result<Self, FastCryptoError> {
        if bytes.len() != Self::LENGTH {
            return Err(FastCryptoError::InputLengthWrong(Self::LENGTH));
        }
        let mut sig_bytes = [0; Self::LENGTH];
        sig_bytes.copy_from_slice(bytes);
        Ok(Self(sig_bytes))
    }
}

// This struct exists due to the limitations of the `enum_dispatch` library.
//
pub trait IotaSignatureInner: Sized + ToFromBytes + PartialEq + Eq + Hash {
    type Sig: Authenticator<PubKey = Self::PubKey>;
    type PubKey: VerifyingKey<Sig = Self::Sig> + IotaPublicKey;
    type KeyPair: KeypairTraits<PubKey = Self::PubKey, Sig = Self::Sig>;

    const LENGTH: usize = Self::Sig::LENGTH + Self::PubKey::LENGTH + 1;
    const SCHEME: SignatureScheme = Self::PubKey::SIGNATURE_SCHEME;

    /// Returns the deserialized signature and deserialized pubkey.
    fn get_verification_inputs(&self) -> IotaResult<(Self::Sig, Self::PubKey)> {
        let pk = Self::PubKey::from_bytes(self.public_key_bytes())
          .map_err(|_| IotaError::KeyConversion("Invalid public key".to_string()))?;

        // deserialize the signature
        let signature = Self::Sig::from_bytes(self.signature_bytes()).map_err(|_| {
            IotaError::InvalidSignature {
                error: "Fail to get pubkey and sig".to_string(),
            }
        })?;

        Ok((signature, pk))
    }

    fn new(kp: &Self::KeyPair, message: &[u8]) -> Self {
        let sig = Signer::sign(kp, message);

        let mut signature_bytes: Vec<u8> = Vec::new();
        signature_bytes
          .extend_from_slice(&[<Self::PubKey as IotaPublicKey>::SIGNATURE_SCHEME.flag()]);
        signature_bytes.extend_from_slice(sig.as_ref());
        signature_bytes.extend_from_slice(kp.public().as_ref());
        Self::from_bytes(&signature_bytes[..])
          .expect("Serialized signature did not have expected size")
    }
}

pub trait IotaPublicKey: VerifyingKey {
    const SIGNATURE_SCHEME: SignatureScheme;
}

#[enum_dispatch(Signature)]
pub trait IotaSignature: Sized + ToFromBytes {
    fn signature_bytes(&self) -> &[u8];
    fn public_key_bytes(&self) -> &[u8];
    fn scheme(&self) -> SignatureScheme;

    fn verify_secure<T>(
        &self,
        value: &IntentMessage<T>,
        author: IotaAddress,
        scheme: SignatureScheme,
    ) -> IotaResult<()>
    where
        T: Serialize;
}

impl<S: IotaSignatureInner + Sized> IotaSignature for S {
    fn signature_bytes(&self) -> &[u8] {
        // Access array slice is safe because the array bytes is initialized as
        // flag || signature || pubkey with its defined length.
        &self.as_ref()[1..1 + S::Sig::LENGTH]
    }

    fn public_key_bytes(&self) -> &[u8] {
        // Access array slice is safe because the array bytes is initialized as
        // flag || signature || pubkey with its defined length.
        &self.as_ref()[S::Sig::LENGTH + 1..]
    }

    fn scheme(&self) -> SignatureScheme {
        S::PubKey::SIGNATURE_SCHEME
    }

    fn verify_secure<T>(
        &self,
        value: &IntentMessage<T>,
        author: IotaAddress,
        scheme: SignatureScheme,
    ) -> Result<(), IotaError>
    where
        T: Serialize,
    {
        let mut hasher = DefaultHash::default();
        hasher.update(bcs::to_bytes(&value).expect("Message serialization should not fail"));
        let digest = hasher.finalize().digest;

        let (sig, pk) = &self.get_verification_inputs()?;
        match scheme {
            SignatureScheme::ZkLoginAuthenticator => {} // Pass this check because zk login does
            // not derive address from pubkey.
            _ => {
                let address = IotaAddress::from(pk);
                if author != address {
                    return Err(IotaError::IncorrectSigner {
                        error: format!(
                            "Incorrect signer, expected {:?}, got {:?}",
                            author, address
                        ),
                    });
                }
            }
        }

        pk.verify(&digest, sig)
            .map_err(|e| IotaError::InvalidSignature {
                error: format!("Fail to verify user sig {}", e),
            })
    }
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
    PasskeyAuthenticator,
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
            SignatureScheme::PasskeyAuthenticator => 0x06,
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
            0x06 => Ok(SignatureScheme::PasskeyAuthenticator),
            _ => Err(IotaError::KeyConversion("Invalid key scheme".to_string())),
        }
    }
}

impl FromStr for Signature {
    type Err = eyre::Report;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::decode_base64(s).map_err(|e| eyre!("Fail to decode base64 {}", e.to_string()))
    }
}