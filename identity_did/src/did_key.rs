use std::fmt::Debug;
use std::fmt::Display;
use std::str::FromStr;

use crate::CoreDID;
use crate::DIDUrl;
use crate::Error;
use crate::DID;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Deserialize, serde::Serialize)]
#[repr(transparent)]
#[serde(into = "DIDUrl", try_from = "DIDUrl")]
/// A type representing a `did:key` DID.
pub struct DIDKey(DIDUrl);

impl DIDKey {
  /// [`DIDKey`]'s method.
  pub const METHOD: &'static str = "key";

  /// Tries to parse a [`DIDKey`] from a string.
  pub fn parse(s: &str) -> Result<Self, Error> {
    s.parse()
  }

  /// Returns this [`DIDKey`]'s optional fragment.
  pub fn fragment(&self) -> Option<&str> {
    self.0.fragment()
  }

  /// Sets the fragment of this [`DIDKey`].
  pub fn set_fragment(&mut self, fragment: Option<&str>) -> Result<(), Error> {
    self.0.set_fragment(fragment)
  }
}

impl AsRef<CoreDID> for DIDKey {
  fn as_ref(&self) -> &CoreDID {
    self.0.did()
  }
}

impl From<DIDKey> for CoreDID {
  fn from(value: DIDKey) -> Self {
    value.0.did().clone()
  }
}

impl<'a> TryFrom<&'a str> for DIDKey {
  type Error = Error;
  fn try_from(value: &'a str) -> Result<Self, Self::Error> {
    value.parse()
  }
}

impl TryFrom<DIDUrl> for DIDKey {
  type Error = Error;
  fn try_from(value: DIDUrl) -> Result<Self, Self::Error> {
    if value.did().method() != Self::METHOD {
      Err(Error::InvalidMethodName)
    } else if value.path().is_some() {
      Err(Error::InvalidPath)
    } else if value.query().is_some() {
      Err(Error::InvalidQuery)
    } else if !value.did().method_id().starts_with('z') {
      Err(Error::InvalidMethodId)
    } else {
      Ok(Self(value))
    }
  }
}

impl Display for DIDKey {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl FromStr for DIDKey {
  type Err = Error;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    s.parse::<DIDUrl>().and_then(TryFrom::try_from)
  }
}

impl From<DIDKey> for String {
  fn from(value: DIDKey) -> Self {
    value.to_string()
  }
}

impl TryFrom<CoreDID> for DIDKey {
  type Error = Error;
  fn try_from(value: CoreDID) -> Result<Self, Self::Error> {
    DIDUrl::new(value, None).try_into()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_valid_deserialization() -> Result<(), Error> {
    "did:key:z6MkiTBz1ymuepAQ4HEHYSF1H8quG5GLVVQR3djdX3mDooWp".parse::<DIDKey>()?;
    "did:key:z6MkiTBz1ymuepAQ4HEHYSF1H8quG5GLVVQR3djdX3mDooWp#afragment".parse::<DIDKey>()?;

    Ok(())
  }

  #[test]
  fn test_invalid_serialization() {
    assert!(
      "did:iota:0xf4d6f08f5a1b80dd578da7dc1b49c886d580acd4cf7d48119dfeb82b538ad88a"
        .parse::<DIDKey>()
        .is_err()
    );
    assert!("did:key:".parse::<DIDKey>().is_err());
    assert!("did:key:z6MkiTBz1ymuepAQ4HEHYSF1H8quG5GLVVQR3djdX3mDooWp/"
      .parse::<DIDKey>()
      .is_err());
    assert!("did:key:z6MkiTBz1ymuepAQ4HEHYSF1H8quG5GLVVQR3djdX3mDooWp/somepath"
      .parse::<DIDKey>()
      .is_err());
    assert!("did:key:z6MkiTBz1ymuepAQ4HEHYSF1H8quG5GLVVQR3djdX3mDooWp?somequery"
      .parse::<DIDKey>()
      .is_err());
    assert!("did:key:6MkiTBz1ymuepAQ4HEHYSF1H8quG5GLVVQR3djdX3mDooWp"
      .parse::<DIDKey>()
      .is_err());
  }
}
