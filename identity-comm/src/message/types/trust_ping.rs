// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
//! Provides a type used in a `ping message` from the [Trust-Ping Protocol](https://github.com/decentralized-identity/didcomm-messaging/blob/84e5a7c66c87440d39e93df81e4440855273f987/docs/spec-files/trustping.md)

use identity_core::common::Url;
use identity_iota::did::IotaDIDUrl;
use uuid::Uuid;

use crate::message::Timing;

/// A DIDComm `trust-ping` Message.
///
/// [Reference](https://github.com/decentralized-identity/didcomm-messaging/blob/84e5a7c66c87440d39e93df81e4440855273f987/docs/spec-files/trustping.md)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct TrustPing {
  context: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  thread: Option<Uuid>,
  #[serde(rename = "callbackURL", skip_serializing_if = "Option::is_none")]
  callback_url: Option<Url>,
  #[serde(rename = "responseRequested", skip_serializing_if = "Option::is_none")]
  response_requested: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  id: Option<IotaDIDUrl>,
  #[serde(skip_serializing_if = "Option::is_none")]
  timing: Option<Timing>,
}

impl TrustPing {
  /// Creates a new `TrustPing`.
  pub fn new(context: String) -> Self {
    Self {
      context,
      thread: None,
      callback_url: None,
      response_requested: None,
      id: None,
      timing: None,
    }
  }

  impl_message_accessor!(context => String);
  impl_message_accessor!(thread => Option<Uuid>);
  impl_message_accessor!(callback_url => Option<Url>);
  impl_message_accessor!(response_requested => Option<bool>);
  impl_message_accessor!(id => Option<IotaDIDUrl>);
  impl_message_accessor!(timing => Option<Timing>);
}
