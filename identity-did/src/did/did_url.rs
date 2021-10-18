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

use did_url::DID as BaseDIDUrl;

use identity_core::diff::Diff;
use identity_core::diff::DiffString;

use crate::did::CoreDID;
use crate::did::DID;
use crate::did::DIDError;

pub type CoreDIDUrl = DIDUrl<CoreDID>;

/// A [DID Url]: a [DID] with [RelativeDIDUrl] components.
///
/// E.g. "did:iota:H3C2AVvLMv6gmMNam3uVAjZar3cJCwDwnZn6z3wXmqPV/path?query1=a&query2=b#fragment"
///
/// [DID Url](https://www.w3.org/TR/did-core/#did-url-syntax)
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

/// A [relative DID Url] with the [path], [query], and [fragment] components defined according
/// to [URI syntax](https://datatracker.ietf.org/doc/html/rfc5234).
///
/// E.g.
/// - `"/path?query#fragment"`
/// - `"/path"`
/// - `"?query"`
/// - `"#fragment"`
///
/// [relative DID Url](https://www.w3.org/TR/did-core/#relative-did-urls)
/// [path](https://www.w3.org/TR/did-core/#path)
/// [query](https://www.w3.org/TR/did-core/#query)
/// [fragment](https://www.w3.org/TR/did-core/#fragment)
#[derive(Clone, Default, PartialEq, Eq)]
pub struct RelativeDIDUrl {
  // Path including the leading '/'
  path: Option<String>,
  // Query including the leading '?'
  query: Option<String>,
  // Fragment including the leading '#'
  fragment: Option<String>,
}

impl RelativeDIDUrl {

  /// Create an empty [RelativeDIDUrl].
  pub fn new() -> Self {
    Self {
      fragment: None,
      path: None,
      query: None,
    }
  }

  /// Return the [path] component, including the leading '/'.
  ///
  /// E.g. `"/path/sub-path/resource"`
  ///
  /// [path](https://www.w3.org/TR/did-core/#path)
  pub fn path(&self) -> Option<&str> {
    self.path.as_deref()
  }

  /// Attempt to set the [path] component. The path must start with a '/'.
  ///
  /// # Example
  ///
  /// ```
  /// # use identity_did::did::RelativeDIDUrl;
  /// # let mut url = RelativeDIDUrl::new();
  /// url.set_path(Some("/path/sub-path/resource")).unwrap();
  /// assert_eq!(url.path().unwrap(), "/path/sub-path/resource");
  /// assert_eq!(url.to_string(), "/path/sub-path/resource");
  /// ```
  ///
  /// [path](https://www.w3.org/TR/did-core/#path)
  pub fn set_path(&mut self, value: Option<&str>) -> Result<(), DIDError> {
    self.path = value
      .filter(|s| !s.is_empty())
      .map(|s| if s.starts_with('/') {
          uriparse::Path::try_from(s)
            .map_err(|_| DIDError::InvalidPath)
            .map(|_| s.to_string())
        } else {
          Err(DIDError::InvalidPath)
        }).transpose()?;
    Ok(())
  }

  /// Return the [path] component, excluding the leading '?' delimiter.
  ///
  /// E.g. `"?query1=a&query2=b" -> "query1=a&query2=b"`
  ///
  /// [query](https://www.w3.org/TR/did-core/#query)
  pub fn query(&self) -> Option<&str> {
    self.query.as_deref().map(|query| &query[1..])
  }

  /// Attempt to set the [query] component. A leading '?' is ignored.
  ///
  /// # Example
  ///
  /// ```
  /// # use identity_did::did::RelativeDIDUrl;
  /// # let mut url = RelativeDIDUrl::new();
  /// // Set the query with a leading '?'
  /// url.set_query(Some("?query1=a")).unwrap();
  /// assert_eq!(url.query().unwrap(), "query1=a");
  /// assert_eq!(url.to_string(), "?query1=a");
  ///
  /// // Set the query without a leading '?'
  /// url.set_query(Some("query1=a&query2=b")).unwrap();
  /// assert_eq!(url.query().unwrap(), "query1=a&query2=b");
  /// assert_eq!(url.to_string(), "?query1=a&query2=b");
  /// ```
  ///
  /// [query](https://www.w3.org/TR/did-core/#query)
  pub fn set_query(&mut self, value: Option<&str>) -> Result<(), DIDError> {
    self.query = value
      .filter(|s| !s.is_empty())
      .map(|mut s| {
           // Ignore leading '?' during validation.
           if s.starts_with('?') {
             s = &s[1..];
           }
          uriparse::Query::try_from(s)
            .map_err(|_| DIDError::InvalidQuery)
            .map(|_| format!("?{}", s))
        }).transpose()?;
    Ok(())
  }

  /// Return an iterator of (name, value) pairs in the query string.
  ///
  /// E.g. `"query1=a&query2=b" -> [("query1", "a"), ("query2", "b")]`
  ///
  /// See [form_urlencoded::parse].
  pub fn query_pairs(&self) -> form_urlencoded::Parse<'_> {
    form_urlencoded::parse(self.query().unwrap_or_default().as_bytes())
  }

  /// Return the [fragment] component, excluding the leading '#' delimiter.
  ///
  /// E.g. `"#fragment" -> "fragment"`
  ///
  /// [fragment](https://www.w3.org/TR/did-core/#fragment)
  pub fn fragment(&self) -> Option<&str> {
    self.fragment.as_deref().map(|fragment| &fragment[1..])
  }

  /// Attempt to set the [fragment] component. A leading '#' is ignored.
  ///
  /// # Example
  ///
  /// ```
  /// # use identity_did::did::RelativeDIDUrl;
  /// # let mut url = RelativeDIDUrl::new();
  /// // Set the fragment with a leading '#'
  /// url.set_fragment(Some("#fragment1")).unwrap();
  /// assert_eq!(url.fragment().unwrap(), "fragment1");
  /// assert_eq!(url.to_string(), "#fragment1");
  ///
  /// // Set the fragment without a leading '#'
  /// url.set_fragment(Some("fragment2")).unwrap();
  /// assert_eq!(url.fragment().unwrap(), "fragment2");
  /// assert_eq!(url.to_string(), "#fragment2");
  /// ```
  ///
  /// [fragment](https://www.w3.org/TR/did-core/#fragment)
  pub fn set_fragment(&mut self, value: Option<&str>) -> Result<(), DIDError> {
    self.fragment = value
      .filter(|s| !s.is_empty())
      .map(|mut s| {
        // Ignore leading '#' during validation.
        if s.starts_with('#') {
          s = &s[1..];
        }
        uriparse::Fragment::try_from(s)
          .map_err(|_| DIDError::InvalidFragment)
          .map(|_| format!("#{}", s))
      }).transpose()?;
    Ok(())
  }
}

impl Display for RelativeDIDUrl {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.write_fmt(format_args!("{}{}{}",
                             self.path.as_deref().unwrap_or_default(),
                             self.query.as_deref().unwrap_or_default(),
                             self.fragment.as_deref().unwrap_or_default()))
  }
}

impl Debug for RelativeDIDUrl {
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
    let did_url: BaseDIDUrl = BaseDIDUrl::parse(input)?;
    Self::from_base_did_url(did_url)
  }

  fn from_base_did_url(did_url: BaseDIDUrl) -> Result<Self, DIDError> {
    // Extract relative DID URL
    let url: RelativeDIDUrl = {
      let mut url: RelativeDIDUrl = RelativeDIDUrl::new();
      url.set_path(Some(did_url.path()))?;
      url.set_query(did_url.query())?;
      url.set_fragment(did_url.fragment())?;
      url
    };

    // Extract base DID
    let did: T = {
      let mut base_did: BaseDIDUrl = did_url;
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
  ///
  /// See [`RelativeDIDUrl::path`].
  pub fn path(&self) -> Option<&str> {
    self.url.path()
  }

  /// Sets the `path` component of the [`DIDUrl`].
  pub fn set_path(&mut self, value: Option<&str>) -> Result<(), DIDError> {
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

  /// Append a string representing a path, query, and/or fragment to this [`DIDUrl`].
  pub fn join(self, segment: impl AsRef<str>) -> Result<Self, DIDError> {
    let segment: &str = segment.as_ref();

    // Accept only a relative path, query, or fragment to reject altering the method id segment.
    if !segment.starts_with('/')
      && !segment.starts_with('?')
      && !segment.starts_with('#') {
      return Err(DIDError::InvalidPath);
    }

    // Parse DID Url.
    let base_did_url: BaseDIDUrl = BaseDIDUrl::parse(self.to_string())?
      .join(segment)?;
    Self::from_base_did_url(base_did_url)
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
