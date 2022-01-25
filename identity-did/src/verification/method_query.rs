// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::borrow::Cow;

use identity_core::crypto::Signature;

use crate::did::CoreDID;
use crate::did::CoreDIDUrl;
use crate::did::DIDUrl;
use crate::did::RelativeDIDUrl;
use crate::did::DID;

/// Specifies the conditions of a DID document method resolution query.
///
/// See `Document::resolve`.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct MethodQuery<'query>(Cow<'query, str>);

impl<'query> MethodQuery<'query> {
  /// Returns whether this query matches the given DIDUrl.
  pub(crate) fn matches(&self, did_url: &CoreDIDUrl) -> bool {
    // Ensure the DID matches if included in the query.
    if let Some(did_str) = self.did_str() {
      if did_str != did_url.did().as_str() {
        return false;
      }
    }

    // Compare fragments.
    match self.fragment().zip(did_url.fragment()) {
      Some((a, b)) => a == b,
      None => false,
    }
  }

  /// Extract the DID portion of the query if it exists.
  fn did_str(&self) -> Option<&str> {
    let query: &str = self.0.as_ref();
    if !query.starts_with(CoreDID::SCHEME) {
      return None;
    }

    // Find end of DID section.
    // did:example:123/path?service=agent&relativeRef=/credentials#degree
    let mut end_pos: usize = query.len();
    end_pos = end_pos.min(query.find('?').unwrap_or(end_pos));
    end_pos = end_pos.min(query.find('/').unwrap_or(end_pos));
    end_pos = end_pos.min(query.find('#').unwrap_or(end_pos));
    query.get(0..end_pos)
  }

  /// Extract the query fragment if it exists.
  fn fragment(&self) -> Option<&str> {
    let query: &str = self.0.as_ref();
    let fragment_maybe: Option<&str> = if query.starts_with(CoreDID::SCHEME) {
      // Extract the fragment from a full DID-Url-like string.
      query.rfind('#').and_then(|index| query.get(index + 1..))
    } else if let Some(fragment_delimiter_index) = query.rfind('#') {
      // Extract the fragment from a relative DID-Url.
      query.get(fragment_delimiter_index + 1..)
    } else {
      // Assume the entire string is a fragment.
      Some(query)
    };
    fragment_maybe.filter(|fragment| !fragment.is_empty())
  }
}

impl<'query> From<&'query str> for MethodQuery<'query> {
  fn from(other: &'query str) -> Self {
    Self(Cow::Borrowed(other))
  }
}

impl<'query> From<&'query String> for MethodQuery<'query> {
  fn from(other: &'query String) -> Self {
    Self(Cow::Borrowed(&**other))
  }
}

impl<'query, T> From<&'query DIDUrl<T>> for MethodQuery<'query>
where
  T: DID,
{
  fn from(other: &'query DIDUrl<T>) -> Self {
    Self(Cow::Owned(other.to_string()))
  }
}

impl<'query, T> From<DIDUrl<T>> for MethodQuery<'query>
where
  T: DID,
{
  fn from(other: DIDUrl<T>) -> Self {
    Self(Cow::Owned(other.to_string()))
  }
}

impl<'query> From<&'query RelativeDIDUrl> for MethodQuery<'query> {
  fn from(other: &'query RelativeDIDUrl) -> Self {
    // TODO: improve RelativeDIDUrl performance - internal string segments representation
    Self(Cow::Owned(other.to_string()))
  }
}

impl<'query> From<&'query Signature> for MethodQuery<'query> {
  fn from(other: &'query Signature) -> Self {
    Self(Cow::Borrowed(other.verification_method()))
  }
}

#[cfg(test)]
mod tests {
  use std::ops::Not;

  use super::*;

  #[test]
  fn test_did_str() {
    assert!(MethodQuery::from("").did_str().is_none());
    assert!(MethodQuery::from("fragment").did_str().is_none());
    assert!(MethodQuery::from("#fragment").did_str().is_none());
    assert!(MethodQuery::from("?query").did_str().is_none());
    assert!(MethodQuery::from("/path").did_str().is_none());
    assert!(MethodQuery::from("/path?query#fragment").did_str().is_none());
    assert!(MethodQuery::from("method:missingscheme123").did_str().is_none());
    assert!(MethodQuery::from("iota:example").did_str().is_none());
    assert_eq!(
      MethodQuery::from("did:iota:example").did_str(),
      Some("did:iota:example")
    );
    assert_eq!(
      MethodQuery::from("did:iota:example#fragment").did_str(),
      Some("did:iota:example")
    );
    assert_eq!(
      MethodQuery::from("did:iota:example?query").did_str(),
      Some("did:iota:example")
    );
    assert_eq!(
      MethodQuery::from("did:iota:example/path").did_str(),
      Some("did:iota:example")
    );
    assert_eq!(
      MethodQuery::from("did:iota:example/path?query#fragment").did_str(),
      Some("did:iota:example")
    );
    assert_eq!(
      MethodQuery::from("did:iota:example/path?query&relativeRef=/#fragment").did_str(),
      Some("did:iota:example")
    );
  }

  #[test]
  fn test_fragment() {
    assert!(MethodQuery::from("").fragment().is_none());
    assert_eq!(MethodQuery::from("fragment").fragment(), Some("fragment"));
    assert_eq!(MethodQuery::from("#fragment").fragment(), Some("fragment"));
    assert_eq!(MethodQuery::from("/path?query#fragment").fragment(), Some("fragment"));
    assert!(MethodQuery::from("did:iota:example").fragment().is_none());
    assert_eq!(
      MethodQuery::from("did:iota:example#fragment").fragment(),
      Some("fragment")
    );
    assert!(MethodQuery::from("did:iota:example?query").fragment().is_none());
    assert!(MethodQuery::from("did:iota:example/path").fragment().is_none());
    assert_eq!(
      MethodQuery::from("did:iota:example/path?query#fragment").fragment(),
      Some("fragment")
    );
    assert_eq!(
      MethodQuery::from("did:iota:example/path?query&relativeRef=/#fragment").fragment(),
      Some("fragment")
    );
  }

  #[test]
  fn test_matches() {
    let did_base: CoreDIDUrl = CoreDIDUrl::parse("did:iota:example").unwrap();
    let did_path: CoreDIDUrl = CoreDIDUrl::parse("did:iota:example/path").unwrap();
    let did_query: CoreDIDUrl = CoreDIDUrl::parse("did:iota:example?query").unwrap();
    let did_fragment: CoreDIDUrl = CoreDIDUrl::parse("did:iota:example#fragment").unwrap();
    let did_url: CoreDIDUrl = CoreDIDUrl::parse("did:iota:example/path?query#fragment").unwrap();
    let did_url_complex: CoreDIDUrl = CoreDIDUrl::parse("did:iota:example/path?query&relativeRef=/#fragment").unwrap();

    // Invalid: empty query should not match anything.
    let empty_query: MethodQuery = MethodQuery::from("");
    assert!(empty_query.matches(&did_base).not());
    assert!(empty_query.matches(&did_path).not());
    assert!(empty_query.matches(&did_query).not());
    assert!(empty_query.matches(&did_fragment).not());
    assert!(empty_query.matches(&did_url).not());
    assert!(empty_query.matches(&did_url_complex).not());

    // Invalid: query with DID and no fragment should not match anything.
    let empty_query: MethodQuery = MethodQuery::from("");
    assert!(empty_query.matches(&did_base).not());
    assert!(empty_query.matches(&did_path).not());
    assert!(empty_query.matches(&did_query).not());
    assert!(empty_query.matches(&did_fragment).not());
    assert!(empty_query.matches(&did_url).not());
    assert!(empty_query.matches(&did_url_complex).not());

    assert_eq!(MethodQuery::from("fragment").fragment(), Some("fragment"));
    assert_eq!(MethodQuery::from("#fragment").fragment(), Some("fragment"));
    assert_eq!(MethodQuery::from("/path?query#fragment").fragment(), Some("fragment"));
    assert!(MethodQuery::from("did:iota:example").fragment().is_none());
    assert_eq!(
      MethodQuery::from("did:iota:example#fragment").fragment(),
      Some("fragment")
    );
    assert!(MethodQuery::from("did:iota:example?query").fragment().is_none());
    assert!(MethodQuery::from("did:iota:example/path").fragment().is_none());
    assert_eq!(
      MethodQuery::from("did:iota:example/path?query#fragment").fragment(),
      Some("fragment")
    );
    assert_eq!(
      MethodQuery::from("did:iota:example/path?query&relativeRef=/#fragment").fragment(),
      Some("fragment")
    );
  }
}
