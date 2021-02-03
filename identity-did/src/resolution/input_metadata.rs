// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Object;

/// The content type of a JSON DID Document.
pub const MIME_DID: &str = "application/did+json";

/// The content type of a JSON-LD DID Document.
pub const MIME_DID_LD: &str = "application/did+ld+json";

// TODO: Support versioning via `version-id`/`version-time`
// TODO: Support caching via `no-cache`

/// Input options used to configure a [DID resolution][SPEC] process.
///
/// [SPEC]: https://www.w3.org/TR/did-core/#dfn-did-resolution
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputMetadata {
  /// The MIME type of the preferred representation of the DID document.
  ///
  /// Note: This is only relevant when using stream-based resolution.
  ///
  /// [More Info](https://www.w3.org/TR/did-spec-registries/#accept)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub accept: Option<String>,
  /// Additional input metadata properties.
  #[serde(flatten)]
  pub properties: Object,
}

impl InputMetadata {
  /// Creates a new [`InputMetadata`].
  pub fn new() -> Self {
    Self {
      accept: None,
      properties: Object::new(),
    }
  }
}
