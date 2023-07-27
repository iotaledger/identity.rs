// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::Deserialize;
use serde::Serialize;

use identity_core::common::Object;
use identity_core::common::OneOrMany;
use identity_core::common::Url;

/// Information used to refresh or assert the status of a [`Credential`][crate::credential::Credential].
///
/// [More Info](https://www.w3.org/TR/vc-data-model/#refreshing)
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct RefreshService {
  /// The Url of the credential refresh service.
  pub id: Url,
  /// The type(s) of the credential refresh service.
  #[serde(rename = "type")]
  pub types: OneOrMany<String>,
  /// Additional properties of the credential refresh service.
  #[serde(flatten)]
  pub properties: Object,
}

impl RefreshService {
  /// Creates a new `RefreshService`.
  pub fn new<T>(id: Url, types: T) -> Self
  where
    T: Into<OneOrMany<String>>,
  {
    Self::with_properties(id, types, Object::new())
  }

  /// Creates a new `RefreshService` with the given `properties`.
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

  use crate::credential::RefreshService;

  const JSON: &str = include_str!("../../tests/fixtures/refresh-1.json");

  #[test]
  fn test_from_json() {
    let service: RefreshService = RefreshService::from_json(JSON).unwrap();
    assert_eq!(service.id, "https://example.edu/refresh/3732");
    assert_eq!(service.types.as_slice(), ["ManualRefreshService2018"]);
  }
}
