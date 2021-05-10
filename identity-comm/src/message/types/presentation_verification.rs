// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Url;
use identity_credential::presentation::Presentation;
use identity_iota::did::IotaDID;
use uuid::Uuid;

use crate::message::Timing;

/// A DIDComm `presentation-verification` Request.
///
/// [Reference](https://github.com/iotaledger/identity.rs/blob/dev/docs/DID%20Communications%20Research%20and%20Specification/i_presentation-verification.md#presentationrequest)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PresentationRequest {
  context: String,
  thread: Uuid,
  #[serde(rename = "callbackURL")]
  callback_url: Url,
  #[serde(rename = "trustedIssuers", skip_serializing_if = "Option::is_none")]
  trusted_issuers: Option<Vec<TrustedIssuer>>,
  #[serde(rename = "responseRequested", skip_serializing_if = "Option::is_none")]
  response_requested: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  id: Option<IotaDID>,
  #[serde(skip_serializing_if = "Option::is_none")]
  timing: Option<Timing>,
}

impl PresentationRequest {
  /// Creates a new `PresentationRequest`.
  pub fn new(context: String, thread: Uuid, callback_url: Url) -> Self {
    Self {
      context,
      thread,
      callback_url,
      trusted_issuers: None,
      response_requested: None,
      id: None,
      timing: None,
    }
  }

  impl_message_accessor!(context => String);
  impl_message_accessor!(thread => Uuid);
  impl_message_accessor!(callback_url => Url);
  impl_message_accessor!(trusted_issuers => Option<Vec<TrustedIssuer>>);
  impl_message_accessor!(response_requested => Option<bool>);
  impl_message_accessor!(id => Option<IotaDID>);
  impl_message_accessor!(timing => Option<Timing>);
}

/// A DIDComm `presentation-verification` Response.
///
/// [Reference](https://github.com/iotaledger/identity.rs/blob/dev/docs/DID%20Communications%20Research%20and%20Specification/i_presentation-verification.md#presentationresponse)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PresentationResponse {
  context: String,
  thread: Uuid,
  #[serde(rename = "verifiablePresentation")]
  verifiable_presentation: Presentation,
  #[serde(rename = "callbackURL", skip_serializing_if = "Option::is_none")]
  callback_url: Option<Url>,
  #[serde(rename = "responseRequested", skip_serializing_if = "Option::is_none")]
  response_requested: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  id: Option<IotaDID>,
  #[serde(skip_serializing_if = "Option::is_none")]
  timing: Option<Timing>,
}

impl PresentationResponse {
  /// Creates a new `PresentationResponse`.
  pub fn new(context: String, thread: Uuid, verifiable_presentation: Presentation) -> Self {
    Self {
      context,
      thread,
      verifiable_presentation,
      callback_url: None,
      response_requested: None,
      id: None,
      timing: None,
    }
  }

  impl_message_accessor!(context => String);
  impl_message_accessor!(thread => Uuid);
  impl_message_accessor!(verifiable_presentation => Presentation);
  impl_message_accessor!(callback_url => Option<Url>);
  impl_message_accessor!(response_requested => Option<bool>);
  impl_message_accessor!(id => Option<IotaDID>);
  impl_message_accessor!(timing => Option<Timing>);
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TrustedIssuer {
  #[serde(rename = "credentialTypes")]
  credential_types: Vec<String>,
  #[serde(rename = "supportedIssuers")]
  supported_issuers: Vec<String>,
}

impl TrustedIssuer {
  /// Creates a new `TrustedIssuer`.
  pub fn new() -> Self {
    Self {
      credential_types: Vec::new(),
      supported_issuers: Vec::new(),
    }
  }

  impl_message_accessor!(credential_types => Vec<String>);
  impl_message_accessor!(supported_issuers => Vec<String>);
}
