// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Url;
use identity_iota::did::IotaDID;
use uuid::Uuid;

use crate::message::Timing;

/// A DIDComm `did-introduction` Request.
///
/// [Reference](https://github.com/iotaledger/identity.rs/blob/dev/docs/DID%20Communications%20Research%20and%20Specification/i_did-introduction.md#introductionproposal)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct IntroductionProposal {
  context: String,
  thread: Uuid,
  #[serde(rename = "callbackURL")]
  callback_url: Url,
  #[serde(rename = "responseRequested", skip_serializing_if = "Option::is_none")]
  response_requested: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  id: Option<IotaDID>,
  #[serde(skip_serializing_if = "Option::is_none")]
  comment: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  timing: Option<Timing>,
}

impl IntroductionProposal {
  /// Creates a new `IntroductionProposal`.
  pub fn new(context: String, thread: Uuid, callback_url: Url) -> Self {
    Self {
      context,
      thread,
      callback_url,
      response_requested: None,
      id: None,
      comment: None,
      timing: None,
    }
  }

  impl_message_accessor!(context => String);
  impl_message_accessor!(thread => Uuid);
  impl_message_accessor!(callback_url => Url);
  impl_message_accessor!(response_requested => Option<bool>);
  impl_message_accessor!(id => Option<IotaDID>);
  impl_message_accessor!(comment => Option<String>);
  impl_message_accessor!(timing => Option<Timing>);
}

/// A DIDComm `did-introduction` Response.
///
/// [Reference](https://github.com/iotaledger/identity.rs/blob/dev/docs/DID%20Communications%20Research%20and%20Specification/i_did-introduction.md#introductionresponse)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct IntroductionResponse {
  context: String,
  thread: Uuid,
  consent: bool,
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

impl IntroductionResponse {
  /// Creates a new `IntroductionResponse`.
  pub fn new(context: String, thread: Uuid, consent: bool) -> Self {
    Self {
      context,
      thread,
      consent,
      callback_url: None,
      response_requested: None,
      id: None,
      comment: None,
      timing: None,
    }
  }

  impl_message_accessor!(context => String);
  impl_message_accessor!(thread => Uuid);
  impl_message_accessor!(consent => bool);
  impl_message_accessor!(callback_url => Option<Url>);
  impl_message_accessor!(response_requested => Option<bool>);
  impl_message_accessor!(id => Option<IotaDID>);
  impl_message_accessor!(comment => Option<String>);
  impl_message_accessor!(timing => Option<Timing>);
}

/// A DIDComm `introduction` Message
///
/// [Reference](https://github.com/iotaledger/identity.rs/blob/dev/docs/DID%20Communications%20Research%20and%20Specification/i_did-introduction.md#introduction)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Introduction {
  context: String,
  thread: Uuid,
  ids: Vec<IotaDID>,
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

impl Introduction {
  /// Creates a new `Introduction`.
  pub fn new(context: String, thread: Uuid, ids: Vec<IotaDID>) -> Self {
    Self {
      context,
      thread,
      ids,
      callback_url: None,
      response_requested: None,
      id: None,
      comment: None,
      timing: None,
    }
  }

  impl_message_accessor!(context => String);
  impl_message_accessor!(thread => Uuid);
  impl_message_accessor!(ids => Vec<IotaDID>);
  impl_message_accessor!(callback_url => Option<Url>);
  impl_message_accessor!(response_requested => Option<bool>);
  impl_message_accessor!(id => Option<IotaDID>);
  impl_message_accessor!(comment => Option<String>);
  impl_message_accessor!(timing => Option<Timing>);
}
