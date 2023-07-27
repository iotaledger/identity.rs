// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::Deserialize;
use serde::Serialize;

use identity_core::common::Object;
use identity_core::common::OneOrMany;
use identity_core::common::Url;

/// Information used to validate the structure of a [`Credential`][crate::credential::Credential].
///
/// [More Info](https://www.w3.org/TR/vc-data-model/#data-schemas)
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Schema {
  /// A Url identifying the credential schema file.
  pub id: Url,
  /// The type(s) of the credential schema.
  #[serde(rename = "type")]
  pub types: OneOrMany<String>,
  /// Additional properties of the credential schema.
  #[serde(flatten)]
  pub properties: Object,
}

impl Schema {
  /// Creates a new `Schema`.
  pub fn new<T>(id: Url, types: T) -> Self
  where
    T: Into<OneOrMany<String>>,
  {
    Self::with_properties(id, types, Object::new())
  }

  /// Creates a new `Schema` with the given `properties`.
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

  use crate::credential::Schema;

  const JSON1: &str = include_str!("../../tests/fixtures/schema-1.json");
  const JSON2: &str = include_str!("../../tests/fixtures/schema-2.json");
  const JSON3: &str = include_str!("../../tests/fixtures/schema-3.json");

  #[test]
  fn test_from_json() {
    let schema: Schema = Schema::from_json(JSON1).unwrap();
    assert_eq!(schema.id, "https://example.org/examples/degree.json");
    assert_eq!(schema.types.as_slice(), ["JsonSchemaValidator2018"]);

    let schema: Schema = Schema::from_json(JSON2).unwrap();
    assert_eq!(schema.id, "https://example.org/examples/degree.zkp");
    assert_eq!(schema.types.as_slice(), ["ZkpExampleSchema2018"]);

    let schema: Schema = Schema::from_json(JSON3).unwrap();
    assert_eq!(schema.id, "did:example:cdf:35LB7w9ueWbagPL94T9bMLtyXDj9pX5o");
    assert_eq!(
      schema.types.as_slice(),
      ["did:example:schema:22KpkXgecryx9k7N6XN1QoN3gXwBkSU8SfyyYQG"]
    );
  }
}
