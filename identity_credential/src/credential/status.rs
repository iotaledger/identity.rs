// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::Deserialize;
use serde::Serialize;

use identity_core::common::Object;
use identity_core::common::Url;

use crate::revocation::status_list_2021::StatusList2021Entry;

/// Credential status interface
pub trait CredentialStatus {
  /// `credentialStatus.type` field
  fn r#type(&self) -> &str;
  /// `credentialStatus.id` field
  fn id(&self) -> &Url;
}

/// Information used to determine the current status of a [`Credential`][crate::credential::Credential].
///
/// [More Info](https://www.w3.org/TR/vc-data-model/#status)
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Status<T = Object> {
  /// CredentialStatus using [`StatusList2021Entry`]
  StatusList2021(StatusList2021Entry),
  /// Any other status
  Other(CustomStatus<T>),
}

impl CredentialStatus for Status {
  fn id(&self) -> &Url {
    match self {
      Self::StatusList2021(s) => s.id(),
      Self::Other(s) => s.id(),
    }
  }
  fn r#type(&self) -> &str {
    match self {
      Self::StatusList2021(s) => s.r#type(),
      Self::Other(s) => s.r#type(),
    }
  }
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
/// A weakly type credential status that can encode any status
pub struct CustomStatus<T = Object> {
  /// A Url identifying the credential status.
  pub id: Url,
  /// The type(s) of the credential status.
  #[serde(rename = "type")]
  pub type_: String,
  /// Additional properties of the credential status.
  #[serde(flatten)]
  pub properties: T,
}

impl<T> CredentialStatus for CustomStatus<T> {
  fn id(&self) -> &Url {
    &self.id
  }
  fn r#type(&self) -> &str {
    &self.type_
  }
}

impl CustomStatus<Object> {
  /// Creates a new `Status`.
  pub fn new(id: Url, type_: String) -> Self {
    Self::new_with_properties(id, type_, Object::new())
  }
}

impl<T> CustomStatus<T> {
  /// Creates a new `Status` with the given `properties`.
  pub fn new_with_properties(id: Url, type_: String, properties: T) -> Self {
    Self { id, type_, properties }
  }
}

#[cfg(test)]
mod tests {
  use identity_core::convert::FromJson;

  use super::*;

  const JSON: &str = include_str!("../../tests/fixtures/status-1.json");

  #[test]
  fn test_from_json() {
    let status = Status::from_json(JSON).unwrap();
    assert_eq!(status.id().as_str(), "https://example.edu/status/24");
    assert_eq!(status.r#type(), "CredentialStatusList2017");
  }
}
