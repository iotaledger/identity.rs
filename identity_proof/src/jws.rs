use identity_core::{
    common::AsJson as _,
    utils::{decode_b64, encode_b64},
};
use identity_crypto::{self as crypto, PublicKey, SecretKey, Sign, Verify};
use serde::{Deserialize, Serialize};
use std::str::from_utf8;

use crate::error::{Error, Result};

const DETACHED_JWS_LEN: usize = 3;
const DETACHED_JWS_HEADER: usize = 0;
const DETACHED_JWS_SIGNATURE: usize = 2;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Deserialize, Serialize)]
pub enum Algorithm {
    HS256,  // HMAC using SHA-256
    HS384,  // HMAC using SHA-384
    HS512,  // HMAC using SHA-512
    RS256,  // RSASSA-PKCS1-v1_5 using SHA-256
    RS384,  // RSASSA-PKCS1-v1_5 using SHA-384
    RS512,  // RSASSA-PKCS1-v1_5 using SHA-512
    PS256,  // RSASSA-PSS using SHA-256 and MGF1 with SHA-256
    PS384,  // RSASSA-PSS using SHA-384 and MGF1 with SHA-384
    PS512,  // RSASSA-PSS using SHA-512 and MGF1 with SHA-512
    ES256,  // ECDSA using P-256 and SHA-256
    ES384,  // ECDSA using P-384 and SHA-384
    ES512,  // ECDSA using P-521 and SHA-512
    ES256K, // ECDSA using secp256k1 curve and SHA-256
    EdDSA,  // EdDSA using Ed25519 curve
}

impl Algorithm {
    pub fn sign(self, message: &[u8], secret: &SecretKey) -> Result<Vec<u8>> {
        let signature = match self {
            Self::HS256 => crypto::HmacSha256.sign(message, secret)?,
            Self::HS384 => crypto::HmacSha384.sign(message, secret)?,
            Self::HS512 => crypto::HmacSha512.sign(message, secret)?,
            Self::RS256 => crypto::RsaPkcs1Sha256.sign(message, secret)?,
            Self::RS384 => crypto::RsaPkcs1Sha384.sign(message, secret)?,
            Self::RS512 => crypto::RsaPkcs1Sha512.sign(message, secret)?,
            Self::PS256 => crypto::RsaPssSha256.sign(message, secret)?,
            Self::PS384 => crypto::RsaPssSha384.sign(message, secret)?,
            Self::PS512 => crypto::RsaPssSha512.sign(message, secret)?,
            Self::ES256 => crypto::EcdsaP256Sha256.sign(message, secret)?,
            Self::ES384 => crypto::EcdsaP384Sha384.sign(message, secret)?,
            Self::ES512 => todo!("Implement ES512::sign"),
            Self::ES256K => crypto::Secp256k1.sign(message, secret)?,
            Self::EdDSA => crypto::Ed25519.sign(message, secret)?,
        };

        Ok(signature)
    }

    pub fn verify(self, message: &[u8], signature: &[u8], public: &PublicKey) -> Result<bool> {
        let result = match self {
            Self::HS256 => crypto::HmacSha256.verify(message, signature, public)?,
            Self::HS384 => crypto::HmacSha384.verify(message, signature, public)?,
            Self::HS512 => crypto::HmacSha512.verify(message, signature, public)?,
            Self::RS256 => crypto::RsaPkcs1Sha256.verify(message, signature, public)?,
            Self::RS384 => crypto::RsaPkcs1Sha384.verify(message, signature, public)?,
            Self::RS512 => crypto::RsaPkcs1Sha512.verify(message, signature, public)?,
            Self::PS256 => crypto::RsaPssSha256.verify(message, signature, public)?,
            Self::PS384 => crypto::RsaPssSha384.verify(message, signature, public)?,
            Self::PS512 => crypto::RsaPssSha512.verify(message, signature, public)?,
            Self::ES256 => crypto::EcdsaP256Sha256.verify(message, signature, public)?,
            Self::ES384 => crypto::EcdsaP384Sha384.verify(message, signature, public)?,
            Self::ES512 => todo!("Implement ES512::verify"),
            Self::ES256K => crypto::Secp256k1.verify(message, signature, public)?,
            Self::EdDSA => crypto::Ed25519.verify(message, signature, public)?,
        };

        Ok(result)
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct JwsHeader {
    alg: Algorithm,
    #[serde(default = "JwsHeader::default_b64")]
    b64: bool,
    #[serde(default)]
    crit: Vec<String>,
}

impl JwsHeader {
    const B64: &'static str = "b64";

    fn new(alg: Algorithm) -> Self {
        Self {
            alg,
            b64: false,
            crit: vec![Self::B64.into()],
        }
    }

    fn verify(&self, algorithm: Algorithm) -> Result<()> {
        if self.alg != algorithm {
            return Err(Error::InvalidLDSignature("JWS".into()));
        }

        if self.crit.len() != 1 || self.crit[0] != Self::B64 {
            return Err(Error::InvalidLDSignature("JWS".into()));
        }

        if self.b64 {
            return Err(Error::InvalidLDSignature("JWS".into()));
        }

        Ok(())
    }

    pub fn verify_slice(algorithm: Algorithm, slice: &[u8]) -> Result<()> {
        Self::from_json_slice(slice)?.verify(algorithm)
    }

    fn default_b64() -> bool {
        true
    }
}

// Create a raw (bytes) JWS header for the given `algorithm`.
fn create_raw_header(algorithm: Algorithm) -> Result<Vec<u8>> {
    JwsHeader::new(algorithm).to_json_vec().map_err(Into::into)
}

// Create a base64 encoded JWS header for the given `algorithm`.
fn create_b64_header(algorithm: Algorithm) -> Result<String> {
    create_raw_header(algorithm).map(|header| encode_b64(&header))
}

// Creates a JWS signature payload in the following form:
//   <header>.<payload>
fn create_payload(header: &[u8], message: &[u8]) -> Vec<u8> {
    let mut output = Vec::with_capacity(header.len() + 1 + message.len());
    output.extend_from_slice(header);
    output.push(b'.');
    output.extend_from_slice(message);
    output
}

// Creates a JWS signature in the following form:
//   <header>..<signature>
//
// Note: <payload> is excluded as this is a detached signature
fn create_jws(header: &[u8], signature: &str) -> Vec<u8> {
    let mut output = Vec::with_capacity(header.len() + 2 + signature.len());
    output.extend_from_slice(header);
    output.push(b'.');
    output.push(b'.');
    output.extend_from_slice(signature.as_bytes());
    output
}

// Extracts the JWS header and signature from the given detached `signature`.
fn parse_detached(signature: &[u8]) -> Result<(&str, &str)> {
    let sig: &str = from_utf8(signature).map_err(|error| Error::Custom(error.into()))?;
    let jws: Vec<&str> = sig.split('.').collect();

    if jws.len() != DETACHED_JWS_LEN {
        return Err(Error::InvalidLDSignature("JWS".into()));
    }

    let header = jws.get(DETACHED_JWS_HEADER).expect("infallible");
    let signature = jws.get(DETACHED_JWS_SIGNATURE).expect("infallible");

    Ok((header, signature))
}

/// Creates a detached JWS signature with an unencoded payload.
///
/// See [rfc7515](https://tools.ietf.org/html/rfc7515#appendix-F) for more info on detached signatures.
///
/// See [rfc7797](https://tools.ietf.org/html/rfc7797) for more info on unencoded payloads.
pub fn create_detached(algorithm: Algorithm, message: &[u8], secret: &SecretKey) -> Result<Vec<u8>> {
    let header = create_b64_header(algorithm)?;
    let payload = create_payload(header.as_bytes(), message);

    let signature = algorithm
        .sign(&payload, secret)
        .map_err(|error| crypto::Error::SignError(error.into()))?;
    let detached = create_jws(header.as_bytes(), &encode_b64(&signature));

    Ok(detached)
}

/// Verifies a detached JWS signature with an unencoded payload.
///
/// See [rfc7515](https://tools.ietf.org/html/rfc7515#appendix-F) for more info on detached signatures.
///
/// See [rfc7797](https://tools.ietf.org/html/rfc7797) for more info on unencoded payloads.
pub fn verify_detached(algorithm: Algorithm, message: &[u8], signature: &[u8], public: &PublicKey) -> Result<bool> {
    // Extract the header and signature values from encoded bytes
    let (header, signature) = parse_detached(signature)?;

    let decoded_header = decode_b64(header)?;
    let decoded_signature = decode_b64(signature)?;

    // Parse and validate the header
    JwsHeader::verify_slice(algorithm, &decoded_header)?;

    // Reconstruct the original payload
    let payload = create_payload(header.as_bytes(), message);

    // Verify the reconstructed payload
    let result = algorithm.verify(&payload, &decoded_signature, public)?;

    // Return a bool indicating if all is well.
    Ok(result)
}
