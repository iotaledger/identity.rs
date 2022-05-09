// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::actor::Error;
use crate::actor::Result as ActorResult;
use std::borrow::Cow;
use std::fmt::Display;
use std::str::Split;

use serde::Deserialize;
use serde::Serialize;

static HOOK: &str = "hook";

/// A path-like identifier for a handler function. As an example, `identity/create`
/// could be an endpoint of the "identity" namespace and the concrete action "create".
///
/// An endpoint can also represent a `hook` function, indicated by a `/hook` postfix.
///
/// Endpoints are separated by slashes and only allow alphabetic ascii characters and `_`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct Endpoint {
  name: Cow<'static, str>,
}

impl Endpoint {
  /// Returns whether this endpoint is a hook.
  pub fn is_hook(&self) -> bool {
    let split: Split<char> = self.name.as_ref().split('/');
    split.last().expect("split did not have at least one element") == HOOK
  }

  /// Converts this endpoint into a hook.
  pub fn into_hook(mut self) -> Self {
    if !self.is_hook() {
      let mut name: String = self.name.into_owned();
      name.reserve("/".len() + HOOK.len());
      self.name = Cow::Owned(name + "/" + HOOK);
    }
    self
  }

  /// Validates the endpoint.
  fn validate(string: &str) -> ActorResult<()> {
    let mut split: Split<char> = string.split('/');

    let name: &str = split.next().expect("split did not have at least one element");
    let handler: &str = split.next().ok_or(Error::InvalidEndpoint)?;
    let hook: Option<&str> = split.next();

    let is_valid_segment =
      |segment: &str| !segment.is_empty() && segment.chars().all(|c| c.is_ascii_alphabetic() || c == '_');

    if !is_valid_segment(name) || !is_valid_segment(handler) {
      return Err(Error::InvalidEndpoint);
    }

    match hook {
      Some("hook") | None => (),
      Some(_) => return Err(Error::InvalidEndpoint),
    }

    if split.next().is_some() {
      return Err(Error::InvalidEndpoint);
    }

    Ok(())
  }
}

impl TryFrom<&'static str> for Endpoint {
  type Error = Error;

  /// Creates a new endpoint from a string. Returns an [`Error::InvalidEndpoint`]
  /// if disallowed characters are encountered.
  fn try_from(endpoint: &'static str) -> Result<Self, Self::Error> {
    Self::validate(endpoint)?;

    Ok(Self {
      name: Cow::Borrowed(endpoint),
    })
  }
}

impl TryFrom<String> for Endpoint {
  type Error = Error;

  /// Creates a new endpoint from a string. Returns an [`Error::InvalidEndpoint`]
  /// if disallowed characters are encountered.
  fn try_from(endpoint: String) -> Result<Self, Self::Error> {
    Self::validate(&endpoint)?;

    Ok(Self {
      name: Cow::Owned(endpoint),
    })
  }
}

impl Display for Endpoint {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.name.as_ref())
  }
}

impl AsRef<str> for Endpoint {
  fn as_ref(&self) -> &str {
    self.name.as_ref()
  }
}

impl From<Endpoint> for String {
  fn from(endpoint: Endpoint) -> Self {
    endpoint.name.into_owned()
  }
}

#[cfg(test)]
mod tests {
  use identity_core::convert::FromJson;

  use crate::actor::Endpoint;
  use crate::actor::Error;

  #[test]
  fn test_endpoint_invalid() {
    assert!(matches!(Endpoint::try_from("").unwrap_err(), Error::InvalidEndpoint));
    assert!(matches!(Endpoint::try_from("/").unwrap_err(), Error::InvalidEndpoint));
    assert!(matches!(Endpoint::try_from("//").unwrap_err(), Error::InvalidEndpoint));
    assert!(matches!(Endpoint::try_from("a/").unwrap_err(), Error::InvalidEndpoint));
    assert!(matches!(Endpoint::try_from("/b").unwrap_err(), Error::InvalidEndpoint));
    assert!(matches!(
      Endpoint::try_from("a/b/").unwrap_err(),
      Error::InvalidEndpoint
    ));
    assert!(matches!(
      Endpoint::try_from("a/b/c").unwrap_err(),
      Error::InvalidEndpoint
    ));
    assert!(matches!(
      Endpoint::try_from("a/b/c/d").unwrap_err(),
      Error::InvalidEndpoint
    ));
    assert!(matches!(
      Endpoint::try_from("1a/b").unwrap_err(),
      Error::InvalidEndpoint
    ));
    assert!(matches!(
      Endpoint::try_from("a/b2").unwrap_err(),
      Error::InvalidEndpoint
    ));
    assert!(matches!(
      Endpoint::try_from("a/b/钩").unwrap_err(),
      Error::InvalidEndpoint
    ));
    assert!(matches!(
      Endpoint::try_from("longer/hyphenated-word").unwrap_err(),
      Error::InvalidEndpoint
    ));
  }

  #[test]
  fn test_endpoint_valid() {
    assert!(Endpoint::try_from("a/b").is_ok());
    assert!(Endpoint::try_from("a/b").is_ok());
    assert!(Endpoint::try_from("longer/word").is_ok());
    assert!(Endpoint::try_from("longer_endpoint/underscored_word").is_ok());
  }

  #[test]
  fn test_endpoint_hook() {
    assert!(!Endpoint::try_from("a/b").unwrap().is_hook());
    assert!(Endpoint::try_from("a/b/hook").unwrap().is_hook());

    let endpoint: Endpoint = Endpoint::try_from("a/b").unwrap().into_hook();
    assert_eq!(endpoint.as_ref(), "a/b/hook");
    assert_eq!(endpoint.into_hook().as_ref(), "a/b/hook");
  }

  #[test]
  fn test_endpoint_deserialization_validates() {
    let err = Endpoint::from_json(r#"{ "name": "a/b/invalid" }"#).unwrap_err();
    assert!(matches!(
      err,
      identity_core::Error::DecodeJSON(serde_json::Error { .. })
    ));
  }
}
