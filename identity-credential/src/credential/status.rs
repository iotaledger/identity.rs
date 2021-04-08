// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Object;
use identity_core::common::OneOrMany;
use identity_core::common::Url;

/// Information used to determine the current status of a [`Credential`][crate::credential::Credential].
///
/// [More Info](https://www.w3.org/TR/vc-data-model/#status)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Status {
  /// A Url identifying the credential status.
  pub id: Url,
  /// The type(s) of the credential status.
  #[serde(rename = "type")]
  pub types: OneOrMany<String>,
  /// Additional properties of the credential status.
  #[serde(flatten)]
  pub properties: Object,
}

impl Status {
  /// Creates a new `Status`.
  pub fn new<T>(id: Url, types: T) -> Self
  where
    T: Into<OneOrMany<String>>,
  {
    Self::with_properties(id, types, Object::new())
  }

  /// Creates a new `Status` with the given `properties`.
  pub fn with_properties<T>(id: Url, types: T, properties: Object) -> Self
  where
    T: Into<OneOrMany<String>>,
  {
    Self {
      id,
      types: types.into(),
      properties,
    }
  }
}

#[cfg(test)]
mod tests {
  use identity_core::convert::FromJson;

  use crate::credential::Status;

  const JSON: &str = include_str!("../../tests/fixtures/status-1.json");

  #[test]
  fn test_from_json() {
    let status: Status = Status::from_json(JSON).unwrap();
    assert_eq!(status.id, "https://example.edu/status/24");
    assert_eq!(status.types.as_slice(), ["CredentialStatusList2017"]);
  }
}
