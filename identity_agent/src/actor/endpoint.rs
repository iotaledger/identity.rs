// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::borrow::Cow;
use std::fmt::Display;
use std::str::Split;

use serde::Deserialize;
use serde::Serialize;

use crate::actor::Error;
use crate::actor::Result as AgentResult;

/// A path-like identifier for an actor request. As an example, `identity/resolve`
/// could be the endpoint of the "resolve" request within the "identity" namespace.
///
/// The namespace and request are separated by a slash and only allow alphabetic ascii characters and `_`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct Endpoint {
  name: Cow<'static, str>,
}

impl Endpoint {
  /// Checks whether the given `string` is a valid [`Endpoint`].
  fn validate(string: &str) -> AgentResult<()> {
    let mut split: Split<'_, char> = string.split('/');

    // Once the Never (`!`) type lands in stable Rust, we can try to map the `None` variant to ! instead.
    let namespace: &str = split.next().expect("split always returns at least one element");
    let request: &str = split.next().ok_or(Error::InvalidEndpoint)?;

    let is_valid_segment =
      |segment: &str| !segment.is_empty() && segment.chars().all(|c| c.is_ascii_alphabetic() || c == '_');

    if !is_valid_segment(namespace) || !is_valid_segment(request) {
      return Err(Error::InvalidEndpoint);
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
    for invalid_endpoint in [
      "",
      "/",
      "//",
      "a/",
      "/b",
      "a/b/",
      "a/b/c",
      "a/b/c/d",
      "1a/b",
      "a/b2",
      "námespace/endpoint",
      "namespace/hyphenated-word",
    ] {
      assert!(matches!(
        Endpoint::try_from(invalid_endpoint).unwrap_err(),
        Error::InvalidEndpoint
      ),);
    }
  }

  #[test]
  fn test_endpoint_valid() {
    for valid_endpoint in ["a/b", "longer/word", "longer_endpoint/underscored_word"] {
      assert!(
        Endpoint::try_from(valid_endpoint).is_ok(),
        "expected `{valid_endpoint}` to be a valid endpoint"
      );
    }
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
