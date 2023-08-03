// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::Deserialize;
use serde::Serialize;

use identity_core::common::Object;
use identity_core::common::Url;

/// A [`Credential`][crate::credential::Credential] issuer in object form.
///
/// [More Info](https://www.w3.org/TR/vc-data-model/#issuer)
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct IssuerData {
  /// A Url identifying the credential issuer.
  pub id: Url,
  /// Additional properties of the credential issuer.
  #[serde(flatten)]
  pub properties: Object,
}

/// An identifier representing the issuer of a [`Credential`][crate::credential::Credential].
///
/// [More Info](https://www.w3.org/TR/vc-data-model/#issuer)
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Issuer {
  /// A credential issuer expressed as a Url.
  Url(Url),
  /// A credential issuer expressed as a JSON object.
  Obj(IssuerData),
}

impl Issuer {
  /// Returns a reference to the credential issuer Url.
  pub fn url(&self) -> &Url {
    match self {
      Self::Url(url) => url,
      Self::Obj(obj) => &obj.id,
    }
  }
}
impl<T> From<T> for Issuer
where
  T: Into<Url>,
{
  fn from(other: T) -> Self {
    Self::Url(other.into())
  }
}

#[cfg(test)]
mod tests {
  use identity_core::convert::FromJson;

  use crate::credential::Issuer;

  const JSON1: &str = include_str!("../../tests/fixtures/issuer-1.json");
  const JSON2: &str = include_str!("../../tests/fixtures/issuer-2.json");

  #[test]
  fn test_from_json() {
    let issuer: Issuer = Issuer::from_json(JSON1).unwrap();
    assert!(matches!(issuer, Issuer::Url(_)));
    assert_eq!(issuer.url(), "https://example.edu/issuers/14");

    let issuer: Issuer = Issuer::from_json(JSON2).unwrap();
    assert!(matches!(issuer, Issuer::Obj(_)));
    assert_eq!(issuer.url(), "did:example:76e12ec712ebc6f1c221ebfeb1f");
  }
}
