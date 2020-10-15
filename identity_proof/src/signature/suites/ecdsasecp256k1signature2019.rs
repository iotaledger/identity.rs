use identity_crypto::{self as crypto, PublicKey, SecretKey, Sign, Verify};

use crate::{
    error::{Error, Result},
    jws::{self, Algorithm::ES256K},
    signature::{SignatureSuite, SignatureValue},
};

/// An implementation of the 2019 Ecdsa Secp256k1 Signature Suite
///
/// TODO: Use Urdna2015 canonicalization
///
/// [Specification](https://w3c-ccg.github.io/lds-ecdsa-secp256k1-2019/)
///
/// [Vocabulary Definition](https://w3c-ccg.github.io/security-vocab/#EcdsaSecp256k1Signature2019)
#[derive(Clone, Copy, Debug)]
pub struct EcdsaSecp256k1Signature2019;

impl Sign for EcdsaSecp256k1Signature2019 {
    fn sign(&self, message: &[u8], secret: &SecretKey) -> crypto::Result<Vec<u8>> {
        jws::create_detached(ES256K, message, secret).map_err(|error| crypto::Error::SignError(error.into()))
    }
}

impl Verify for EcdsaSecp256k1Signature2019 {
    fn verify(&self, message: &[u8], signature: &[u8], public: &PublicKey) -> crypto::Result<bool> {
        jws::verify_detached(ES256K, message, signature, public)
            .map_err(|error| crypto::Error::VerifyError(error.into()))
    }
}

impl SignatureSuite for EcdsaSecp256k1Signature2019 {
    fn signature(&self) -> &'static str {
        "EcdsaSecp256k1Signature2019"
    }

    fn to_signature_value(&self, signature: Vec<u8>) -> Result<SignatureValue> {
        String::from_utf8(signature)
            .map(SignatureValue::Jws)
            .map_err(|error| Error::Custom(error.into()))
    }

    fn from_signature_value(&self, signature: &str) -> Result<Vec<u8>> {
        Ok(signature.as_bytes().to_vec())
    }
}
