// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::str;
use serde_json::from_slice;
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

pub type KeyId<'a> = &'a str;
pub type Message<'a> = &'a [u8];
pub type Signature<'a> = &'a [u8];

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

pub struct Decoder<'b, FUN, ERR>
where
  FUN: Fn(JwsAlgorithm, KeyId<'_>, Message<'_>, Signature<'_>) -> std::result::Result<(), ERR>,
  ERR: Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
{
  /// The expected format of the encoded token.
  format: JwsFormat,
  /// The function used for signature verification.
  verify: FUN,
  /// A list of permitted signature algorithms.
  algs: Option<Vec<JwsAlgorithm>>,
  /// A list of permitted extension parameters.
  crits: Option<Vec<String>>,
  /// The detached payload, if using detached content
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7515#appendix-F)
  payload: Option<&'b [u8]>,
}

impl<'a, 'b, FUN, ERR> Decoder<'b, FUN, ERR>
where
  FUN: Fn(JwsAlgorithm, KeyId<'_>, Message<'_>, Signature<'_>) -> std::result::Result<(), ERR>,
  ERR: Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
{
  pub fn new(verify: FUN) -> Self {
    Self {
      format: JwsFormat::Compact,
      verify,
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

  pub fn decode(&self, data: &'b [u8]) -> Result<Token<'b>> {
    self.expand(data, |payload, signatures| {
      for signature in signatures {
        // if let Ok(token) = self.decode_one(payload, signature) {
        //   return Ok(token);
        // }
        match self.decode_one(payload, signature) {
          Ok(token) => return Ok(token),
          Err(err) => {
            println!("{err}");
          }
        }
      }

      Err(Error::InvalidContent("Recipient (not found)"))
    })
  }

  fn decode_one(&self, payload: &'b [u8], signature: JwsSignature<'a>) -> Result<Token<'b>> {
    let protected: Option<JwsHeader> = signature.protected.map(decode_b64_json).transpose()?;

    validate_jws_headers(protected.as_ref(), signature.header.as_ref(), self.crits.as_deref())?;

    let merged: HeaderSet<'_> = HeaderSet::new()
      .protected(protected.as_ref())
      .unprotected(signature.header.as_ref());
    let alg: JwsAlgorithm = merged.try_alg()?;

    self.check_alg(alg)?;

    {
      let protected: &[u8] = signature.protected.map(str::as_bytes).unwrap_or_default();
      let message: Vec<u8> = create_message(protected, payload);
      let signature: Vec<u8> = decode_b64(signature.signature)?;

      (self.verify)(
        alg,
        merged.try_kid()?.as_ref(),
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
      unprotected: signature.header,
      claims,
    })
  }

  fn expand<T>(&self, data: &'b [u8], format: impl Fn(&'b [u8], Vec<JwsSignature<'_>>) -> Result<T>) -> Result<T> {
    match self.format {
      JwsFormat::Compact => {
        let split: Vec<&[u8]> = data.split(|byte| *byte == b'.').collect();

        if split.len() != COMPACT_SEGMENTS {
          return Err(Error::InvalidContent("Segments (count)"));
        }

        let signature: JwsSignature<'_> = JwsSignature {
          header: None,
          protected: Some(parse_utf8(split[0])?),
          signature: parse_utf8(split[2])?,
        };

        format(self.expand_payload(Some(split[1]))?, vec![signature])
      }
      JwsFormat::General => {
        let data: General<'_> = from_slice(data)?;

        format(self.expand_payload(data.payload)?, data.signatures)
      }
      JwsFormat::Flatten => {
        let data: Flatten<'_> = from_slice(data)?;

        format(self.expand_payload(data.payload)?, vec![data.signature])
      }
    }
  }

  fn expand_payload(&self, payload: Option<&'b (impl AsRef<[u8]> + ?Sized)>) -> Result<&'b [u8]> {
    match (self.payload, filter_non_empty_bytes(payload)) {
      (Some(payload), None) => Ok(payload),
      (None, Some(payload)) => Ok(payload),
      (Some(_), Some(_)) => Err(Error::InvalidContent("Payload (multiple)")),
      (None, None) => Err(Error::InvalidContent("Payload (missing)")),
    }
  }

  fn check_alg(&self, value: JwsAlgorithm) -> Result<()> {
    check_slice_param("alg", self.algs.as_deref(), &value)
  }
}
