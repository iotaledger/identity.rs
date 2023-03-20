// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0



use crate::error::Result;
use std::borrow::Cow;

use crate::jwu;

use super::CharSet;
use super::JwsHeader;
mod utils; 


pub struct PreSignedCompactJws<'a> {
  protected_header: String,
  processed_payload: Option<Cow<'a, str>>,
  signing_input: Box<[u8]>,
}

impl<'payload> PreSignedCompactJws<'payload> {
  pub fn new(payload: &'payload [u8], protected_header: &JwsHeader) -> Result<Self> {
    Self::new_with_charset(payload, protected_header, CharSet::Default)
  }

  pub fn new_with_charset(payload: &'payload [u8], protected_header: &JwsHeader, charset: CharSet) -> Result<Self> {
    Self::validate_header(protected_header)?;

    // Transform payload according to b64.
    // See: https://tools.ietf.org/html/rfc7797#section-3
    let payload: Cow<'payload, str> = {
      if jwu::extract_b64(Some(protected_header)) {
        Cow::Owned(jwu::encode_b64(payload))
      } else {
        // Unencoded payloads need to be validated against the given `charset` to ensure they satisfy
        // application requirements.
        let payload = charset.validate(payload)?;
        Cow::Borrowed(payload)
      }
    };
    let protected_header: String = jwu::encode_b64_json(protected_header)?;
    let signing_input: Box<[u8]> = jwu::create_message(protected_header.as_bytes(), payload.as_bytes()).into();
    Ok(Self {
      protected_header,
      processed_payload: Some(payload),
      signing_input,
    })
  }

  pub fn new_detached(payload: &'payload [u8], protected_header: &JwsHeader) -> Result<Self> {
    Self::validate_header(protected_header)?; 
    let payload: Cow<'payload, [u8]> = utils::process_detached_payload(payload, Some(protected_header));

    let protected_header: String = jwu::encode_b64_json(protected_header)?;
    let signing_input: Box<[u8]> = jwu::create_message(protected_header.as_bytes(), &payload).into();
    Ok(Self {
      protected_header,
      // Don't forward the payload since it is detached
      processed_payload: None,
      signing_input,
    })
  }
  /// The signing input.
  pub fn signing_input(&self) -> &[u8] {
    &self.signing_input
  }

  fn validate_header(protected_header: &JwsHeader) -> Result<()> {
    jwu::validate_jws_headers(Some(protected_header), None, protected_header.crit())
  }

  /// convert this into a JWS. The `signature` value is expected to be
  /// the signature on [`Self::signing_input`] by the private key corresponding to the public key
  /// referenced in the JWS header in accordance with the `alg` value of said header.
  pub fn into_jws(self, signature: &[u8]) -> String {
    let signature = jwu::encode_b64(signature);
    if let Some(payload) = self.processed_payload {
      format!("{}.{}.{}", self.protected_header, payload, &signature)
    } else {
      format!("{}..{}", self.protected_header, &signature)
    }
  }
}


// ===================================================================================
//  JWS JSON Serialization 
// ===================================================================================

pub struct PreSignedFlattenedJws<'payload, 'unprotected> {
    processed_payload: Option<Cow<'payload, str>>,
    signing_data: SigningData,
    unprotected_header: Option<&'unprotected JwsHeader>
  }
  
  pub fn new(payload: &'payload [u8], protected_header: Option<&JwsHeader>, unprotected_header: Option<&JwsHeader>) -> Result<Self> {
    Self::validate_headers(protected_header, unprotected_header)?;
    // Transform payload according to b64.
    // See: https://tools.ietf.org/html/rfc7797#section-3
    let payload: Cow<'payload, str> = utils::process_payload_json_serialization(payload, protected_header);
    let signing_data: SigningData = utils::generate_signing_data(payload, protected_header);
    Ok(
        Self{
            processed_payload: Some(payload), 
            signing_data, 
            unprotected_header
        }
    )
  }
  
  pub fn new_detached(payload: &'payload [u8], protected_header: Option<&JwsHeader>, unprotected_header: Option<&JwsHeader>) -> Result<Self> {
    Self::validate_headers(protected_header, unprotected_header)?;
    let payload: Cow<'payload, [u8]> = utils::process_detached_payload(payload, protected_header);
    let signing_data: SigningData = utils::generate_signing_data(payload, protected_header);
    Ok(
        Self {
            processed_payload: None, 
            signing_data,
            unprotected_header
        }
    )
  }
  
  
  impl<'payload, 'unprotected> PreSignedFlattenedJws<'payload, 'unprotected> {
    pub fn signing_input(&self) -> &[u8] {
      &self.signing_data.signing_input
    }
  
    /// convert this into a JWS. The `signature` value is expected to be
    /// the signature on [`Self::signing_input`] by the private key corresponding to the public key
    /// referenced in the JWS header in accordance with the `alg` value of said header.
    pub fn into_jws(self, signature: &[u8]) -> Result<String> {
      let PreSignedFlattenedJws{processed_payload, signing_data: SigningData{protected_header, ..}, unprotected_header } = self;
      let jws_signature: JwsSignature<&'unprotected JwsHeader> = JwsSignature {
        protected: protected_header,
        header: unprotected_header,
        signature: jwu::encode_b64(signature)
      };
      to_json!(Flatten{
        payload: processed_payload,
        signature: jws_signature
      })
    }
  }