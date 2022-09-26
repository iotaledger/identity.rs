// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;
use core::fmt::Display;
use core::fmt::Formatter;

use identity_core::common::Object;
use identity_core::common::Timestamp;
use identity_core::convert::FmtJson;
use serde::Deserialize;
use serde::Serialize;

/// Additional attributes related to a [`IotaDocument`][crate::IotaDocument].
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct IotaDocumentMetadata {
  // TODO: store created in the immutable metadata, if possible?
  #[serde(skip_serializing_if = "Option::is_none")]
  pub created: Option<Timestamp>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub updated: Option<Timestamp>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub deactivated: Option<bool>,
  /// Bech32-encoded address of the governor unlock condition.
  #[serde(rename = "governorAddress", skip_serializing_if = "Option::is_none")]
  pub governor_address: Option<String>,
  /// Bech32-encoded address of the state controller unlock condition.
  #[serde(rename = "stateControllerAddress", skip_serializing_if = "Option::is_none")]
  pub state_controller_address: Option<String>,
  #[serde(flatten)]
  pub properties: Object,
}

impl IotaDocumentMetadata {
  /// Creates a new `IotaDocumentMetadata` with the current system datetime used for `created`
  /// and `updated` timestamps.
  pub fn new() -> Self {
    let now: Timestamp = Timestamp::now_utc();
    Self {
      created: Some(now),
      updated: Some(now),
      deactivated: None,
      governor_address: None,
      state_controller_address: None,
      properties: Object::default(),
    }
  }
}

impl Default for IotaDocumentMetadata {
  fn default() -> Self {
    Self::new()
  }
}

impl Display for IotaDocumentMetadata {
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    self.fmt_json(f)
  }
}
