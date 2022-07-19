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

/// Additional attributes related to a [`StardustDocument`][crate::StardustDocument].
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct StardustDocumentMetadata {
  // TODO: store created in the immutable metadata, if possible?
  #[serde(skip_serializing_if = "Option::is_none")]
  pub created: Option<Timestamp>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub updated: Option<Timestamp>,
  #[serde(flatten)]
  pub properties: Object,
}

impl StardustDocumentMetadata {
  /// Creates a new `StardustDocumentMetadata` with the current system datetime used for `created`
  /// and `updated` timestamps.
  pub fn new() -> Self {
    let now: Timestamp = Timestamp::now_utc();
    Self {
      created: Some(now),
      updated: Some(now),
      properties: Object::default(),
    }
  }
}

impl Default for StardustDocumentMetadata {
  fn default() -> Self {
    Self::new()
  }
}

impl Display for StardustDocumentMetadata {
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    self.fmt_json(f)
  }
}
