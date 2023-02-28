// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::str;
use std::borrow::Cow;

use crate::error::Error;
use crate::error::Result;
use crate::jwk::Jwk;
use crate::jws::JwsAlgorithm;
use crate::jws::JwsFormat;
use crate::jws::JwsHeader;
use crate::jwt::JwtHeaderSet;
use crate::jwu::create_message;
use crate::jwu::decode_b64;
use crate::jwu::decode_b64_json;
use crate::jwu::filter_non_empty_bytes;
use crate::jwu::parse_utf8;
use crate::jwu::validate_jws_headers;

use super::DefaultJwsSignatureVerifier;
use super::JwsSignatureVerifier;
use super::VerificationInput;

type HeaderSet<'a> = JwtHeaderSet<'a, JwsHeader>;

const COMPACT_SEGMENTS: usize = 3;

/// A partially decoded token from a JWS.
///
/// Contains the decoded headers and the raw claims.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Token<'a> {
  pub protected: Option<JwsHeader>,
  pub unprotected: Option<JwsHeader>,
  pub claims: Cow<'a, [u8]>,
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct JwsSignature<'a> {
  header: Option<JwsHeader>,
  protected: Option<&'a str>,
  signature: &'a str,
}

#[derive(Clone, Debug)]
/// Configuration defining the behaviour of a [`Decoder`].
pub struct JWSValidationConfig {
  crits: Option<Vec<String>>,

  jwk_must_have_alg: bool,

  strict_signature_verification: bool,

  format: JwsFormat,

  fallback_to_jwk_header: bool,
}

impl Default for JWSValidationConfig {
  fn default() -> Self {
    Self {
      crits: None,
      jwk_must_have_alg: true,
      strict_signature_verification: true,
      format: JwsFormat::Compact,
      fallback_to_jwk_header: false,
    }
  }
}

impl JWSValidationConfig {
  /// Append values to the list of permitted extension parameters.
  pub fn critical(mut self, value: impl Into<String>) -> Self {
    self.crits.get_or_insert_with(Vec::new).push(value.into());
    self
  }

  /// Defines whether a given [`Jwk`](crate::jwk::Jwk) used to verify a JWS,
  /// must have an `alg` parameter corresponding to the one extracted from the JWS header.
  /// This value is `true` by default.  
  pub fn jwk_must_have_alg(mut self, value: bool) -> Self {
    self.jwk_must_have_alg = value;
    self
  }

  /// When verifying a JWS encoded with the general JWS JSON serialization
  /// this value decides whether all signatures must be verified (the default behavior),
  /// otherwise only one signature needs to be verified in order for the entire JWS to be accepted.
  pub fn strict_signature_verification(mut self, value: bool) -> Self {
    self.strict_signature_verification = value;
    self
  }

  /// Specify the serialization format the `Decoder` accepts. The default is [`JwsFormat::Compact`].
  pub fn serialization_format(mut self, value: JwsFormat) -> Self {
    self.format = value;
    self
  }

  /// Specify whether to attempt to extract a public key from the JOSE header if the
  /// `jwk` provider fails to provide one.
  pub fn fallback_to_jwk_header(mut self, value: bool) -> Self {
    self.fallback_to_jwk_header = value;
    self
  }

  /// Specify which jws serialization format the [`Decoder`] should accept.
  pub fn format(mut self, value: JwsFormat) -> Self {
    self.format = value;
    self
  }
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct General<'a> {
  payload: Option<&'a str>,
  signatures: Vec<JwsSignature<'a>>,
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct Flatten<'a> {
  payload: Option<&'a str>,
  #[serde(flatten)]
  signature: JwsSignature<'a>,
}

// =============================================================================
// =============================================================================

/// The [`Decoder`] allows decoding a raw JWS into a [`Token`], verifying
/// the structure of the JWS and its signature.
pub struct Decoder<T = DefaultJwsSignatureVerifier>
where
  T: JwsSignatureVerifier,
{
  config: JWSValidationConfig,
  verifier: T,
}

impl<T> Decoder<T>
where
  T: JwsSignatureVerifier,
{
  pub fn new(verifier: T) -> Self {
    Self {
      config: JWSValidationConfig::default(),
      verifier,
    }
  }

  /// Set the [`JWSValidationConfig`] for the decoder.
  pub fn config(mut self, configuration: JWSValidationConfig) -> Self {
    self.config = configuration;
    self
  }
  /// Decode the given `data` which is a base64url-encoded JWS.
  ///
  /// The `jwk_provider` is a closure taking the `kid` extracted from a JWS header and returning the corresponding
  /// [`Jwk`] if possible. In the cases where a `kid` is not present in the JWS header the `jwk_provider` may return
  /// `None`, or some "default" JWK in the scenario where that is reasonable.  
  ///
  /// If using `detached_payload` one should supply a `Some` value for the `detached_payload` parameter.
  /// [More Info](https://tools.ietf.org/html/rfc7515#appendix-F)
  pub fn decode<'b, 'c, F>(
    &self,
    data: &'b [u8],
    jwk_provider: F,
    detached_payload: Option<&'b [u8]>,
  ) -> Result<Token<'b>>
  where
    F: 'c,
    F: Fn(Option<&str>) -> Option<&'c Jwk>,
  {
    // TODO: Only Vec in the general case, consider using OneOrMany or Either to remove the allocation for the other two
    // cases.
    let (payload, signatures): (&[u8], Vec<JwsSignature>) = match self.config.format {
      JwsFormat::Compact => {
        let split: Vec<&[u8]> = data.split(|byte| *byte == b'.').collect();

        if split.len() != COMPACT_SEGMENTS {
          return Err(Error::InvalidContent("invalid segments count"));
        }

        let signature: JwsSignature<'_> = JwsSignature {
          header: None,
          protected: Some(parse_utf8(split[0])?),
          signature: parse_utf8(split[2])?,
        };

        (Self::expand_payload(detached_payload, Some(split[1]))?, vec![signature])
      }
      JwsFormat::General => {
        let data: General<'_> = serde_json::from_slice(data).map_err(Error::InvalidJson)?;

        (Self::expand_payload(detached_payload, data.payload)?, data.signatures)
      }
      JwsFormat::Flatten => {
        let data: Flatten<'_> = serde_json::from_slice(data).map_err(Error::InvalidJson)?;

        (
          Self::expand_payload(detached_payload, data.payload)?,
          vec![data.signature],
        )
      }
    };

    let mut result: Result<Token> = Err(Error::InvalidContent("recipient not found"));

    for jws_signature in signatures {
      result = self.decode_one(&jwk_provider, payload, jws_signature);
      /*
        result =  {
        let protected: Option<JwsHeader> = jws_signature.protected.map(decode_b64_json).transpose()?;

        validate_jws_headers(
          protected.as_ref(),
          jws_signature.header.as_ref(),
          self.config.crits.as_deref(),
        )?;

        let merged = HeaderSet::new()
          .with_protected(protected.as_ref())
          .with_unprotected(jws_signature.header.as_ref());

        let payload_is_b64_encoded = merged.b64().unwrap_or(true);

        // Obtain a JWK before proceeding.

        let kid = merged.kid();
        let key: &Jwk = jwk_provider(kid)
          .ok_or_else(|| crate::error::Error::JwkNotProvided)?;

        // Validate the header's alg against the requirements of the JWK.
        let alg: JwsAlgorithm = merged.try_alg()?;
        {
          if let Some(key_alg) = key.alg() {
            if alg.name() != key_alg {
              return Err(crate::error::Error::AlgorithmMismatch);
            }
          } else if self.config.jwk_must_have_alg {
            return Err(crate::error::Error::JwkWithoutAlg);
          }
        }

        // Verify the signature
        {
          let protected_bytes: &[u8] = jws_signature.protected.map(str::as_bytes).unwrap_or_default();
          let message: Vec<u8> = create_message(protected_bytes, payload);
          let signature: Vec<u8> = decode_b64(jws_signature.signature)?;
          let verification_input = VerificationInput {
            jose_header: &merged,
            signing_input: message,
            signature: &signature,
          };

          self
            .verifier
            .verify(&verification_input, &key)
            .map_err(Error::SignatureVerificationError)?;
          drop(key);
        }

        let claims: Cow<'b, [u8]> = if payload_is_b64_encoded {
          Cow::Owned(decode_b64(payload)?)
        } else {
          Cow::Borrowed(payload)
        };

        Ok(Token {
          protected,
          unprotected: jws_signature.header,
          claims,
        })
      };
      */
      if result.is_err() && self.config.strict_signature_verification {
        // With strict signature verification all validations must be successful
        // hence we return on the first error discovered.
        return result;
      }
      if result.is_ok() && !self.config.strict_signature_verification {
        // If signature verification is not strict only one verification must succeed
        // hence we return on the first one.
        return result;
      }
    }
    result

    /*
    self.expand(data, detached_payload, |payload, signatures| {
      let mut result: Result<Token> = Err(Error::InvalidContent("recipient not found"));

      for signature in signatures {
        result = self.decode_one(&jwk_provider, payload, signature);
        if result.is_err() && self.config.strict_signature_verification {
          // With strict signature verification all validations must be successful
          // hence we return on the first error discovered.
          return result;
        }
        if result.is_ok() && !self.config.strict_signature_verification {
          // If signature verification is not strict only one verification must succeed
          // hence we return on the first one.
          return result;
        }
      }
      result
    })
    */
  }

  fn decode_one<'a, 'b, 'c, F>(
    &self,
    jwk_provider: &F,
    payload: &'b [u8],
    jws_signature: JwsSignature<'a>,
  ) -> Result<Token<'b>>
  where
    F: 'c,
    F: Fn(Option<&str>) -> Option<&'c Jwk>,
  {
    let protected: Option<JwsHeader> = jws_signature.protected.map(decode_b64_json).transpose()?;

    validate_jws_headers(
      protected.as_ref(),
      jws_signature.header.as_ref(),
      self.config.crits.as_deref(),
    )?;

    let merged = HeaderSet::new()
      .with_protected(protected.as_ref())
      .with_unprotected(jws_signature.header.as_ref());

    let payload_is_b64_encoded = merged.b64().unwrap_or(true);

    // Obtain a JWK before proceeding.

    let kid = merged.kid();
    let key: &Jwk = jwk_provider(kid).ok_or_else(|| crate::error::Error::JwkNotProvided)?;

    // Validate the header's alg against the requirements of the JWK.
    let alg: JwsAlgorithm = merged.try_alg()?;
    {
      if let Some(key_alg) = key.alg() {
        if alg.name() != key_alg {
          return Err(crate::error::Error::AlgorithmMismatch);
        }
      } else if self.config.jwk_must_have_alg {
        return Err(crate::error::Error::JwkWithoutAlg);
      }
    }

    // Verify the signature
    {
      let protected_bytes: &[u8] = jws_signature.protected.map(str::as_bytes).unwrap_or_default();
      let message: Vec<u8> = create_message(protected_bytes, payload);
      let signature: Vec<u8> = decode_b64(jws_signature.signature)?;
      let verification_input = VerificationInput {
        jose_header: &merged,
        signing_input: message,
        signature: &signature,
      };

      self
        .verifier
        .verify(&verification_input, &key)
        .map_err(Error::SignatureVerificationError)?;
      drop(key);
    }

    let claims: Cow<'b, [u8]> = if payload_is_b64_encoded {
      Cow::Owned(decode_b64(payload)?)
    } else {
      Cow::Borrowed(payload)
    };

    Ok(Token {
      protected,
      unprotected: jws_signature.header,
      claims,
    })
  }

  fn expand_payload<'b>(
    detached_payload: Option<&'b [u8]>,
    parsed_payload: Option<&'b (impl AsRef<[u8]> + ?Sized)>,
  ) -> Result<&'b [u8]> {
    //TODO: Do we allow an empty detached payload? (The previous stateful version did).
    match (detached_payload, filter_non_empty_bytes(parsed_payload)) {
      (Some(payload), None) => Ok(payload),
      (None, Some(payload)) => Ok(payload),
      (Some(_), Some(_)) => Err(Error::InvalidContent("multiple payloads")),
      (None, None) => Err(Error::InvalidContent("missing payload")),
    }
  }
}

#[cfg(any(feature = "default-jws-signature-verifier", doc))]
impl Default for Decoder {
  /// Default constructor for the [`Decoder`] capable of decoding
  /// a JWS secured with an algorithm the [`DefaultJwsSignatureVerifier`] can handle.
  /// This constructor is only available when the `default-jws-signature-verifier` is enabled.
  fn default() -> Self {
    Decoder::new(DefaultJwsSignatureVerifier::default())
  }
}

#[cfg(test)]
mod tests {

  use crate::jwk::EdCurve;
  use crate::jwk::Jwk;
  use crate::jwk::JwkParamsOkp;
  use crate::jwk::JwkType;
  use crate::jws::Decoder;
  use crate::jws::Encoder;
  use crate::jws::JwsAlgorithm;
  use crate::jws::JwsHeader;
  use crate::jws::JwsSignatureVerifierFn;
  use crate::jws::JwsVerifierError;
  use crate::jws::JwsVerifierErrorKind;
  use crate::jws::Recipient;
  use crate::jws::VerificationInput;
  use crate::jwt::JwtClaims;
  use crate::jwt::JwtHeaderSet;
  use crate::jwu;
  use crypto::signatures::ed25519;
  use crypto::signatures::ed25519::PublicKey;
  use crypto::signatures::ed25519::SecretKey;
  use std::sync::Arc;
  use std::time::SystemTime;

  async fn _jws_example() -> Result<(), Box<dyn std::error::Error>> {
    // =============================
    // Generate an Ed25519 key pair
    // =============================
    let secret_key = SecretKey::generate()?;
    let public_key = secret_key.public_key();

    // ====================================
    // Create the header for the recipient
    // ====================================
    let mut header: JwsHeader = JwsHeader::new();
    header.set_alg(JwsAlgorithm::EdDSA);
    let kid = "did:iota:0x123#signing-key";
    header.set_kid(kid);

    // ==================================
    // Create the claims we want to sign
    // ==================================
    let mut claims: JwtClaims<serde_json::Value> = JwtClaims::new();
    claims.set_iss("issuer");
    claims.set_iat(SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_secs() as i64);
    claims.set_custom(serde_json::json!({"num": 42u64}));

    // ==================
    // Encode the claims
    // ==================
    let encoder: Encoder = Encoder::new().recipient(Recipient::new().protected(&header));
    let claims_bytes: Vec<u8> = serde_json::to_vec(&claims)?;
    let secret_key: Arc<SecretKey> = Arc::new(secret_key);
    let sign_fn = move |protected: Option<JwsHeader>, unprotected: Option<JwsHeader>, msg: Vec<u8>| {
      let sk: Arc<SecretKey> = secret_key.clone();
      async move {
        let header_set: JwtHeaderSet<JwsHeader> = JwtHeaderSet::new()
          .with_protected(&protected)
          .with_unprotected(&unprotected);
        if header_set.try_alg().map_err(|_| "missing `alg` parameter")? != JwsAlgorithm::EdDSA {
          return Err("incompatible `alg` parameter");
        }
        let sig: [u8; ed25519::SIGNATURE_LENGTH] = sk.sign(msg.as_slice()).to_bytes();
        Ok(jwu::encode_b64(sig))
      }
    };
    let token: String = encoder.encode(&sign_fn, &claims_bytes).await?;

    // ============
    // Create Public key for verification
    // =============
    let mut public_key_jwk = Jwk::new(JwkType::Okp);
    public_key_jwk.set_kid(kid);
    public_key_jwk
      .set_params(JwkParamsOkp {
        crv: "Ed25519".into(),
        x: crate::jwu::encode_b64(public_key.as_slice()),
        d: None,
      })
      .unwrap();

    // ==================
    // Decode the claims
    // ==================

    // Set up a verifier that verifies JWS signatures secured with the Ed25519 algorithm
    let verify_fn = JwsSignatureVerifierFn::from(
      |verification_input: &VerificationInput, jwk: &Jwk| -> Result<(), JwsVerifierError> {
        if verification_input
          .jose_header()
          .alg()
          .filter(|value| *value == JwsAlgorithm::EdDSA)
          .is_none()
        {
          return Err(JwsVerifierErrorKind::UnsupportedAlg.into());
        }

        let params: &JwkParamsOkp = jwk
          .try_okp_params()
          .map_err(|_| JwsVerifierErrorKind::UnsupportedKeyType)?;

        if params.try_ed_curve().unwrap() != EdCurve::Ed25519 {
          return Err(JwsVerifierErrorKind::UnsupportedKeyParams.into());
        }

        let pk: [u8; ed25519::PUBLIC_KEY_LENGTH] = jwu::decode_b64(params.x.as_str()).unwrap().try_into().unwrap();

        let public_key = PublicKey::try_from(pk).map_err(|_| JwsVerifierErrorKind::KeyDecodingFailure)?;
        let signature_arr = <[u8; ed25519::SIGNATURE_LENGTH]>::try_from(verification_input.signature())
          .map_err(|err| JwsVerifierError::new(JwsVerifierErrorKind::InvalidSignature).with_source(err))?;
        let signature = ed25519::Signature::from_bytes(signature_arr);
        if public_key.verify(&signature, verification_input.signing_input()) {
          Ok(())
        } else {
          Err(JwsVerifierErrorKind::InvalidSignature.into())
        }
      },
    );
    let decoder = Decoder::new(verify_fn);
    // We don't use a detached payload.
    let detached_payload = None;
    let token = decoder.decode(token.as_bytes(), |_| Some(&public_key_jwk), detached_payload)?;

    // ==================================
    // Assert the claims are as expected
    // ==================================
    let recovered_claims: JwtClaims<serde_json::Value> = serde_json::from_slice(&token.claims)?;
    assert_eq!(claims, recovered_claims);
    Ok(())
  }
}
