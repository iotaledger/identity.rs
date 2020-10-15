use identity_crypto::{self as crypto, PublicKey, SecretKey, Sign, Verify};

use crate::{
    error::{Error, Result},
    jws::{self, Algorithm::EdDSA},
    signature::{SignatureSuite, SignatureValue},
};

/// An implementation of the 2018 Ed25519 Signature Suite
///
/// TODO: Use Urdna2015 canonicalization
///
/// Note: This doesn't use sha512 as the reference implementations use sha256.
/// [More Info](https://github.com/w3c-ccg/security-vocab/issues/21)
///
/// [Specification](https://w3c-ccg.github.io/lds-ed25519-2018/)
///
/// [Vocabulary Definition](https://w3c-ccg.github.io/security-vocab/#Ed25519Signature2018)
#[derive(Clone, Copy, Debug)]
pub struct Ed25519Signature2018;

impl Sign for Ed25519Signature2018 {
    fn sign(&self, message: &[u8], secret: &SecretKey) -> crypto::Result<Vec<u8>> {
        jws::create_detached(EdDSA, message, secret).map_err(|error| crypto::Error::SignError(error.into()))
    }
}

impl Verify for Ed25519Signature2018 {
    fn verify(&self, message: &[u8], signature: &[u8], public: &PublicKey) -> crypto::Result<bool> {
        jws::verify_detached(EdDSA, message, signature, public)
            .map_err(|error| crypto::Error::VerifyError(error.into()))
    }
}

impl SignatureSuite for Ed25519Signature2018 {
    fn signature(&self) -> &'static str {
        "Ed25519Signature2018"
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
