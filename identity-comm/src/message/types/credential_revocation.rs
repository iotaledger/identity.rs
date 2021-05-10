// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Url;
use identity_iota::did::IotaDID;
use uuid::Uuid;

use crate::message::Timing;

/// A DIDComm `credential-revocation` Message
///
/// [Reference](https://github.com/iotaledger/identity.rs/blob/dev/docs/DID%20Communications%20Research%20and%20Specification/i_credential-revocation.md#revocation)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CredentialRevocation {
  context: String,
  thread: Uuid,
  #[serde(rename = "credentialId")]
  credential_id: String,
  #[serde(rename = "callbackURL", skip_serializing_if = "Option::is_none")]
  callback_url: Option<Url>,
  #[serde(rename = "responseRequested", skip_serializing_if = "Option::is_none")]
  response_requested: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  id: Option<IotaDID>,
  #[serde(skip_serializing_if = "Option::is_none")]
  comment: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  timing: Option<Timing>,
}

impl CredentialRevocation {
  /// Creates a new `CredentialRevocation`.
  pub fn new(context: String, thread: Uuid, credential_id: String) -> Self {
    Self {
      context,
      thread,
      credential_id,
      callback_url: None,
      response_requested: None,
      id: None,
      comment: None,
      timing: None,
    }
  }

  impl_message_accessor!(context => String);
  impl_message_accessor!(thread => Uuid);
  impl_message_accessor!(credential_id => String);
  impl_message_accessor!(callback_url => Option<Url>);
  impl_message_accessor!(response_requested => Option<bool>);
  impl_message_accessor!(id => Option<IotaDID>);
  impl_message_accessor!(comment => Option<String>);
  impl_message_accessor!(timing => Option<Timing>);
}
