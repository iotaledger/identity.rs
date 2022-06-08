// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Object;
use identity_did::did::CoreDID;
use identity_did::did::DIDUrl;
use identity_did::did::DID;

/// Information used to determine the current status of a [`Credential`][crate::credential::Credential].
///
/// [More Info](https://www.w3.org/TR/vc-data-model/#status)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Status<D: DID = CoreDID, T = Object> {
  /// A Url identifying the credential status.
  pub id: DIDUrl<D>,
  /// The type(s) of the credential status.
  #[serde(rename = "type")]
  pub type_: String,
  /// Additional properties of the credential status.
  #[serde(flatten)]
  pub properties: T,
}

impl<D: DID> Status<D, Object> {
  /// Creates a new `Status`.
  pub fn new(id: DIDUrl<D>, type_: String) -> Self {
    Self::new_with_properties(id, type_, Object::new())
  }
}

impl<D: DID, T> Status<D, T> {
  /// Creates a new `Status` with the given `properties`.
  pub fn new_with_properties(id: DIDUrl<D>, type_: String, properties: T) -> Self {
    Self { id, type_, properties }
  }
}

#[cfg(test)]
mod tests {
  use identity_core::convert::FromJson;
  use identity_did::did::DIDUrl;

  use crate::credential::Status;

  const JSON: &str = include_str!("../../tests/fixtures/status-1.json");

  #[test]
  fn test_from_json() {
    let status: Status = Status::from_json(JSON).unwrap();
    assert_eq!(status.id, DIDUrl::parse("https://example.edu/status/24").unwrap());
    assert_eq!(status.type_, "CredentialStatusList2017".to_owned());
  }
}
