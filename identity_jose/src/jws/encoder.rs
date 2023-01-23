// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::str;
use serde::Serialize;
use std::future::Future;

use crate::error::Error;
use crate::error::Result;
use crate::jws::CharSet;
use crate::jws::JwsAlgorithm;
use crate::jws::JwsFormat;
use crate::jws::JwsHeader;
use crate::jws::Recipient;
use crate::jwt::JwtHeaderSet;
use crate::jwu;

macro_rules! to_json {
  ($data:expr) => {{
    ::serde_json::to_string(&$data).map_err(|err| Error::InvalidJson(err))
  }};
}

#[derive(Serialize)]
struct Signature<'a> {
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
  signatures: Vec<Signature<'a>>,
}

#[derive(Serialize)]
struct Flatten<'a, 'b> {
  #[serde(skip_serializing_if = "Option::is_none")]
  payload: Option<&'a str>,
  #[serde(flatten)]
  signature: &'b Signature<'a>,
}

// =============================================================================
// =============================================================================

// TODO: Use type alias for keyId instead of raw string.
pub struct Encoder<'a, FUN, FUT, ERR>
where
  FUN: Fn(JwsAlgorithm, String, Vec<u8>) -> FUT + 'static + Send + Sync,
  FUT: Future<Output = std::result::Result<String, ERR>> + Send,
  ERR: Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
{
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
  /// The function to sign with. Returns the signature as a base64-encoded string.
  sign: FUN,
}

impl<'a, FUN, FUT, ERR> Encoder<'a, FUN, FUT, ERR>
where
  FUN: Fn(JwsAlgorithm, String, Vec<u8>) -> FUT + 'static + Send + Sync,
  FUT: Future<Output = std::result::Result<String, ERR>> + Send,
  ERR: Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
{
  pub fn new(sign: FUN) -> Self {
    Self {
      format: JwsFormat::Compact,
      charset: CharSet::Default,
      detached: false,
      recipients: Vec::new(),
      sign,
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

  pub async fn encode_serde<T>(&self, claims: &T) -> Result<String>
  where
    T: Serialize,
  {
    self
      .encode(&serde_json::to_vec(claims).map_err(|err| Error::InvalidJson(err))?)
      .await
  }

  pub async fn encode(&self, claims: &[u8]) -> Result<String> {
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

    let mut encoded: Vec<Signature<'a>> = Vec::with_capacity(self.recipients.len());
    for recipient in self.recipients.iter().copied() {
      encoded.push(self.encode_recipient(payload, recipient).await?);
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
        unreachable!()
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

  async fn encode_recipient<'b>(&self, payload: &[u8], recipient: Recipient<'b>) -> Result<Signature<'b>> {
    let header_set: JwtHeaderSet<_> = JwtHeaderSet::new()
      .protected(recipient.protected)
      .unprotected(recipient.unprotected);
    let algorithm: JwsAlgorithm = header_set.try_alg()?;
    let kid: &str = header_set.try_kid()?;

    let protected: Option<String> = recipient.protected.map(jwu::encode_b64_json).transpose()?;
    let header: &[u8] = protected.as_deref().map(str::as_bytes).unwrap_or_default();
    let message: Vec<u8> = jwu::create_message(header, payload);
    let signature: String = (self.sign)(algorithm, kid.to_owned(), message)
      .await
      .map_err(|err| Error::SignatureCreationError(err.into()))?;

    Ok(Signature {
      header: recipient.unprotected,
      protected,
      signature,
    })
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
