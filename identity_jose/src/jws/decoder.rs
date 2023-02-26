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

use self::default_verification::DefaultJwsVerifier;

use super::JwsSignatureVerifier;
use super::JwsVerifierError;
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
pub struct Decoder<T = DefaultJwsVerifier>
where
  T: JwsSignatureVerifier,
{
  /// The expected format of the encoded token.
  format: JwsFormat,
  config: JWSValidationConfig,
  verifier: T,
}

impl<'a, 'b, T> Decoder<T>
where
  T: JwsSignatureVerifier,
{
  pub fn new(verifier: T) -> Self {
    Self {
      format: JwsFormat::Compact,
      config: JWSValidationConfig::default(),
      verifier,
    }
  }

  pub fn format(mut self, value: JwsFormat) -> Self {
    self.format = value;
    self
  }

  /// Decode the given `data` which is a base64url-encoded JWS.
  ///
  /// The `jwk_provider` is a closure taking the `kid` extracted from a JWS header and returning the corresponding JWK
  /// if possible.
  ///
  /// If using `detached_payload` one should supply a `Some` value for the `detached_payload` parameter.
  /// [More Info](https://tools.ietf.org/html/rfc7515#appendix-F)
  pub fn decode<F>(&self, data: &'b [u8], jwk_provider: &F, detached_payload: Option<&'b [u8]>) -> Result<Token<'b>>
  where
    F: Fn(&str) -> Option<&Jwk>,
  {
    self.expand(data, detached_payload, |payload, signatures| {
      let mut result: Result<Token> = Err(Error::InvalidContent("recipient not found"));

      for signature in signatures {
        result = self.decode_one(jwk_provider, payload, signature);
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
  }

  fn decode_one<F>(&self, jwk_provider: F, payload: &'b [u8], jws_signature: JwsSignature<'a>) -> Result<Token<'b>>
  where
    F: Fn(&str) -> Option<&Jwk>,
  {
    let protected: Option<JwsHeader> = jws_signature.protected.map(decode_b64_json).transpose()?;

    validate_jws_headers(
      protected.as_ref(),
      jws_signature.header.as_ref(),
      self.config.crits.as_deref(),
    )?;

    let merged: HeaderSet<'_> = HeaderSet::new()
      .with_protected(protected.as_ref())
      .with_unprotected(jws_signature.header.as_ref());
    let alg: JwsAlgorithm = merged.try_alg()?;

    let payload_is_b64_encoded = merged.b64().unwrap_or(true);

    let kid = merged.kid();

    // Obtain a JWK before proceeding.

    // TODO: Better error.
    let key: &Jwk = kid
      .and_then(jwk_provider)
      .or_else(|| {
        if self.config.fallback_to_jwk_header {
          merged.jwk()
        } else {
          None
        }
      })
      .ok_or_else(|| crate::error::Error::SignatureVerificationError("could not obtain public key".into()))?;

    // Validate the header's alg against the requirements of the JWK.
    {
      if let Some(key_alg) = key.alg() {
        if alg.name() != key_alg {
          return Err(crate::error::Error::SignatureVerificationError(
            "algorithm mismatch between jwk and jws header".into(),
          ));
        }
      } else if self.config.jwk_must_have_alg {
        return Err(crate::error::Error::SignatureVerificationError(
          "the jwk is missing the alg parameter required by the given config".into(),
        ));
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
        .map_err(|err| Error::SignatureVerificationError(err.into()))?;
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

  fn expand<S>(
    &self,
    data: &'b [u8],
    detached_payload: Option<&'b [u8]>,
    format: impl Fn(&'b [u8], Vec<JwsSignature<'_>>) -> Result<S>,
  ) -> Result<S> {
    match self.format {
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

        format(Self::expand_payload(detached_payload, Some(split[1]))?, vec![signature])
      }
      JwsFormat::General => {
        let data: General<'_> = serde_json::from_slice(data).map_err(Error::InvalidJson)?;

        format(Self::expand_payload(detached_payload, data.payload)?, data.signatures)
      }
      JwsFormat::Flatten => {
        let data: Flatten<'_> = serde_json::from_slice(data).map_err(Error::InvalidJson)?;

        format(
          Self::expand_payload(detached_payload, data.payload)?,
          vec![data.signature],
        )
      }
    }
  }

  fn expand_payload(
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

pub mod default_verification {
  use super::*;

  #[non_exhaustive]
  pub struct DefaultJwsVerifier;

  impl JwsSignatureVerifier for DefaultJwsVerifier {
    #[cfg(feature = "default-jws-signature-verifier")]
    fn verify(&self, input: &VerificationInput<'_>, public_key: &Jwk) -> std::result::Result<(), JwsVerifierError> {
      let alg = input.jose_header().alg().ok_or(JwsVerifierError::UnsupportedAlg)?;
      match alg {
        // EdDSA is the only supported algorithm for now, we can consider supporting more by default in the future.
        JwsAlgorithm::EdDSA => DefaultJwsVerifier::verify_eddsa_jws_prechecked_alg(input, public_key),
        _ => Err(JwsVerifierError::UnsupportedAlg),
      }
    }

    #[cfg(not(feature = "default-jws-signature-verifier"))]
    fn verify(&self, input: &VerificationInput<'_>, public_key: &Jwk) -> std::result::Result<(), JwsVerifierError> {
      panic!("it should not be possible to construct DefaultJwsVerifier without the 'default-jws-signature-verifier' feature. We encourage you to report this bug at: https://github.com/iotaledger/identity.rs/issues");
    }
  }

  #[cfg(feature = "default-jws-signature-verifier")]
  mod default_impls {
    use crate::jwk::EdCurve;
    use crate::jwk::JwkParamsOkp;

    use super::*;

    impl DefaultJwsVerifier {
      /// Verify a JWS signature secured with the EdDsa algorithm.
      /// Only [`EdCurve::Ed25519`] is supported for now.  
      pub fn verify_eddsa_jws_prechecked_alg(
        input: &VerificationInput<'_>,
        public_key: &Jwk,
      ) -> Result<(), JwsVerifierError> {
        // Obtain an Ed25519 public key
        let params: &JwkParamsOkp = public_key
          .try_okp_params()
          .map_err(|_| JwsVerifierError::UnsupportedKeyType)?;

        if params
          .try_ed_curve()
          .ok()
          .filter(|curve_param| *curve_param == EdCurve::Ed25519)
          .is_none()
        {
          return Err(JwsVerifierError::UnsupportedKeyParams);
        }

        let pk: [u8; crypto::signatures::ed25519::PUBLIC_KEY_LENGTH] = crate::jwu::decode_b64(params.x.as_str())
          .map_err(|_| JwsVerifierError::Unspecified("could not extract Ed25519 public key".into()))
          .and_then(|value| TryInto::try_into(value).map_err(|_| JwsVerifierError::UnsupportedKeyParams))?;

        let public_key_ed25519 =
          crypto::signatures::ed25519::PublicKey::try_from(pk).map_err(|_| JwsVerifierError::UnsupportedKeyParams)?;

        let signature_arr = <[u8; crypto::signatures::ed25519::SIGNATURE_LENGTH]>::try_from(input.signature())
          .map_err(|err| err.to_string())
          .unwrap();

        let signature = crypto::signatures::ed25519::Signature::from_bytes(signature_arr);

        if crypto::signatures::ed25519::PublicKey::verify(&public_key_ed25519, &signature, input.signing_input()) {
          Ok(())
        } else {
          Err(JwsVerifierError::SignatureVerificationError(
            "could not verify Ed25519 signature".into(),
          ))
        }
      }
    }

    impl Default for Decoder {
      fn default() -> Self {
        Decoder::new(DefaultJwsVerifier)
      }
    }
  }
}
