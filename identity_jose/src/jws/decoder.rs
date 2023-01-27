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

pub struct Decoder<'b> {
  /// The expected format of the encoded token.
  format: JwsFormat,
  /// A list of permitted signature algorithms.
  algs: Option<Vec<JwsAlgorithm>>,
  /// A list of permitted extension parameters.
  crits: Option<Vec<String>>,
  /// The detached payload, if using detached content
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7515#appendix-F)
  payload: Option<&'b [u8]>,
}

impl<'a, 'b> Decoder<'b> {
  pub fn new() -> Self {
    Self {
      format: JwsFormat::Compact,

      algs: None,
      crits: None,
      payload: None,
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

  pub fn payload(mut self, value: &'b [u8]) -> Self {
    self.payload = Some(value);
    self
  }

  pub fn decode<FUN, ERR>(&self, verify_fn: &FUN, data: &'b [u8]) -> Result<Token<'b>>
  where
    FUN: Fn(
      Option<DecoderProtectedHeader<'_>>,
      Option<DecoderUnprotectedHeader<'_>>,
      DecoderMessage<'_>,
      DecoderSignature<'_>,
    ) -> std::result::Result<(), ERR>,
    ERR: Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
  {
    self.expand(data, |payload, signatures| {
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
    FUN: Fn(
      Option<DecoderProtectedHeader<'_>>,
      Option<DecoderUnprotectedHeader<'_>>,
      DecoderMessage<'_>,
      DecoderSignature<'_>,
    ) -> std::result::Result<(), ERR>,
    ERR: Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
  {
    let protected: Option<JwsHeader> = jws_signature.protected.map(decode_b64_json).transpose()?;

    validate_jws_headers(protected.as_ref(), jws_signature.header.as_ref(), self.crits.as_deref())?;

    let merged: HeaderSet<'_> = HeaderSet::new()
      .protected(protected.as_ref())
      .unprotected(jws_signature.header.as_ref());
    let alg: JwsAlgorithm = merged.try_alg()?;

    self.check_alg(alg)?;

    {
      let protected_bytes: &[u8] = jws_signature.protected.map(str::as_bytes).unwrap_or_default();
      let message: Vec<u8> = create_message(protected_bytes, payload);
      let signature: Vec<u8> = decode_b64(jws_signature.signature)?;

      verify_fn(
        protected.as_ref(),
        jws_signature.header.as_ref(),
        message.as_slice(),
        signature.as_slice(),
      )
      .map_err(|err| Error::SignatureVerificationError(err.into()))?;
    }

    let claims: Cow<'b, [u8]> = if merged.b64().unwrap_or(true) {
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

  fn expand<T>(&self, data: &'b [u8], format: impl Fn(&'b [u8], Vec<JwsSignature<'_>>) -> Result<T>) -> Result<T> {
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

        format(self.expand_payload(Some(split[1]))?, vec![signature])
      }
      JwsFormat::General => {
        let data: General<'_> = serde_json::from_slice(data).map_err(Error::InvalidJson)?;

        format(self.expand_payload(data.payload)?, data.signatures)
      }
      JwsFormat::Flatten => {
        let data: Flatten<'_> = serde_json::from_slice(data).map_err(Error::InvalidJson)?;

        format(self.expand_payload(data.payload)?, vec![data.signature])
      }
    }
  }

  fn expand_payload(&self, payload: Option<&'b (impl AsRef<[u8]> + ?Sized)>) -> Result<&'b [u8]> {
    match (self.payload, filter_non_empty_bytes(payload)) {
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

impl<'a> Default for Decoder<'a> {
  fn default() -> Self {
    Self::new()
  }
}
