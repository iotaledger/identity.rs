// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::convert::TryFrom;
use core::fmt::Debug;
use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use core::str::FromStr;
use std::hash::Hash;

use did_url::DID as BaseDIDUrl;

use identity_core::diff::Diff;
use identity_core::diff::DiffString;

use crate::did::DIDError;
use crate::did::DIDUrl;
use crate::error::Error;

pub trait DID: Clone + PartialEq + Eq + PartialOrd + Ord + Hash + FromStr {
  const SCHEME: &'static str = BaseDIDUrl::SCHEME;

  /// Returns the [`DID`] scheme. See [`DID::SCHEME`].
  fn scheme(&self) -> &'static str;

  /// Returns the [`DID`] authority.
  fn authority(&self) -> &str;

  /// Returns the [`DID`] method name.
  fn method(&self) -> &str;

  /// Returns the [`DID`] method-specific ID.
  fn method_id(&self) -> &str;

  /// Returns the serialized [`DID`].
  ///
  /// This is fast since the serialized value is stored in the [`DID`].
  fn as_str(&self) -> &str;

  /// Consumes the [`DID`] and returns its serialization.
  fn into_string(self) -> String;

  /// Constructs a [`DIDUrl`] by attempting to append a string representing a path, query, or
  /// fragment to this [`DID`].
  fn join(self, value: impl AsRef<str>) -> Result<DIDUrl<Self>, DIDError>
  where
    Self: Sized;

  /// Clones the [`DID`] into a [`DIDUrl`] of the same method.
  fn to_url(&self) -> DIDUrl<Self>;

  /// Converts the [`DID`] into a [`DIDUrl`] of the same method.
  fn into_url(self) -> DIDUrl<Self>;
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Deserialize, serde::Serialize)]
#[serde(into = "BaseDIDUrl", try_from = "BaseDIDUrl")]
pub struct CoreDID(BaseDIDUrl);

impl CoreDID {
  /// Parses a [`DID`] from the given `input`.
  ///
  /// # Errors
  ///
  /// Returns `Err` if the input is not a valid [`DID`].
  pub fn parse(input: impl AsRef<str>) -> Result<Self, DIDError> {
    let did_url = BaseDIDUrl::parse(input).map_err(DIDError::from)?;
    let _ = Self::check_validity(&did_url)?;
    Ok(Self(did_url))
  }

  /// Change the method-specific-id of the [`DID`].
  pub fn set_method_id(&mut self, value: impl AsRef<str>) -> Result<(), DIDError> {
    // TODO: validate method_id contents?
    self.0.set_method_id(value);
    todo!()
  }

  /// Checks if the given `DID` is valid according to the [`DID`] method
  /// specification.
  ///
  /// # Errors
  ///
  /// Returns `Err` if the input is not a valid [`IotaDID`].
  pub fn check_validity(did: &BaseDIDUrl) -> Result<(), DIDError> {
    if !did.path().is_empty() || !did.fragment().is_none() || !did.query().is_none() {
      return Err(DIDError::InvalidMethodId);
    }

    Ok(())
  }
}

impl DID for CoreDID {
  fn scheme(&self) -> &'static str {
    self.0.scheme()
  }

  /// TODO: examples
  /// e.g. did:example:12345678 -> example:12345678
  /// e.g. did:iota:main:12345678 -> iota:main:12345678
  fn authority(&self) -> &str {
    self.0.authority()
  }

  fn method(&self) -> &str {
    self.0.method()
  }

  fn method_id(&self) -> &str {
    self.0.method_id()
  }

  fn as_str(&self) -> &str {
    self.0.as_str()
  }

  fn into_string(self) -> String {
    self.0.into_string()
  }

  fn join(self, value: impl AsRef<str>) -> Result<DIDUrl<Self>, DIDError> {
    DIDUrl::new(self, None).join(value)
  }

  fn to_url(&self) -> DIDUrl<Self> {
    DIDUrl::new(self.clone(), None)
  }

  fn into_url(self) -> DIDUrl<Self> {
    DIDUrl::new(self, None)
  }
}

impl Into<BaseDIDUrl> for CoreDID {
  fn into(self) -> BaseDIDUrl {
    self.0
  }
}

impl TryFrom<BaseDIDUrl> for CoreDID {
  type Error = Error;

  fn try_from(value: BaseDIDUrl) -> std::prelude::rust_2015::Result<Self, Self::Error> {
    let _ = Self::check_validity(&value)?;
    Ok(Self(value))
  }
}

impl Debug for CoreDID {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.write_fmt(format_args!("{}", self.as_str()))
  }
}

impl Display for CoreDID {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.write_fmt(format_args!("{}", self.as_str()))
  }
}

impl AsRef<str> for CoreDID {
  fn as_ref(&self) -> &str {
    self.0.as_ref()
  }
}

impl FromStr for CoreDID {
  type Err = DIDError;

  fn from_str(string: &str) -> Result<Self, Self::Err> {
    Self::parse(string)
  }
}

impl TryFrom<&str> for CoreDID {
  type Error = DIDError;

  fn try_from(other: &str) -> Result<Self, Self::Error> {
    Self::parse(other)
  }
}

impl TryFrom<String> for CoreDID {
  type Error = DIDError;

  fn try_from(other: String) -> Result<Self, Self::Error> {
    Self::parse(other)
  }
}

impl From<CoreDID> for String {
  fn from(did: CoreDID) -> Self {
    did.into_string()
  }
}

impl Diff for CoreDID {
  type Type = DiffString;

  fn diff(&self, other: &Self) -> identity_core::diff::Result<Self::Type> {
    self.to_string().diff(&other.to_string())
  }

  fn merge(&self, diff: Self::Type) -> identity_core::diff::Result<Self> {
    self
      .to_string()
      .merge(diff)
      .and_then(|this| Self::parse(&this).map_err(identity_core::diff::Error::merge))
  }

  fn from_diff(diff: Self::Type) -> identity_core::diff::Result<Self> {
    String::from_diff(diff).and_then(|this| Self::parse(&this).map_err(identity_core::diff::Error::convert))
  }

  fn into_diff(self) -> identity_core::diff::Result<Self::Type> {
    self.to_string().into_diff()
  }
}

impl PartialEq<str> for CoreDID {
  fn eq(&self, other: &str) -> bool {
    self.as_str() == other
  }
}

impl PartialEq<&'_ str> for CoreDID {
  fn eq(&self, other: &&'_ str) -> bool {
    self == *other
  }
}

impl PartialEq<&CoreDID> for CoreDID {
  fn eq(&self, other: &&CoreDID) -> bool {
    self == other
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test() {
    let did = BaseDIDUrl::parse("did:iota:main:1234567").unwrap();
    todo!("add more tests");
    panic!("{:?}", did.join("did:asdf"));
  }
}
