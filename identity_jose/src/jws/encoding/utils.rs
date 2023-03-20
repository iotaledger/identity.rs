// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::Serialize;

use super::JwsHeader;
use crate::error::Error;
use crate::error::Result;
use crate::jws::Recipient;
use std::borrow::Cow;

use crate::jwu;

/// Transform detached payload according to the b64 extracted from the protected header.  
/// See: https://tools.ietf.org/html/rfc7797#section-3
pub(super) fn process_detached_payload<'payload>(
  payload: &'payload [u8],
  protected_header: Option<&JwsHeader>,
) -> Cow<'payload, [u8]> {
  jwu::extract_b64(protected_header)
    .then(|| Cow::Owned(jwu::encode_b64(payload).into()))
    .unwrap_or(Cow::Borrowed(payload))
}

/// Transform non-detached payload according to the b64 extracted from the protected header.
///
/// For non-encoded payloads (b64 = false) validations beyond checking for valid utf-8 encoding
/// depend on the serialization format and can be specialized with the `additional_validations` parameter.
pub(super) fn process_payload<'payload, F>(
  payload: &'payload [u8],
  protected_header: Option<&JwsHeader>,
  additional_validations: F,
) -> Result<Cow<'payload, str>>
where
  // TODO: Change to &str
  F: FnOnce(&[u8]) -> Result<()>,
{
  let payload: Cow<'payload, str> = {
    if jwu::extract_b64(protected_header) {
      Cow::Owned(jwu::encode_b64(payload))
    } else {
      let payload: &str = std::str::from_utf8(payload).map_err(|_| Error::InvalidContent("invalid UTF-8"))?;
      additional_validations(payload.as_bytes())?;
      Cow::Borrowed(payload)
    }
  };
  Ok(payload)
}

// ===================================================================================
//  JWS JSON Serialization encoding utils
// ===================================================================================

pub(super) fn validate_headers_json_serialization(recipient: Recipient<'_>) -> Result<()> {
  let Recipient { protected, unprotected } = recipient;
  if protected.is_none() && unprotected.is_none() {
    return Err(Error::MissingHeader("at least one header must be set"));
  };

  jwu::validate_jws_headers(protected, unprotected, protected.and_then(|header| header.crit()))
}

#[derive(Serialize)]
pub(super) struct JwsSignature<'unprotected> {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub(super) header: Option<&'unprotected JwsHeader>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub(super) protected: Option<String>,
  pub(super) signature: String,
}

#[derive(Serialize)]
pub(super) struct Flatten<'payload, 'unprotected> {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub(super) payload: Option<&'payload str>,
  #[serde(flatten)]
  pub(super) signature: JwsSignature<'unprotected>,
}

impl<'payload, 'unprotected> Flatten<'payload, 'unprotected> {
  pub(super) fn to_json(&self) -> Result<String> {
    serde_json::to_string(&self).map_err(Error::InvalidJson)
  }
}

#[derive(Serialize)]
pub(super) struct General<'payload, 'unprotected> {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub(super) payload: Option<&'payload str>,
  pub(super) signatures: Vec<JwsSignature<'unprotected>>,
}

impl<'payload, 'unprotected> General<'payload, 'unprotected> {
  pub(super) fn to_json(&self) -> Result<String> {
    serde_json::to_string(&self).map_err(Error::InvalidJson)
  }
}

pub(super) struct SigningData {
  pub(super) signing_input: Box<[u8]>,
  pub(super) protected_header: Option<String>,
}

impl SigningData {
  pub(super) fn new(processed_payload: &[u8], protected_header: Option<&JwsHeader>) -> Result<Self> {
    let protected_header: Option<String> = protected_header.map(jwu::encode_b64_json).transpose()?;
    let signing_input: Box<[u8]> = jwu::create_message(
      protected_header.as_deref().map(str::as_bytes).unwrap_or_default(),
      processed_payload,
    )
    .into();
    Ok(SigningData {
      signing_input,
      protected_header,
    })
  }

  pub(super) fn into_signature<'unprotected>(
    self,
    signature: &[u8],
    unprotected_header: Option<&'unprotected JwsHeader>,
  ) -> JwsSignature<'unprotected> {
    JwsSignature {
      protected: self.protected_header,
      header: unprotected_header,
      signature: jwu::encode_b64(signature),
    }
  }
}
