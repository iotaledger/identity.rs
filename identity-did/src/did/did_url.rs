// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::convert::TryFrom;
use core::fmt::Debug;
use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use core::str::FromStr;
use std::cmp::Ordering;
use std::convert::TryInto;
use std::hash::Hash;
use std::hash::Hasher;

use identity_core::diff::Diff;
use identity_core::diff::DiffString;

use crate::did::CoreDID;
use crate::did::DIDError;
use crate::did::DID;

// fn take_did(did: impl DID);
// fn take_did(did: IotaDID);
// fn take_did(did: CoreDID);
// fn take_did(did: AsRef<IotaDID>);
// fn take_did(did: AsRef<CoreDID>);
// fn take_did(did: Into<CoreDID>);

pub type CoreDIDUrl = DIDUrl<CoreDID>;

#[derive(Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(into = "String", try_from = "String")]
pub struct DIDUrl<T>
where
  Self: Sized,
  T: DID + Sized,
{
  did: T,
  url: RelativeDIDUrl,
}

#[derive(Clone, Default, PartialEq, Eq)]
pub struct RelativeDIDUrl {
  fragment: Option<String>,
  path: Option<String>,
  query: Option<String>,
}

impl RelativeDIDUrl {
  pub fn new() -> Self {
    Self {
      fragment: None,
      path: None,
      query: None,
    }
  }

  pub fn fragment(&self) -> Option<&str> {
    self.fragment.as_deref()
  }

  pub fn set_fragment(&mut self, value: Option<impl AsRef<str>>) -> Result<(), DIDError> {
    // TODO: validation
    self.fragment = value.map(|s| s.as_ref().to_owned());
    Ok(())
  }

  pub fn path(&self) -> Option<&str> {
    self.path.as_deref()
  }

  pub fn set_path(&mut self, value: Option<impl AsRef<str>>) -> Result<(), DIDError> {
    // TODO: validation
    self.path = value.map(|s| s.as_ref().to_owned());
    Ok(())
  }

  pub fn query(&self) -> Option<&str> {
    self.query.as_deref()
  }

  pub fn set_query(&mut self, value: Option<impl AsRef<str>>) -> Result<(), DIDError> {
    // TODO: validation
    self.query = value.map(|s| s.as_ref().to_owned());
    Ok(())
  }

  pub fn query_pairs(&self) -> form_urlencoded::Parse<'_> {
    form_urlencoded::parse(self.query().unwrap_or_default().as_bytes())
  }

  pub fn to_string(&self) -> String {
    // TODO: does this include the separator characters?
    format!(
      "{}{}{}",
      self.path().unwrap_or_default(),
      self.query().unwrap_or_default(),
      self.fragment().unwrap_or_default()
    )
  }
}

impl Debug for RelativeDIDUrl {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.write_fmt(format_args!("{}", self.to_string()))
  }
}

impl Display for RelativeDIDUrl {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.write_fmt(format_args!("{}", self.to_string()))
  }
}

impl PartialOrd for RelativeDIDUrl {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    // TODO: improve performance of comparisons
    self.to_string().partial_cmp(&other.to_string())
  }
}

impl Ord for RelativeDIDUrl {
  fn cmp(&self, other: &Self) -> Ordering {
    // TODO: improve performance of comparisons
    self.to_string().cmp(&other.to_string())
  }
}

impl Hash for RelativeDIDUrl {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.to_string().hash(state)
  }
}

impl<T> DIDUrl<T>
where
  T: DID + Sized,
{
  pub fn new(did: T, url: Option<RelativeDIDUrl>) -> Self {
    Self {
      did,
      url: url.unwrap_or_default(),
    }
  }

  pub fn parse(input: impl AsRef<str>) -> Result<Self, DIDError> {
    let did_url: did_url::DID = did_url::DID::parse(input)?;

    // Extract relative DID URL
    let url: RelativeDIDUrl = {
      let mut url: RelativeDIDUrl = RelativeDIDUrl::new();
      url.set_path(Some(did_url.path()).filter(|path| !path.is_empty()))?;
      url.set_query(did_url.query())?;
      url.set_fragment(did_url.fragment())?;
      url
    };

    // Extract base DID
    let did: T = {
      let mut base_did: did_url::DID = did_url;
      base_did.set_path("");
      base_did.set_query(None);
      base_did.set_fragment(None);
      // TODO: convert directly?
      T::from_str(base_did.as_str()).map_err(|_| DIDError::Other("invalid DID"))?
    };

    Ok(Self { did, url })
  }

  pub fn did(&self) -> &T {
    &self.did
  }

  pub fn url(&self) -> &RelativeDIDUrl {
    &self.url
  }

  pub fn set_url(&mut self, url: RelativeDIDUrl) {
    self.url = url
  }

  /// Returns the [`DIDUrl`] method fragment, if any.
  pub fn fragment(&self) -> Option<&str> {
    self.url.fragment()
  }

  /// Sets the `fragment` component of the [`DIDUrl`].
  pub fn set_fragment(&mut self, value: Option<&str>) -> Result<(), DIDError> {
    self.url.set_fragment(value)
  }

  /// Returns the [`DIDUrl`] path.
  /// TODO: check what this actually returns, only the path or also parts of the DID?
  pub fn path(&self) -> Option<&str> {
    self.url.path()
  }

  /// Sets the `path` component of the [`DIDUrl`].
  pub fn set_path(&mut self, value: Option<impl AsRef<str>>) -> Result<(), DIDError> {
    self.url.set_path(value)
  }

  /// Returns the [`DIDUrl`] method query, if any.
  pub fn query(&self) -> Option<&str> {
    self.url.query()
  }

  /// Sets the `query` component of the [`DIDUrl`].
  pub fn set_query(&mut self, value: Option<&str>) -> Result<(), DIDError> {
    self.url.set_query(value)
  }

  /// Parses the [`DIDUrl`] query and returns an iterator of (key, value) pairs.
  pub fn query_pairs(&self) -> form_urlencoded::Parse<'_> {
    self.url.query_pairs()
  }

  /// Appends a string representing a path, query, and/or fragment to this [`DIDUrl`].
  ///
  /// The string may also be a full did???
  /// TODO: figure this out...
  pub fn join(mut self, segment: impl AsRef<str>) -> Result<Self, DIDError> {
    // self.url = self.url.join(segment)?;
    // Ok(self)
    todo!()
  }

  pub fn to_string(&self) -> String {
    format!("{}{}", self.did.as_str(), self.url.to_string())
  }

  /// Attempt to convert a [`DIDUrl`] from a [`DIDUrl`] of a different DID method.
  ///
  /// Workaround for lack of specialisation preventing a generic `From` implementation.
  pub fn from<U>(other: DIDUrl<U>) -> Self
  where
    U: DID + Into<T>,
  {
    let did: T = other.did.into();
    Self { did, url: other.url }
  }

  /// Attempt to convert a [`DIDUrl`] from a [`DIDUrl`] of a different DID method.
  ///
  /// Workaround for lack of specialisation preventing a generic `TryFrom` implementation.
  pub fn try_from<U>(other: DIDUrl<U>) -> Result<Self, DIDError>
  where
    U: DID + TryInto<T>,
    U::Error: Into<DIDError>,
  {
    let did: T = other.did.try_into().map_err(|err| err.into())?;
    Ok(Self { did, url: other.url })
  }
}

impl<T> From<T> for DIDUrl<T>
where
  T: DID,
{
  fn from(did: T) -> Self {
    Self::new(did, None)
  }
}

impl<T> FromStr for DIDUrl<T>
where
  T: DID,
{
  type Err = DIDError;

  fn from_str(string: &str) -> Result<Self, Self::Err> {
    Self::parse(string)
  }
}

impl<T> TryFrom<String> for DIDUrl<T>
where
  T: DID,
{
  type Error = DIDError;

  fn try_from(other: String) -> Result<Self, Self::Error> {
    Self::parse(other)
  }
}

impl<T> From<DIDUrl<T>> for String
where
  T: DID,
{
  fn from(did_url: DIDUrl<T>) -> Self {
    did_url.to_string()
  }
}

impl<T> AsRef<T> for DIDUrl<T>
where
  T: DID,
{
  fn as_ref(&self) -> &T {
    &self.did
  }
}

impl<T> PartialOrd for DIDUrl<T>
where
  T: DID,
{
  #[inline]
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    match self.did().partial_cmp(other.did()) {
      None => None,
      Some(Ordering::Equal) => self.url().partial_cmp(other.url()),
      Some(ord) => Some(ord),
    }
  }
}

impl<T> Ord for DIDUrl<T>
where
  T: DID,
{
  #[inline]
  fn cmp(&self, other: &Self) -> Ordering {
    match self.did().cmp(other.did()) {
      Ordering::Equal => self.url().cmp(&other.url()),
      ord => ord,
    }
  }
}

impl<T> Hash for DIDUrl<T>
where
  T: DID,
{
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.to_string().hash(state)
  }
}

impl<T> Debug for DIDUrl<T>
where
  T: DID,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.write_fmt(format_args!("{}", self.to_string()))
  }
}

impl<T> Display for DIDUrl<T>
where
  T: DID,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.write_fmt(format_args!("{}", self.to_string()))
  }
}

impl<T> Diff for DIDUrl<T>
where
  T: DID,
{
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
