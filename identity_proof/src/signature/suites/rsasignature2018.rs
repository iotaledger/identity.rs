use identity_crypto::{self as crypto, PublicKey, SecretKey, Sign, Verify};

use crate::{
    error::{Error, Result},
    jws::{self, Algorithm::PS256},
    signature::{SignatureSuite, SignatureValue},
};

/// An implementation of the 2018 RSA Signature Suite
///
/// TODO: Use Urdna2015 canonicalization
///
/// Note: PS256 is used by reference implementations although the algorithm
/// is specified in the spec as RS256
///
/// [Specification](https://w3c-ccg.github.io/lds-rsa2018/)
///
/// [Vocabulary Definition](https://w3c-ccg.github.io/security-vocab/#RsaSignature2018)
#[derive(Clone, Copy, Debug)]
pub struct RsaSignature2018;

impl Sign for RsaSignature2018 {
    fn sign(&self, message: &[u8], secret: &SecretKey) -> crypto::Result<Vec<u8>> {
        jws::create_detached(PS256, message, secret).map_err(|error| crypto::Error::SignError(error.into()))
    }
}

impl Verify for RsaSignature2018 {
    fn verify(&self, message: &[u8], signature: &[u8], public: &PublicKey) -> crypto::Result<bool> {
        jws::verify_detached(PS256, message, signature, public)
            .map_err(|error| crypto::Error::VerifyError(error.into()))
    }
}

impl SignatureSuite for RsaSignature2018 {
    fn signature(&self) -> &'static str {
        "RsaSignature2018"
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
