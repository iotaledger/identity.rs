// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::JwsHeader;
use crate::error::Result;
use std::borrow::Cow;

use crate::jwu;

/// Transform detached payload according to the b64 extracted from the protected header.  
/// See: https://tools.ietf.org/html/rfc7797#section-3
fn process_detached_payload<'a>(payload: &'payload [u8], protected_header: Option<JwsHeader>) -> Cow<'payload, [u8]> {
    jwu::extract_b64(protected_header)
    .then(|| Cow::Owned(jwu::encode_b64(payload).into()))
    .unwrap_or(Cow::Borrowed(payload))
  }


// ===================================================================================
//  JWS JSON Serialization encoding utils
// ===================================================================================

macro_rules! to_json {
    ($data:expr) => {{
      ::serde_json::to_string(&$data).map_err(Error::InvalidJson)
    }};
  }
  
  #[derive(Serialize)]
  struct JwsSignature<HEADER>
  where
    HEADER: Borrow<JwsHeader>,
  {
    #[serde(skip_serializing_if = "Option::is_none")]
    header: Option<HEADER>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protected: Option<String>,
    signature: String,
  }
  
  #[derive(Serialize)]
  struct Flatten<'a, 'b> {
    #[serde(skip_serializing_if = "Option::is_none")]
    payload: Option<&'a str>,
    #[serde(flatten)]
    signature: &'b JwsSignature<'a>,
  }
  
  #[derive(Serialize)]
  struct General<'a, 'b> {
    #[serde(skip_serializing_if = "Option::is_none")]
    payload: Option<&'a str>,
    signatures: Vec<JwsSignature<'b>>,
  }
  
  
  /// Process non-detached payload 
  fn process_payload_json_serialization<'a> (payload: &'payload [u8], protected_header: Option<JwsHeader>) -> Result<Cow<'payload, str>> {
     jwu::extract_b64(protected_header)
    .then(||Cow::Owned(jwu::encode_b64(payload)))
    .unwrap_or(Cow::Borrowed(str::from_utf8(payload).map_err(|_|Error::InvalidContent("invalid UTF-8")?)))
  }
  
  struct SigningData {
  signing_input: Box<[u8]>,
  protected_header: Option<String>
  }
  
  fn generate_signing_data(processed_payload: &[u8], protected_header: Option<&JwsHeader>) -> SigningData {
    let protected_header: Option<String> = protected_header.map(jwu::encode_b64_json).transpose()?;
    let signing_input: Box<[u8]> = jwu::create_message(protected_header.as_deref().map(str::as_bytes).unwrap_or_default(), processed_payload).into();
    SigningData {
      signing_input,
      protected_header
    }
  }