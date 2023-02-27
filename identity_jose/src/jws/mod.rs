// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! This module features a high-level encoding/decoding API to create JSON Web Signatures ([JWS](https://tools.ietf.org/html/rfc7515)).
//!
//! # Example
//!
//! In this example we encode and decode a JWS using the Ed25519 signature scheme.
//!
//! ```rust
//! # use std::sync::Arc;
//! # use std::time::SystemTime;
//! # use crypto::signatures::ed25519;
//! # use crypto::signatures::ed25519::SecretKey;
//! # use identity_jose::jws::Decoder;
//! # use identity_jose::jws::JWSValidationConfig;
//! # use identity_jose::jws::VerificationInput;
//! # use identity_jose::jws::Encoder;
//! # use identity_jose::jws::JwsAlgorithm;
//! # use identity_jose::jws::JwsHeader;
//! # use identity_jose::jws::Recipient;
//! # use identity_jose::jwt::JwtClaims;
//! # use identity_jose::jwt::JwtHeaderSet;
//! # use identity_jose::jwu;
//! # async fn _jws_example() -> Result<(), Box<dyn std::error::Error>> {
//! // =============================
//! // Generate an Ed25519 key pair
//! // =============================
//! let secret_key = SecretKey::generate()?;
//! let public_key = secret_key.public_key();
//!
//! // ====================================
//! // Create the header for the recipient
//! // ====================================
//! let mut header: JwsHeader = JwsHeader::new();
//! header.set_alg(JwsAlgorithm::EdDSA);
//! header.set_kid("did:iota:0x123#signing-key");
//!
//! // ==================================
//! // Create the claims we want to sign
//! // ==================================
//! let mut claims: JwtClaims<serde_json::Value> = JwtClaims::new();
//! claims.set_iss("issuer");
//! claims.set_iat(
//!   SystemTime::now()
//!     .duration_since(SystemTime::UNIX_EPOCH)?
//!     .as_secs() as i64,
//! );
//! claims.set_custom(serde_json::json!({"num": 42u64}));
//!
//! // ==================
//! // Encode the claims
//! // ==================
//! let encoder: Encoder = Encoder::new().recipient(Recipient::new().protected(&header));
//! let claims_bytes: Vec<u8> = serde_json::to_vec(&claims)?;
//! let secret_key: Arc<SecretKey> = Arc::new(secret_key);
//! let sign_fn = move |protected: Option<JwsHeader>, unprotected: Option<JwsHeader>, msg: Vec<u8>| {
//!   let sk: Arc<SecretKey> = secret_key.clone();
//!   async move {
//!     let header_set: JwtHeaderSet<JwsHeader> = JwtHeaderSet::new().with_protected(&protected).with_unprotected(&unprotected);
//!     if header_set.try_alg().map_err(|_| "missing `alg` parameter")? != JwsAlgorithm::EdDSA {
//!       return Err("incompatible `alg` parameter");
//!     }
//!     let sig: [u8; ed25519::SIGNATURE_LENGTH] = sk.sign(msg.as_slice()).to_bytes();
//!     Ok(jwu::encode_b64(sig))
//!   }
//! };
//! let token: String = encoder.encode(&sign_fn, &claims_bytes).await?;
//!
//! // ==================
//! // Decode the claims
//! // ==================
//!
//! // Set up a verifier that verifies JWS signatures secured with the Ed25519 algorithm
//! let verify_fn = JwsSignatureVerifierFn::from(|verification_input: &VerificationInput , jwk: &Jwk| -> Result<(), JwsVerifierError> {
//!   if verification_input.jose_header().alg().filter(|value| value == JwsAlgorithm::EdDSA).is_none() {
//!     return Err(JwsVerifierErrorKind::InvalidAlg.into());
//!   }
//!
//!    let params: &JwkParamsOkp = jwk.try_okp_params().map_err(|_| JwsVerifierErrorKind::UnsupportedKeyType)?;
//!
//!  if params.try_ed_curve().unwrap() != EdCurve::Ed25519 {
//!     return Err(JwsVerifierErrorKind::UnsupportedKeyParams);
//!  }
//!
//!  let pk: [u8; ed25519::PUBLIC_KEY_LENGTH] = jwu::decode_b64(params.x.as_str()).unwrap().try_into().unwrap();
//!
//!   PublicKey::try_from(pk).unwrap()  
//!   let signature_arr = <[u8; ed25519::SIGNATURE_LENGTH]>::try_from(verification_input.signature())
//!     .map_err(|err| JwsVerifierErrorKind::InvalidSignature)
//!     ?;
//!   let signature = ed25519::Signature::from_bytes(signature_arr);
//!   if public_key.verify(&signature, verification_input.signing_input()) {
//!     Ok(())
//!   } else {
//!     Err(JwsVerifierErrorKind::InvalidSignature.into())
//!   }
//! });
//! let decoder = Decoder::new(verify_fn);
//! let token = decoder.decode(token.as_bytes(), None)?;
//!
//! // ==================================
//! // Assert the claims are as expected
//! // ==================================
//! let recovered_claims: JwtClaims<serde_json::Value> = serde_json::from_slice(&token.claims)?;
//! assert_eq!(claims, recovered_claims);
//! # Ok(())
//! # }
//! ```

mod algorithm;
mod charset;
mod decoder;
mod encoder;
mod format;
mod header;
mod jws_verifier;
mod recipient;

pub use self::algorithm::*;
pub use self::charset::*;
pub use self::decoder::*;
pub use self::encoder::*;
pub use self::format::*;
pub use self::header::*;
pub use self::jws_verifier::*;
pub use self::recipient::*;
