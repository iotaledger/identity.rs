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
use crate::jws::JwsHeaderSet;
use crate::jwu::create_message;
use crate::jwu::decode_b64;
use crate::jwu::decode_b64_json;
use crate::jwu::filter_non_empty_bytes;
use crate::jwu::parse_utf8;
use crate::jwu::validate_jws_headers;

use super::decoder_config::DecodingConfig;
#[cfg(feature = "default-jws-signature-verifier")]
use super::DefaultJwsSignatureVerifier;
use super::JwsSignatureVerifier;
use super::VerificationInput;

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

// =============================================================================================
// Format dependent deserializable helper structs used by the decoder
// =============================================================================================
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct JwsSignature<'a> {
  header: Option<JwsHeader>,
  protected: Option<&'a str>,
  signature: &'a str,
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
// Decoder
// =============================================================================

/// The [`Decoder`] allows decoding a raw JWS into a [`Token`], verifying
/// the structure of the JWS and its signature.
#[derive(Default)]
pub struct Decoder {
  config: DecodingConfig,
}

impl Decoder {
  /// Constructs a new [`Decoder`].
  pub fn new() -> Self {
    Self {
      config: DecodingConfig::default(),
    }
  }

  /// Append values to the list of permitted extension parameters.
  pub fn critical(mut self, value: impl Into<String>) -> Self {
    self.config.crits.get_or_insert_with(Vec::new).push(value.into());
    self
  }

  /// Defines whether a given [`Jwk`](crate::jwk::Jwk) used to verify a JWS,
  /// must have an `alg` parameter corresponding to the one extracted from the JWS header.
  /// This value is `true` by default.  
  pub fn jwk_must_have_alg(mut self, value: bool) -> Self {
    self.config.jwk_must_have_alg = value;
    self
  }

  /// When verifying a JWS encoded with the general JWS JSON serialization
  /// this value decides whether all signatures must be verified (the default behavior),
  /// otherwise only one signature needs to be verified in order for the entire JWS to be accepted.
  pub fn strict_signature_verification(mut self, value: bool) -> Self {
    self.config.strict_signature_verification = value;
    self
  }

  /// Specify the serialization format the [`Decoder`](crate::jws::Decoder) accepts. The default is
  /// [`JwsFormat::Compact`].
  pub fn format(mut self, value: JwsFormat) -> Self {
    self.config.format = value;
    self
  }

  /// Convenience method equivalent to [`Self::decode(data, &DefaultJwsSignatureVerifier::default(),<other
  /// parameters>)`](Self::decode()). This method is only available when the `default-jws-signature-verifier` feature
  /// is enabled.
  #[cfg(any(feature = "default-jws-signature-verifier", doc))]
  pub fn decode_default<'b, 'c, F>(
    &self,
    data: &'b [u8],
    jwk_provider: F,
    detached_payload: Option<&'b [u8]>,
  ) -> Result<Token<'b>>
  where
    F: 'c,
    F: Fn(&JwsHeaderSet<'_>, &mut bool) -> Option<&'c Jwk>,
  {
    self.decode(
      data,
      &DefaultJwsSignatureVerifier::default(),
      jwk_provider,
      detached_payload,
    )
  }

  /// Decode the given `data` which is a base64url-encoded JWS.
  ///
  /// The decoder will verify the JWS signature(s) using the provided [`JwsSignatureVerifier`].
  ///
  /// ### Jwk extraction
  /// The `jwk_provider` argument is a closure responsible for providing a suitable [`Jwk`] or dictating that it should
  /// be extracted from the header (by setting the provided `bool` to `true`). A suitable [`Jwk`] could be one matching
  /// the header's `kid`, or some default JWK whenever that is reasonable.
  ///
  /// ### Working with detached payloads
  /// If using `detached_payload` one should supply a `Some` value for the `detached_payload` parameter.
  /// [More Info](https://tools.ietf.org/html/rfc7515#appendix-F)
  pub fn decode<'b, 'c, T, F>(
    &self,
    data: &'b [u8],
    verifier: &T,
    jwk_provider: F,
    detached_payload: Option<&'b [u8]>,
  ) -> Result<Token<'b>>
  where
    F: 'c,
    F: Fn(&JwsHeaderSet<'_>, &mut bool) -> Option<&'c Jwk>,
    T: JwsSignatureVerifier,
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
      result = self.decode_one(verifier, &jwk_provider, payload, jws_signature);
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
  }

  fn decode_one<'a, 'b, 'c, T, F>(
    &self,
    verifier: &T,
    jwk_provider: &F,
    payload: &'b [u8],
    jws_signature: JwsSignature<'a>,
  ) -> Result<Token<'b>>
  where
    F: 'c,
    F: Fn(&JwsHeaderSet<'_>, &mut bool) -> Option<&'c Jwk>,
    T: JwsSignatureVerifier,
  {
    let JwsSignature {
      header: unprotected_header,
      protected,
      signature,
    } = jws_signature;

    let protected_header: Option<JwsHeader> = protected.map(decode_b64_json).transpose()?;
    validate_jws_headers(
      protected_header.as_ref(),
      unprotected_header.as_ref(),
      self.config.crits.as_deref(),
    )?;

    let merged = JwsHeaderSet::new()
      .with_protected(protected_header.as_ref())
      .with_unprotected(unprotected_header.as_ref());

    let payload_is_b64_encoded = merged.b64().unwrap_or(true);

    // Obtain a JWK before proceeding.
    //let kid = merged.kid();
    let mut fallback_to_header = false;
    let provided_key: Option<&Jwk> = jwk_provider(&merged, &mut fallback_to_header);
    let key = provided_key
      .or_else(|| if fallback_to_header { merged.jwk() } else { None })
      .ok_or(Error::JwkNotProvided)?;

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
      let protected_bytes: &[u8] = protected.map(str::as_bytes).unwrap_or_default();
      let message: Vec<u8> = create_message(protected_bytes, payload);
      let signature: Vec<u8> = decode_b64(signature)?;
      let verification_input = VerificationInput {
        jose_header: &merged,
        signing_input: message,
        signature: &signature,
      };

      verifier
        .verify(&verification_input, key)
        .map_err(Error::SignatureVerificationError)?;
    }

    let claims: Cow<'b, [u8]> = if payload_is_b64_encoded {
      Cow::Owned(decode_b64(payload)?)
    } else {
      Cow::Borrowed(payload)
    };

    Ok(Token {
      protected: protected_header,
      unprotected: unprotected_header,
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

#[cfg(test)]
mod tests {
  use crate::jwk::Jwk;
  use crate::jwk::JwkType;
  use crate::jws::Decoder;
  use crate::jws::JwsAlgorithm;
  use crate::jws::JwsHeaderSet;
  use crate::jws::JwsSignatureVerifierFn;
  use crate::jws::VerificationInput;

  struct MockIssuer {
    id: String,
    keys: Vec<Jwk>,
  }

  // This is a very unrealistic test that is mainly designed to check that the `Decoder::decode` API will be callable
  // from the (JWS)-based PresentationValidator. Tests more relevant for the features of this crate can be found in
  // src/tests.
  #[test]
  fn compiles_with_advanced_jwk_provider() {
    let issuer_did: &str = "did:example:abfe13f712120431c276e12ecab";
    let kid: String = format!("{}#keys-1", issuer_did);
    let alg: String = JwsAlgorithm::RS256.to_string();
    let jws: &str = "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCIsImtpZCI6ImRpZDpleGFtcGxlOmFiZmUxM2Y3MTIxMjA0MzFjMjc2ZTEyZWNhYiNrZXlzLTEifQ.eyJzdWIiOiJkaWQ6ZXhhbXBsZTplYmZlYjFmNzEyZWJjNmYxYzI3NmUxMmVjMjEiLCJqdGkiOiJodHRwOi8vZXhhbXBsZS5lZHUvY3JlZGVudGlhbHMvMzczMiIsImlzcyI6ImRpZDpleGFtcGxlOmFiZmUxM2Y3MTIxMjA0MzFjMjc2ZTEyZWNhYiIsIm5iZiI6MTU0MTQ5MzcyNCwiZXhwIjoxNTczMDI5NzIzLCJub25jZSI6IjY2MCE2MzQ1RlNlciIsInZjIjp7IkBjb250ZXh0IjpbImh0dHBzOi8vdzMub3JnLzIwMTgvY3JlZGVudGlhbHMvdjEiLCJodHRwczovL2V4YW1wbGUuY29tL2V4YW1wbGVzL3YxIl0sInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiLCJVbml2ZXJzaXR5RGVncmVlQ3JlZGVudGlhbCJdLCJjcmVkZW50aWFsU3ViamVjdCI6eyJkZWdyZWUiOnsidHlwZSI6IkJhY2hlbG9yRGVncmVlIiwibmFtZSI6IkJhY2hlbG9yIG9mIFNjaWVuY2UgaW4gTWVjaGFuaWNhbCBFbmdpbmVlcmluZyJ9fX19.kaeFJM08sN7MthR-SWU-E8qbFoyZu2b_h1VllkEgNkLAGT9KpQbaeMUEti7QesFW_Cvwh5VErK62jneaW-uzZS6GPW3HVk8O3uRxWD3qCJx0l5uWZeHpRBX6yMcr2XGKWyFn0OBjoiGHQ78mHU8tNEWDqbrIhCoGQKj87OETvlfUDIkNi4_pRfLrJGh5HBrh6JuA-8uM2_clWC2RELsT52sPnqvMjm7UeYZgQEyaQJL6c41BUwHaCGWjUDCDZNWOd5M04s_Pi4Rqo97-2nbQRh_fuQk7aHKxb-UItQ8Mnk_hUFWEicwtuCfDFqwkZyW_r9dOBwz7-cOheuyP6OiLvw";

    let mut jwk: Jwk = Jwk::new(JwkType::Rsa);
    jwk.set_alg(alg);

    jwk.set_kid(kid);

    let issuer: MockIssuer = MockIssuer {
      id: issuer_did.into(),
      keys: vec![jwk],
    };

    let foo_issuer: MockIssuer = MockIssuer {
      id: "foo".into(),
      keys: Vec::new(),
    };
    let bar_issuer: MockIssuer = MockIssuer {
      id: "bar".into(),
      keys: vec![Jwk::new(JwkType::Ec)],
    };

    let issuers: Vec<MockIssuer> = vec![bar_issuer, foo_issuer, issuer];
    let issuers_slice: &[MockIssuer] = issuers.as_slice();

    let mock_verifier = JwsSignatureVerifierFn::from(|input: &VerificationInput, key: &Jwk| {
      key
        .alg()
        .filter(|alg| *alg == "RS256")
        .ok_or(crate::jws::SignatureVerificationErrorKind::UnsupportedAlg)?;
      if input.signature().len() > 42 {
        Ok(())
      } else {
        Err(crate::jws::SignatureVerificationErrorKind::InvalidSignature.into())
      }
    });

    let decoder = Decoder::new();

    let jwk_provider = |header: &JwsHeaderSet, fallback_to_header_jwk: &mut bool| -> Option<&Jwk> {
      let kid = header.kid()?;
      if header.jwk().filter(|jwk| jwk.kid() == Some(kid)).is_some() {
        *fallback_to_header_jwk = true;
        return None;
      }

      let did = &kid[..kid.rfind('#').unwrap()];
      issuers_slice
        .iter()
        .find(|entry| entry.id.as_str() == did)
        .and_then(|entry| entry.keys.iter().find(|key| key.kid() == Some(kid)))
    };

    assert!(decoder
      .decode(jws.as_bytes(), &mock_verifier, jwk_provider, None)
      .is_ok());
  }
}
