// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::Deserialize;
use serde::Serialize;

use identity_core::common::Object;
use identity_core::common::OneOrMany;

/// Information used to increase confidence in the claims of a `Credential`
///
/// [More Info](https://www.w3.org/TR/vc-data-model/#evidence)
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct Evidence {
  /// A Url that allows retrieval of information about the evidence.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub id: Option<String>,
  /// The type(s) of the credential evidence.
  #[serde(rename = "type")]
  pub types: OneOrMany<String>,
  /// Additional properties of the credential evidence.
  #[serde(flatten)]
  pub properties: Object,
}

impl Evidence {
  /// Creates a new `Evidence` instance.
  pub fn new<T>(types: T) -> Self
  where
    T: Into<OneOrMany<String>>,
  {
    Self::with_properties(types, Object::new())
  }

  /// Creates a new `Evidence` instance with the given `id`.
  pub fn with_id<T, U>(types: T, id: U) -> Self
  where
    T: Into<OneOrMany<String>>,
    U: Into<String>,
  {
    Self {
      id: Some(id.into()),
      types: types.into(),
      properties: Object::new(),
    }
  }

  /// Creates a new `Evidence` instance with the given `properties`.
  pub fn with_properties<T>(types: T, properties: Object) -> Self
  where
    T: Into<OneOrMany<String>>,
  {
    Self {
      id: None,
      types: types.into(),
      properties,
    }
  }

  /// Creates a new `Evidence` instance with the given `id` and `properties`.
  pub fn with_id_and_properties<T, U, V>(types: T, id: U, properties: Object) -> Self
  where
    T: Into<OneOrMany<String>>,
    U: Into<String>,
  {
    Self {
      id: Some(id.into()),
      types: types.into(),
      properties,
    }
  }
}

#[cfg(test)]
mod tests {
  use identity_core::convert::FromJson;

  use crate::credential::Evidence;

  const JSON1: &str = include_str!("../../tests/fixtures/evidence-1.json");
  const JSON2: &str = include_str!("../../tests/fixtures/evidence-2.json");

  #[test]
  fn test_from_json() {
    let evidence: Evidence = Evidence::from_json(JSON1).unwrap();
    assert_eq!(
      evidence.id.unwrap(),
      "https://example.edu/evidence/f2aeec97-fc0d-42bf-8ca7-0548192d4231"
    );
    assert_eq!(evidence.types.as_slice(), ["DocumentVerification"]);
    assert_eq!(evidence.properties["verifier"], "https://example.edu/issuers/14");
    assert_eq!(evidence.properties["evidenceDocument"], "DriversLicense");
    assert_eq!(evidence.properties["subjectPresence"], "Physical");
    assert_eq!(evidence.properties["documentPresence"], "Physical");

    let evidence: Evidence = Evidence::from_json(JSON2).unwrap();
    assert_eq!(
      evidence.id.unwrap(),
      "https://example.edu/evidence/f2aeec97-fc0d-42bf-8ca7-0548192dxyzab"
    );
    assert_eq!(evidence.types.as_slice(), ["SupportingActivity"]);
    assert_eq!(evidence.properties["verifier"], "https://example.edu/issuers/14");
    assert_eq!(evidence.properties["evidenceDocument"], "Fluid Dynamics Focus");
    assert_eq!(evidence.properties["subjectPresence"], "Digital");
    assert_eq!(evidence.properties["documentPresence"], "Digital");
  }
}
