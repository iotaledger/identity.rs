// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
//! Provides types loosely representing the request and response roles from the [Discover Features Protocol](https://github.com/decentralized-identity/didcomm-messaging/blob/84e5a7c66c87440d39e93df81e4440855273f987/docs/spec-files/feature_discovery.md#discover-features-protocol-10)

use identity_core::common::Url;
use identity_iota::did::IotaDIDUrl;
use uuid::Uuid;

use crate::message::Timing;

/// Analogue of a [DIDComm `features-discovery` Request](https://github.com/decentralized-identity/didcomm-messaging/blob/84e5a7c66c87440d39e93df81e4440855273f987/docs/spec-files/feature_discovery.md#discover-features-protocol-10)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct FeaturesRequest {
  context: String,
  thread: Uuid,
  #[serde(rename = "callbackURL")]
  callback_url: Url,
  #[serde(rename = "responseRequested", skip_serializing_if = "Option::is_none")]
  response_requested: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  id: Option<IotaDIDUrl>,
  #[serde(skip_serializing_if = "Option::is_none")]
  timing: Option<Timing>,
}

impl FeaturesRequest {
  /// Creates a new `FeaturesRequest`.
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
  impl_message_accessor!(id => Option<IotaDIDUrl>);
  impl_message_accessor!(timing => Option<Timing>);
}

/// Analogue of a [DIDComm `features-discovery` Response](https://github.com/decentralized-identity/didcomm-messaging/blob/84e5a7c66c87440d39e93df81e4440855273f987/docs/spec-files/feature_discovery.md#discover-features-protocol-10)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct FeaturesResponse {
  context: String,
  thread: Uuid,
  features: Vec<String>,
  #[serde(rename = "callbackURL", skip_serializing_if = "Option::is_none")]
  callback_url: Option<Url>,
  #[serde(rename = "responseRequested", skip_serializing_if = "Option::is_none")]
  response_requested: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  timing: Option<Timing>,
}

impl FeaturesResponse {
  /// Creates a new `FeaturesResponse`.
  pub fn new(context: String, thread: Uuid, features: Vec<String>) -> Self {
    Self {
      context,
      thread,
      features,
      callback_url: None,
      response_requested: None,
      timing: None,
    }
  }

  impl_message_accessor!(context => String);
  impl_message_accessor!(thread => Uuid);
  impl_message_accessor!(features => Vec<String>);
  impl_message_accessor!(callback_url => Option<Url>);
  impl_message_accessor!(response_requested => Option<bool>);
  impl_message_accessor!(timing => Option<Timing>);
}
