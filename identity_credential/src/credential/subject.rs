// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::Deserialize;
use serde::Serialize;

use identity_core::common::Object;
use identity_core::common::Url;

/// An entity who is the target of a set of claims.
///
/// [More Info](https://www.w3.org/TR/vc-data-model/#credential-subject)
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct Subject {
  /// A URI identifying the credential subject.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub id: Option<Url>,
  /// Additional properties of the credential subject.
  #[serde(flatten)]
  pub properties: Object,
}

impl Subject {
  /// Creates a new `Subject`.
  pub fn new() -> Self {
    Self::with_properties(Object::new())
  }

  /// Creates a new `Subject` with the given `id`.
  pub fn with_id(id: Url) -> Self {
    Self::with_id_and_properties(id, Object::new())
  }

  /// Creates a new `Subject` with the given `properties`.
  pub fn with_properties(properties: Object) -> Self {
    Self { id: None, properties }
  }

  /// Creates a new `Subject` with the given `id` and `properties`.
  pub fn with_id_and_properties(id: Url, properties: Object) -> Self {
    Self {
      id: Some(id),
      properties,
    }
  }
}

#[cfg(test)]
mod tests {
  use identity_core::convert::FromJson;

  use crate::credential::Subject;

  const JSON1: &str = include_str!("../../tests/fixtures/subject-1.json");
  const JSON2: &str = include_str!("../../tests/fixtures/subject-2.json");
  const JSON3: &str = include_str!("../../tests/fixtures/subject-3.json");
  const JSON4: &str = include_str!("../../tests/fixtures/subject-4.json");
  const JSON5: &str = include_str!("../../tests/fixtures/subject-5.json");
  const JSON6: &str = include_str!("../../tests/fixtures/subject-6.json");
  const JSON7: &str = include_str!("../../tests/fixtures/subject-7.json");
  const JSON8: &str = include_str!("../../tests/fixtures/subject-8.json");
  const JSON9: &str = include_str!("../../tests/fixtures/subject-9.json");
  const JSON10: &str = include_str!("../../tests/fixtures/subject-10.json");

  #[test]
  fn test_from_json() {
    let subject: Subject = Subject::from_json(JSON1).unwrap();
    assert_eq!(subject.id.unwrap(), "did:example:ebfeb1f712ebc6f1c276e12ec21");
    assert_eq!(
      subject.properties["alumniOf"]["id"],
      "did:example:c276e12ec21ebfeb1f712ebc6f1"
    );
    assert_eq!(subject.properties["alumniOf"]["name"][0]["value"], "Example University");
    assert_eq!(subject.properties["alumniOf"]["name"][0]["lang"], "en");
    assert_eq!(
      subject.properties["alumniOf"]["name"][1]["value"],
      "Exemple d'Université"
    );
    assert_eq!(subject.properties["alumniOf"]["name"][1]["lang"], "fr");

    let subject: Subject = Subject::from_json(JSON2).unwrap();
    assert_eq!(subject.id.unwrap(), "did:example:ebfeb1f712ebc6f1c276e12ec21");
    assert_eq!(subject.properties["degree"]["type"], "BachelorDegree");
    assert_eq!(subject.properties["degree"]["name"], "Bachelor of Science and Arts");

    let subject: Subject = Subject::from_json(JSON3).unwrap();
    assert_eq!(subject.id.unwrap(), "did:example:abcdef1234567");
    assert_eq!(subject.properties["name"], "Jane Doe");

    let subject: Subject = Subject::from_json(JSON4).unwrap();
    assert_eq!(subject.id.unwrap(), "did:example:abcdef1234567");
    assert_eq!(subject.properties["name"], "Jane Doe");
    assert_eq!(subject.properties["favoriteFood"], "Papaya");

    let subject: Subject = Subject::from_json(JSON5).unwrap();
    assert_eq!(subject.properties["givenName"], "Jane");
    assert_eq!(subject.properties["familyName"], "Doe");
    assert_eq!(subject.properties["degree"]["type"], "BachelorDegree");
    assert_eq!(subject.properties["degree"]["name"], "Bachelor of Science and Arts");
    assert_eq!(subject.properties["degree"]["college"], "College of Engineering");

    let subject: Subject = Subject::from_json(JSON6).unwrap();
    assert_eq!(subject.properties["degreeType"], "BachelorDegree");
    assert_eq!(subject.properties["degreeSchool"], "College of Engineering");

    let subject: Subject = Subject::from_json(JSON7).unwrap();
    assert_eq!(subject.id.unwrap(), "http://example.com/credentials/245");
    assert_eq!(subject.properties["currentStatus"], "Disputed");
    assert_eq!(subject.properties["statusReason"]["value"], "Address is out of date.");
    assert_eq!(subject.properties["statusReason"]["lang"], "en");

    let subject: Subject = Subject::from_json(JSON8).unwrap();
    assert_eq!(subject.properties["degree"]["type"], "BachelorDegree");
    assert_eq!(subject.properties["degree"]["name"], "Bachelor of Science and Arts");

    let subject: Subject = Subject::from_json(JSON9).unwrap();
    assert_eq!(subject.id.unwrap(), "did:example:ebfeb1f712ebc6f1c276e12ec21");
    assert_eq!(subject.properties["image"], "https://example.edu/images/58473");
    assert_eq!(
      subject.properties["alumniOf"]["id"],
      "did:example:c276e12ec21ebfeb1f712ebc6f1"
    );
    assert_eq!(subject.properties["alumniOf"]["name"][0]["value"], "Example University");
    assert_eq!(subject.properties["alumniOf"]["name"][0]["lang"], "en");
    assert_eq!(
      subject.properties["alumniOf"]["name"][1]["value"],
      "Exemple d'Université"
    );
    assert_eq!(subject.properties["alumniOf"]["name"][1]["lang"], "fr");

    let subject: Subject = Subject::from_json(JSON10).unwrap();
    assert_eq!(subject.id.unwrap(), "did:example:ebfeb1f712ebc6f1c276e12ec21");
    assert_eq!(
      subject.properties["image"],
      "ipfs:/ipfs/QmXfrS3pHerg44zzK6QKQj6JDk8H6cMtQS7pdXbohwNQfK/image"
    );
    assert_eq!(
      subject.properties["alumniOf"]["id"],
      "did:example:c276e12ec21ebfeb1f712ebc6f1"
    );
    assert_eq!(subject.properties["alumniOf"]["name"][0]["value"], "Example University");
    assert_eq!(subject.properties["alumniOf"]["name"][0]["lang"], "en");
    assert_eq!(
      subject.properties["alumniOf"]["name"][1]["value"],
      "Exemple d'Université"
    );
    assert_eq!(subject.properties["alumniOf"]["name"][1]["lang"], "fr");
  }
}
