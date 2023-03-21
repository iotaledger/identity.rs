// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::error::Error;
use crate::error::Result;
use crate::jws::encoding::utils::JwsSignature;
use std::borrow::Cow;

use crate::jwu;

use self::encoder_state::ReadyState;
use self::encoder_state::RecipientProcessingState;
use self::utils::Flatten;
use self::utils::General;
use self::utils::SigningData;

use super::CharSet;
use super::JwsHeader;
use super::Recipient;
mod utils;

pub struct CompactJwsEncoder<'a> {
  protected_header: String,
  processed_payload: Option<Cow<'a, str>>,
  signing_input: Box<[u8]>,
}

impl<'payload> CompactJwsEncoder<'payload> {
  pub fn new(payload: &'payload [u8], protected_header: &JwsHeader) -> Result<Self> {
    Self::new_with_charset(payload, protected_header, CharSet::Default)
  }

  pub fn new_with_charset(payload: &'payload [u8], protected_header: &JwsHeader, charset: CharSet) -> Result<Self> {
    Self::validate_header(protected_header)?;
    let payload: Cow<'payload, str> = utils::process_payload(payload, Some(protected_header), |payload: &[u8]| {
      charset.validate(payload)
    })?;
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

// ===============================================================================================================================
//  JWS JSON Serialization
// ===============================================================================================================================

// ===================================================================================
//  Flattened JWS Json Serialization
// ===================================================================================
pub struct FlattenedJwsEncoder<'payload, 'unprotected> {
  processed_payload: Option<Cow<'payload, str>>,
  signing_data: SigningData,
  unprotected_header: Option<&'unprotected JwsHeader>,
}

impl<'payload, 'unprotected> FlattenedJwsEncoder<'payload, 'unprotected> {
  pub fn new(payload: &'payload [u8], recipient: Recipient<'unprotected>) -> Result<Self> {
    utils::validate_headers_json_serialization(recipient)?;
    // The only additional validation required when processing the payload to be used with the flattened JWS Json
    // serialization is that it is a valid JSON value, but we check for that later during serialization.
    let payload: Cow<'payload, str> = utils::process_payload(payload, recipient.protected, |_| Ok(()))?;
    let signing_data: SigningData = SigningData::new(payload.as_bytes(), recipient.protected)?;
    Ok(Self {
      processed_payload: Some(payload),
      signing_data,
      unprotected_header: recipient.unprotected,
    })
  }

  pub fn new_detached(payload: &'payload [u8], recipient: Recipient<'unprotected>) -> Result<Self> {
    utils::validate_headers_json_serialization(recipient)?;
    let payload: Cow<'payload, [u8]> = utils::process_detached_payload(payload, recipient.protected);
    let signing_data: SigningData = SigningData::new(&payload, recipient.protected)?;
    Ok(Self {
      processed_payload: None,
      signing_data,
      unprotected_header: recipient.unprotected,
    })
  }

  pub fn signing_input(&self) -> &[u8] {
    &self.signing_data.signing_input
  }

  /// convert this into a JWS. The `signature` value is expected to be
  /// the signature on [`Self::signing_input`] by the private key corresponding to the public key
  /// referenced in the JWS header in accordance with the `alg` value of said header.
  pub fn into_jws(self, signature: &[u8]) -> Result<String> {
    let FlattenedJwsEncoder {
      processed_payload,
      signing_data,
      unprotected_header,
    } = self;

    Flatten {
      payload: processed_payload.as_deref(),
      signature: signing_data.into_signature(signature, unprotected_header),
    }
    .to_json()
  }
}

// ===================================================================================
//  General JWS Json Serialization
// ===================================================================================

mod encoder_state {
  use super::utils::SigningData;
  use crate::jws::JwsHeader;
  pub struct ReadyState;
  pub struct RecipientProcessingState<'unprotected> {
    pub(super) signing_data: SigningData,
    pub(super) unprotected_header: Option<&'unprotected JwsHeader>,
  }
}
pub type RecipientProcessingEncoder<'payload, 'unprotected> =
  GeneralJwsEncoder<'payload, 'unprotected, RecipientProcessingState<'unprotected>>;
pub struct GeneralJwsEncoder<'payload, 'unprotected, STATE = ReadyState> {
  partially_processed_payload: Cow<'payload, [u8]>,
  signatures: Vec<JwsSignature<'unprotected>>,
  detached: bool,
  b64: bool,
  state: STATE,
}

impl<'payload, 'unprotected> GeneralJwsEncoder<'payload, 'unprotected> {
  pub fn new(
    payload: &'payload [u8],
    first_recipient: Recipient<'unprotected>,
  ) -> Result<RecipientProcessingEncoder<'payload, 'unprotected>> {
    Self::initialize(payload, first_recipient, false)
  }

  pub fn new_detached(
    payload: &'payload [u8],
    first_recipient: Recipient<'unprotected>,
  ) -> Result<RecipientProcessingEncoder<'payload, 'unprotected>> {
    Self::initialize(payload, first_recipient, true)
  }

  fn initialize(
    payload: &'payload [u8],
    first_recipient: Recipient<'unprotected>,
    detached: bool,
  ) -> Result<RecipientProcessingEncoder<'payload, 'unprotected>> {
    utils::validate_headers_json_serialization(first_recipient)?;
    // We will handle the detached/non-detached case in Self::into_jws
    let partially_processed_payload: Cow<'payload, [u8]> =
      utils::process_detached_payload(payload, first_recipient.protected);
    let signing_data = SigningData::new(&partially_processed_payload, first_recipient.protected)?;
    Ok(RecipientProcessingEncoder {
      partially_processed_payload,
      signatures: Vec::new(),
      detached,
      b64: jwu::extract_b64(first_recipient.protected),
      state: RecipientProcessingState {
        signing_data,
        unprotected_header: first_recipient.unprotected,
      },
    })
  }

  pub fn add_recipient(
    self,
    recipient: Recipient<'unprotected>,
  ) -> Result<RecipientProcessingEncoder<'payload, 'unprotected>> {
    // Ensure that the b64 value is consistent across all signatures.
    let new_b64 = jwu::extract_b64(recipient.protected);
    if new_b64 != self.b64 {
      return Err(Error::InvalidParam("b64"));
    };
    let signing_data = SigningData::new(&self.partially_processed_payload, recipient.protected)?;
    let state = RecipientProcessingState {
      signing_data,
      unprotected_header: recipient.unprotected,
    };
    let Self {
      partially_processed_payload,
      signatures,
      detached,
      b64,
      ..
    } = self;
    Ok(RecipientProcessingEncoder {
      partially_processed_payload,
      signatures,
      detached,
      b64,
      state,
    })
  }

  pub fn into_jws(self) -> Result<String> {
    let GeneralJwsEncoder {
      partially_processed_payload,
      signatures,
      detached,
      ..
    } = self;
    let general: General = {
      if detached {
        General {
          payload: None,
          signatures,
        }
      } else {
        General {
          payload: Some(
            std::str::from_utf8(&partially_processed_payload).map_err(|_| Error::InvalidContent("invalid utf8"))?,
          ),
          signatures,
        }
      }
    };
    general.to_json()
  }
}

impl<'payload, 'unprotected> RecipientProcessingEncoder<'payload, 'unprotected> {
  pub fn signing_input(&self) -> &[u8] {
    &self.state.signing_data.signing_input
  }
  pub fn set_signature(self, signature: &[u8]) -> GeneralJwsEncoder<'payload, 'unprotected> {
    let Self {
      partially_processed_payload,
      mut signatures,
      b64,
      detached,
      state: RecipientProcessingState {
        unprotected_header,
        signing_data,
      },
    } = self;
    let new_signature = signing_data.into_signature(signature, unprotected_header);
    signatures.push(new_signature);
    GeneralJwsEncoder {
      partially_processed_payload,
      signatures,
      b64,
      detached,
      state: ReadyState {},
    }
  }
}
