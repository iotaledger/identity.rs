// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Url;
use identity_iota::did::IotaDID;
use identity_iota::did::IotaDocument;
use uuid::Uuid;

use crate::message::Timing;

/// A DIDComm `did-resolution` Request.
///
/// [Reference](https://github.com/iotaledger/identity.rs/blob/dev/docs/DID%20Communications%20Research%20and%20Specification/i_did-resolution.md#resolutionrequest)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ResolutionRequest {
  context: String,
  thread: Uuid,
  #[serde(rename = "callbackURL")]
  callback_url: Url,
  #[serde(rename = "responseRequested", skip_serializing_if = "Option::is_none")]
  response_requested: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  id: Option<IotaDID>,
  #[serde(skip_serializing_if = "Option::is_none")]
  timing: Option<Timing>,
}

impl ResolutionRequest {
  /// Creates a new `ResolutionRequest`
  pub fn new(context: String, thread: Uuid, callback_url: Url) -> Self {
    Self {
      context,
      thread,
      callback_url,
      response_requested: None,
      id: None,
      timing: None,
    }
  }

  impl_message_accessor!(context => String);
  impl_message_accessor!(thread => Uuid);
  impl_message_accessor!(callback_url => Url);
  impl_message_accessor!(response_requested => Option<bool>);
  impl_message_accessor!(id => Option<IotaDID>);
  impl_message_accessor!(timing => Option<Timing>);
}

/// A DIDComm `did-resolution` Response.
///
/// [Reference](https://github.com/iotaledger/identity.rs/blob/dev/docs/DID%20Communications%20Research%20and%20Specification/i_did-resolution.md#resolutionresponse)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ResolutionResponse {
  context: String,
  thread: Uuid,
  #[serde(rename = "didDocument")]
  did_document: IotaDocument,
  #[serde(rename = "callbackURL", skip_serializing_if = "Option::is_none")]
  callback_url: Option<Url>,
  #[serde(rename = "responseRequested", skip_serializing_if = "Option::is_none")]
  response_requested: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  timing: Option<Timing>,
}

impl ResolutionResponse {
  /// Creates a new `ResolutionResponse`
  pub fn new(context: String, thread: Uuid, did_document: IotaDocument) -> Self {
    Self {
      context,
      thread,
      did_document,
      callback_url: None,
      response_requested: None,
      timing: None,
    }
  }

  impl_message_accessor!(context => String);
  impl_message_accessor!(thread => Uuid);
  impl_message_accessor!(did_document => IotaDocument);
  impl_message_accessor!(callback_url => Option<Url>);
  impl_message_accessor!(response_requested => Option<bool>);
  impl_message_accessor!(timing => Option<Timing>);
}
