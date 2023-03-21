// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::str;
use serde::Serialize;
use std::future::Future;

use crate::error::Error;
use crate::error::Result;
use crate::jws::CharSet;
use crate::jws::JwsFormat;
use crate::jws::JwsHeader;
use crate::jws::Recipient;
use crate::jwu;

/// The protected JWS header.
pub type EncoderProtectedHeader = JwsHeader;
/// The unprotected JWS header.
pub type EncoderUnprotectedHeader = JwsHeader;
/// The message to sign as a byte vector.
pub type EncoderMessage = Vec<u8>;
/// The base64url-encoded signature.
pub type EncoderSignature = String;

macro_rules! to_json {
  ($data:expr) => {{
    ::serde_json::to_string(&$data).map_err(Error::InvalidJson)
  }};
}

#[derive(Serialize)]
struct JwsSignature<'a> {
  #[serde(skip_serializing_if = "Option::is_none")]
  header: Option<&'a JwsHeader>,
  #[serde(skip_serializing_if = "Option::is_none")]
  protected: Option<String>,
  signature: String,
}

#[derive(Serialize)]
struct General<'a> {
  #[serde(skip_serializing_if = "Option::is_none")]
  payload: Option<&'a str>,
  signatures: Vec<JwsSignature<'a>>,
}

#[derive(Serialize)]
struct Flatten<'a, 'b> {
  #[serde(skip_serializing_if = "Option::is_none")]
  payload: Option<&'a str>,
  #[serde(flatten)]
  signature: &'b JwsSignature<'a>,
}

// =============================================================================
// =============================================================================

/// The [`Encoder`] allows encoding an arbitrary set of claims into a JWS.
///
/// When encoding, the format, charset and recipients of the resulting JWS
/// can be set in builder-style.
///
/// This API does not have any cryptography built-in. Rather, signatures are created
/// through a closure that is passed to the [`encode`](Encoder::encode) method so that users can implement
/// the signature algorithm of their choice.
///
/// To use a particular key for a recipient, it is recommended to set the `kid` parameter
/// in their header. Based on that, the closure can then choose the appropriate key for
/// the recipient.
pub struct Encoder<'a> {
  /// The output format of the encoded token.
  format: JwsFormat,
  /// Content validation rules for unencoded content using the compact format.
  charset: CharSet,
  /// Encode the token with detached content.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7515#appendix-F)
  detached: bool,
  /// Per-recipient configuration.
  recipients: Vec<Recipient<'a>>,
}

impl<'a> Encoder<'a> {
  pub fn new() -> Self {
    Self {
      format: JwsFormat::Compact,
      charset: CharSet::Default,
      detached: false,
      recipients: Vec::new(),
    }
  }

  pub fn format(mut self, value: JwsFormat) -> Self {
    self.format = value;
    self
  }

  pub fn charset(mut self, value: CharSet) -> Self {
    self.charset = value;
    self
  }

  pub fn detached(mut self, value: bool) -> Self {
    self.detached = value;
    self
  }

  pub fn recipient(mut self, recipient: Recipient<'a>) -> Self {
    self.recipients.push(recipient);
    self
  }

  pub async fn encode_serde<T, FUN, FUT, ERR>(&self, sign_fn: &FUN, claims: &T) -> Result<String>
  where
    T: Serialize,
    FUN: Fn(Option<EncoderProtectedHeader>, Option<EncoderUnprotectedHeader>, EncoderMessage) -> FUT
      + 'static
      + Send
      + Sync,
    FUT: Future<Output = std::result::Result<EncoderSignature, ERR>> + Send,
    ERR: Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
  {
    self
      .encode(sign_fn, &serde_json::to_vec(claims).map_err(Error::InvalidJson)?)
      .await
  }

  /// Encode the given `claims` into a JWS.
  ///
  /// The `sign_fn` closure is called to create a signature for every recipient.
  /// Their protected and unprotected headers are passed in which can be merged with
  /// [`JwtHeaderSet`](crate::jwt::JwtHeaderSet). The header parameters of particular interest are `alg` and `kid`.
  /// The closure also takes the bytes to be signed and is expected to return the signature as bytes.
  pub async fn encode<FUN, FUT, ERR>(&self, sign_fn: &FUN, claims: &[u8]) -> Result<String>
  where
    FUN: Fn(Option<EncoderProtectedHeader>, Option<EncoderUnprotectedHeader>, EncoderMessage) -> FUT
      + 'static
      + Send
      + Sync,
    FUT: Future<Output = std::result::Result<EncoderSignature, ERR>> + Send,
    ERR: Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
  {
    if self.recipients.is_empty() {
      return Err(Error::SignatureCreationError("Missing Recipients".into()));
    }

    self.validate()?;

    let b64: bool = jwu::extract_b64(self.recipients[0].protected);
    let tmp: String;

    // Extract the "b64" header parameter and encode the payload as required.
    //
    // See: https://tools.ietf.org/html/rfc7797#section-3
    let payload: &[u8] = if b64 {
      tmp = jwu::encode_b64(claims);
      tmp.as_bytes()
    } else if self.detached {
      claims
    } else {
      self.charset.validate(claims)?;
      claims
    };

    let mut encoded: Vec<JwsSignature<'a>> = Vec::with_capacity(self.recipients.len());
    for recipient in self.recipients.iter().copied() {
      encoded.push(self.encode_recipient(sign_fn, payload, recipient).await?);
    }
    debug_assert_eq!(encoded.len(), self.recipients.len());

    match (self.format, &*encoded) {
      (JwsFormat::Compact, [recipient]) => {
        let protected: &str = recipient.protected.as_deref().unwrap_or_default();

        if let Some(payload) = self.format_payload(payload) {
          Ok(format!("{}.{}.{}", protected, payload, recipient.signature))
        } else {
          Ok(format!("{}..{}", protected, recipient.signature))
        }
      }
      (JwsFormat::General, _) => {
        to_json!(General {
          payload: self.format_payload(payload),
          signatures: encoded,
        })
      }
      (JwsFormat::Flatten, [recipient]) => {
        to_json!(Flatten {
          payload: self.format_payload(payload),
          signature: recipient,
        })
      }
      _ => {
        unreachable!("an error should have been returned because multiple recipients is only permitted with the general JWS JSON serialization format")
      }
    }
  }

  fn format_payload(&self, payload: &'a [u8]) -> Option<&'a str> {
    if self.detached {
      None
    } else {
      // SAFETY: We validated the payload and ensured valid UTF-8 as an earlier
      // step in the encoding process; this function is not exposed to users.
      Some(unsafe { str::from_utf8_unchecked(payload) })
    }
  }

  fn validate(&self) -> Result<()> {
    match (self.format, &*self.recipients) {
      (JwsFormat::Compact, &[recipient @ Recipient { unprotected: None, .. }]) => jwu::validate_jws_headers(
        recipient.protected,
        None,
        recipient.protected.and_then(|header| header.crit()),
      ),
      (JwsFormat::Compact, _) => Err(Error::SignatureCreationError(
        "JWS Compact Serialization doesn't support multiple recipients or unprotected headers".into(),
      )),
      (JwsFormat::General, recipients) => {
        let mut __b64: bool = false;

        for recipient in recipients {
          if !__b64 && recipient.protected.and_then(JwsHeader::b64).is_some() {
            __b64 = true;
          }

          jwu::validate_jws_headers(
            recipient.protected,
            recipient.unprotected,
            recipient.protected.and_then(|header| header.crit()),
          )?;
        }

        if __b64 {
          validate_recipient_b64(recipients)?;
        }

        Ok(())
      }
      (JwsFormat::Flatten, &[recipient]) => jwu::validate_jws_headers(
        recipient.protected,
        recipient.unprotected,
        recipient.protected.and_then(|header| header.crit()),
      ),
      (JwsFormat::Flatten, _) => Err(Error::SignatureCreationError(
        "JWS Flattened Serialization doesn't support multiple recipients".into(),
      )),
    }
  }

  async fn encode_recipient<'b, FUN, FUT, ERR>(
    &self,
    sign_fn: &FUN,
    payload: &[u8],
    recipient: Recipient<'b>,
  ) -> Result<JwsSignature<'b>>
  where
    FUN: Fn(Option<EncoderProtectedHeader>, Option<EncoderUnprotectedHeader>, EncoderMessage) -> FUT
      + 'static
      + Send
      + Sync,
    FUT: Future<Output = std::result::Result<EncoderSignature, ERR>> + Send,
    ERR: Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
  {
    let protected: Option<String> = recipient.protected.map(jwu::encode_b64_json).transpose()?;
    let header: &[u8] = protected.as_deref().map(str::as_bytes).unwrap_or_default();
    let message: Vec<u8> = jwu::create_message(header, payload);
    let signature: String = (sign_fn)(recipient.protected.cloned(), recipient.unprotected.cloned(), message)
      .await
      .map_err(|err| Error::SignatureCreationError(err.into()))?;

    Ok(JwsSignature {
      header: recipient.unprotected,
      protected,
      signature,
    })
  }
}

impl<'a> Default for Encoder<'a> {
  fn default() -> Self {
    Self::new()
  }
}

// =============================================================================
// =============================================================================

fn validate_recipient_b64(recipients: &[Recipient<'_>]) -> Result<()> {
  // The "b64" header parameter value MUST be the same for all recipients
  recipients
    .iter()
    .map(|recipient| jwu::extract_b64(recipient.protected))
    .try_fold::<_, _, Result<_>>(None, |acc, item| match acc {
      Some(current) if current == item => Ok(acc),
      Some(_) => Err(Error::InvalidParam("b64")),
      None => Ok(Some(item)),
    })
    .map(|_| ())
}
