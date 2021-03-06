// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Url;
use identity_iota::did::IotaDID;
use uuid::Uuid;

use crate::message::Timing;

/// A DIDComm `credential-schema` Request
///
/// [Reference](https://github.com/iotaledger/identity.rs/blob/dev/docs/DID%20Communications%20Research%20and%20Specification/i_credential-schema.md#credentialschemarequest)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CredentialSchemaRequest {
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

impl CredentialSchemaRequest {
  /// Creates a new `CredentialSchemaRequest`.
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

/// A DIDComm `credential-schema` Response
///
/// [Reference](https://github.com/iotaledger/identity.rs/blob/dev/docs/DID%20Communications%20Research%20and%20Specification/i_credential-schema.md#credentialschemaresponse)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CredentialSchemaResponse {
  context: String,
  thread: String,
  schemata: Vec<String>,
  #[serde(rename = "callbackURL", skip_serializing_if = "Option::is_none")]
  callback_url: Option<Url>,
  #[serde(rename = "responseRequested", skip_serializing_if = "Option::is_none")]
  response_requested: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  id: Option<IotaDID>,
  #[serde(skip_serializing_if = "Option::is_none")]
  timing: Option<Timing>,
}

impl CredentialSchemaResponse {
  /// Creates a new `CredentialSchemaResponse`.
  pub fn new(context: String, thread: String, schemata: Vec<String>, callback_url: Option<Url>) -> Self {
    Self {
      context,
      thread,
      schemata,
      callback_url,
      response_requested: None,
      id: None,
      timing: None,
    }
  }

  impl_message_accessor!(context => String);
  impl_message_accessor!(thread => String);
  impl_message_accessor!(schemata => Vec<String>);
  impl_message_accessor!(callback_url => Option<Url>);
  impl_message_accessor!(response_requested => Option<bool>);
  impl_message_accessor!(id => Option<IotaDID>);
  impl_message_accessor!(timing => Option<Timing>);
}
