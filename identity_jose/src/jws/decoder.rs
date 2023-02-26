// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::str;
use std::borrow::Cow;

use crate::error::Error;
use crate::error::Result;
use crate::jws::JwsAlgorithm;
use crate::jws::JwsFormat;
use crate::jws::JwsHeader;
use crate::jwt::JwtHeaderSet;
use crate::jwu::check_slice_param;
use crate::jwu::create_message;
use crate::jwu::decode_b64;
use crate::jwu::decode_b64_json;
use crate::jwu::filter_non_empty_bytes;
use crate::jwu::parse_utf8;
use crate::jwu::validate_jws_headers;

type HeaderSet<'a> = JwtHeaderSet<'a, JwsHeader>;

/// The protected JWS header.
pub type DecoderProtectedHeader<'a> = &'a JwsHeader;
/// The unprotected JWS header.
pub type DecoderUnprotectedHeader<'a> = &'a JwsHeader;
/// The message to sign as a slice.
pub type DecoderMessage<'a> = &'a [u8];
/// The signature as a slice.
pub type DecoderSignature<'a> = &'a [u8];

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

/// Input intended for an `alg` specific
/// JWS verifier.
pub struct VerificationInput<'a> {
  jose_header: HeaderSet<'a>,
  signing_input: Vec<u8>,
  signature: &'a [u8],
}

impl<'a> VerificationInput<'a> {
  pub fn jose_header(&self) -> &HeaderSet<'a> {
    &self.jose_header
  }

  pub fn signing_input(&self) -> &[u8] {
    self.signing_input.as_ref()
  }

  pub fn signature(&self) -> &'a [u8] {
    self.signature
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
///
/// When attempting to decode a raw JWS, the decoder will check for the expected format, algorithms
/// and crits which can be set via their respective setters.
///
/// This API does not have any cryptography built-in. Rather, signatures are verified
/// through a closure that is passed to the [`decode`](Decoder::decode) method, so that users can implement
/// verification for their particular algorithm.
pub struct Decoder {
  /// The expected format of the encoded token.
  format: JwsFormat,
  /// A list of permitted signature algorithms.
  algs: Option<Vec<JwsAlgorithm>>,
  /// A list of permitted extension parameters.
  crits: Option<Vec<String>>,
}

impl<'a, 'b> Decoder {
  pub fn new() -> Self {
    Self {
      format: JwsFormat::Compact,

      algs: None,
      crits: None,
    }
  }

  pub fn format(mut self, value: JwsFormat) -> Self {
    self.format = value;
    self
  }

  pub fn algorithm(mut self, value: JwsAlgorithm) -> Self {
    self.algs.get_or_insert_with(Vec::new).push(value);
    self
  }

  pub fn critical(mut self, value: impl Into<String>) -> Self {
    self.crits.get_or_insert_with(Vec::new).push(value.into());
    self
  }

  /// Decode the given `data` which is a base64url-encoded JWS.
  ///
  /// The `verify_fn` closure must be provided to verify signatures on the JWS.
  /// Only one signature on a JWS is required to be verified successfully by `verify_fn`
  /// in order for the entire JWS to be considered valid, hence `verify_fn` can error
  /// on signatures it cannot verify. `verify_fn` must return an error if signature verification
  /// fails or if the `alg` parameter in the header describes a cryptographic algorithm that it cannot handle.
  ///
  /// If using `detached_payload` one should supply a `Some` value for the `detached_payload` parameter.
  /// [More Info](https://tools.ietf.org/html/rfc7515#appendix-F)
  pub fn decode_with<FUN, ERR>(
    &self,
    verify_fn: &FUN,
    data: &'b [u8],
    detached_payload: Option<&'b [u8]>,
  ) -> Result<Token<'b>>
  where
    FUN: Fn(&VerificationInput<'_>) -> std::result::Result<(), ERR>,
    ERR: Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
  {
    self.expand(data, detached_payload, |payload, signatures| {
      for signature in signatures {
        if let Ok(token) = self.decode_one(verify_fn, payload, signature) {
          return Ok(token);
        }
      }

      // TODO: Improve error by somehow including the error returned by decode_one if it exists.
      Err(Error::InvalidContent("recipient not found"))
    })
  }

  fn decode_one<FUN, ERR>(
    &self,
    verify_fn: &FUN,
    payload: &'b [u8],
    jws_signature: JwsSignature<'a>,
  ) -> Result<Token<'b>>
  where
    FUN: Fn(&VerificationInput<'_>) -> std::result::Result<(), ERR>,
    ERR: Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
  {
    let protected: Option<JwsHeader> = jws_signature.protected.map(decode_b64_json).transpose()?;

    validate_jws_headers(protected.as_ref(), jws_signature.header.as_ref(), self.crits.as_deref())?;

    let merged: HeaderSet<'_> = HeaderSet::new()
      .with_protected(protected.as_ref())
      .with_unprotected(jws_signature.header.as_ref());
    let alg: JwsAlgorithm = merged.try_alg()?;

    self.check_alg(alg)?;
    let payload_is_b64_encoded = merged.b64().unwrap_or(true);

    // Verify the signature
    {
      let protected_bytes: &[u8] = jws_signature.protected.map(str::as_bytes).unwrap_or_default();
      let message: Vec<u8> = create_message(protected_bytes, payload);
      let signature: Vec<u8> = decode_b64(jws_signature.signature)?;
      let verification_input = VerificationInput {
        jose_header: merged,
        signing_input: message,
        signature: &signature,
      };

      verify_fn(&verification_input).map_err(|err| Error::SignatureVerificationError(err.into()))?;
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

  fn expand<T>(
    &self,
    data: &'b [u8],
    detached_payload: Option<&'b [u8]>,
    format: impl Fn(&'b [u8], Vec<JwsSignature<'_>>) -> Result<T>,
  ) -> Result<T> {
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
    match (detached_payload, filter_non_empty_bytes(parsed_payload)) {
      (Some(payload), None) => Ok(payload),
      (None, Some(payload)) => Ok(payload),
      (Some(_), Some(_)) => Err(Error::InvalidContent("multiple payloads")),
      (None, None) => Err(Error::InvalidContent("missing payload")),
    }
  }

  fn check_alg(&self, value: JwsAlgorithm) -> Result<()> {
    check_slice_param("alg", self.algs.as_deref(), &value)
  }
}

impl<'a> Default for Decoder {
  fn default() -> Self {
    Self::new()
  }
}
