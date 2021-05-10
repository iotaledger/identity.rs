// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Url;
use identity_iota::did::IotaDID;
use uuid::Uuid;

use crate::message::Timing;

/// A DIDComm `credential-issuance` Request.
///
/// [Reference](https://github.com/iotaledger/identity.rs/blob/dev/docs/DID%20Communications%20Research%20and%20Specification/i_credential-issuance.md#credentialselection)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CredentialSelection {
  context: String,
  thread: Uuid,
  #[serde(rename = "callbackURL")]
  callback_url: Url,
  #[serde(rename = "credentialTypes")]
  credential_types: Vec<String>,
  #[serde(rename = "responseRequested", skip_serializing_if = "Option::is_none")]
  response_requested: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  id: Option<IotaDID>,
  #[serde(skip_serializing_if = "Option::is_none")]
  timing: Option<Timing>,
}

impl CredentialSelection {
  /// Creates a new `CredentialSelection`.
  pub fn new(context: String, thread: Uuid, callback_url: Url, credential_types: Vec<String>) -> Self {
    Self {
      context,
      thread,
      callback_url,
      credential_types,
      response_requested: None,
      id: None,
      timing: None,
    }
  }

  impl_message_accessor!(context => String);
  impl_message_accessor!(thread => Uuid);
  impl_message_accessor!(callback_url => Url);
  impl_message_accessor!(credential_types => Vec<String>);
  impl_message_accessor!(response_requested => Option<bool>);
  impl_message_accessor!(id => Option<IotaDID>);
  impl_message_accessor!(timing => Option<Timing>);
}

/// A DIDComm `credential-issuance` Response.
///
/// [Reference](https://github.com/iotaledger/identity.rs/blob/dev/docs/DID%20Communications%20Research%20and%20Specification/i_credential-issuance.md#credentialissuance)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CredentialIssuance {
  context: String,
  thread: Uuid,
  credentials: Vec<String>,
  #[serde(rename = "callbackURL", skip_serializing_if = "Option::is_none")]
  callback_url: Option<Url>,
  #[serde(rename = "responseRequested", skip_serializing_if = "Option::is_none")]
  response_requested: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  id: Option<IotaDID>,
  #[serde(skip_serializing_if = "Option::is_none")]
  timing: Option<Timing>,
}

impl CredentialIssuance {
  /// Creates a new `CredentialIssuance`.
  pub fn new(context: String, thread: Uuid, credentials: Vec<String>) -> Self {
    Self {
      context,
      thread,
      credentials,
      callback_url: None,
      response_requested: None,
      id: None,
      timing: None,
    }
  }

  impl_message_accessor!(context => String);
  impl_message_accessor!(thread => Uuid);
  impl_message_accessor!(credentials => Vec<String>);
  impl_message_accessor!(callback_url => Option<Url>);
  impl_message_accessor!(response_requested => Option<bool>);
  impl_message_accessor!(id => Option<IotaDID>);
  impl_message_accessor!(timing => Option<Timing>);
}
