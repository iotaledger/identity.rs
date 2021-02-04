// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Object;
use identity_core::common::Timestamp;

/// Metadata associated with a resolved DID Document.
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct DocumentMetadata {
  /// The timestamp of the Create operation.
  ///
  /// [More Info](https://www.w3.org/TR/did-spec-registries/#created)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub created: Option<Timestamp>,
  /// The timestamp of the last Update operation.
  ///
  /// [More Info](https://www.w3.org/TR/did-spec-registries/#updated)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub updated: Option<Timestamp>,
  /// Additional document metadata properties.
  #[serde(flatten)]
  pub properties: Object,
}

impl DocumentMetadata {
  /// Creates a new [`DocumentMetadata`].
  pub fn new() -> Self {
    Self {
      created: None,
      updated: None,
      properties: Object::new(),
    }
  }
}
