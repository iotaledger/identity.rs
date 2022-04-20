// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::actor::Error;
use crate::actor::Result as ActorResult;
use std::fmt::Display;
use std::str::FromStr;

use serde::Deserialize;
use serde::Serialize;

// TODO: Deserialization skips endpoint validation, but that is okay-ish at present.
/// A path-like identifier for a handler function. As an example, `identity/create`
/// could be an endpoint of the "identity" namespace and the concrete action "create".
///
/// An endpoint can also represent a `hook` function, indicated by a `/hook` postfix.
///
/// Endpoints are separated by slashes and only allow alphabetic ascii characters and `_`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Endpoint {
  name: String,
  handler: String,
  pub(crate) is_hook: bool,
}

impl Endpoint {
  /// Creates a new endpoint from a string. Returns an [`Error::InvalidEndpoint`]
  /// if disallowed characters are encountered.
  pub fn new(string: impl AsRef<str>) -> ActorResult<Self> {
    let mut is_hook = false;
    let mut split = string.as_ref().split('/');

    let name = split.next().unwrap().to_owned();
    let handler = split.next().ok_or(Error::InvalidEndpoint)?.to_owned();
    let hook = split.next();

    if name.is_empty()
      || handler.is_empty()
      || !name.chars().all(|c| c.is_ascii_alphabetic() || c == '_')
      || !handler.chars().all(|c| c.is_ascii_alphabetic() || c == '_')
    {
      return Err(Error::InvalidEndpoint);
    }

    if let Some(hook) = hook {
      if hook != "hook" {
        return Err(Error::InvalidEndpoint);
      }
      is_hook = true;
    }

    if split.next().is_some() {
      return Err(Error::InvalidEndpoint);
    }

    Ok(Self { name, handler, is_hook })
  }
}

impl FromStr for Endpoint {
  type Err = Error;

  fn from_str(string: &str) -> Result<Self, Self::Err> {
    Self::new(string)
  }
}

impl Display for Endpoint {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}/{}", self.name, self.handler)?;
    if self.is_hook {
      write!(f, "/hook")?;
    }
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::actor::Endpoint;
  use crate::actor::Error;

  #[test]
  fn invalid_endpoints() {
    assert!(matches!(Endpoint::new("").unwrap_err(), Error::InvalidEndpoint));
    assert!(matches!(Endpoint::new("/").unwrap_err(), Error::InvalidEndpoint));
    assert!(matches!(Endpoint::new("//").unwrap_err(), Error::InvalidEndpoint));
    assert!(matches!(Endpoint::new("a/").unwrap_err(), Error::InvalidEndpoint));
    assert!(matches!(Endpoint::new("/b").unwrap_err(), Error::InvalidEndpoint));
    assert!(matches!(Endpoint::new("a/b/c").unwrap_err(), Error::InvalidEndpoint));
    assert!(matches!(Endpoint::new("a/b/c/d").unwrap_err(), Error::InvalidEndpoint));
    assert!(matches!(Endpoint::new("1a/b").unwrap_err(), Error::InvalidEndpoint));
    assert!(matches!(Endpoint::new("a/b2").unwrap_err(), Error::InvalidEndpoint));
    assert!(matches!(Endpoint::new("a/b/钩").unwrap_err(), Error::InvalidEndpoint));
    assert!(matches!(
      Endpoint::new("longer/hyphenated-word").unwrap_err(),
      Error::InvalidEndpoint
    ));
  }

  #[test]
  fn valid_endpoints() {
    assert!("a/b".parse::<Endpoint>().is_ok());
    assert!(Endpoint::new("a/b").is_ok());
    assert!(Endpoint::new("longer/word").is_ok());
    assert!(Endpoint::new("longer_endpoint/underscored_word").is_ok());

    assert!(!Endpoint::new("a/b").unwrap().is_hook);
    assert!(Endpoint::new("a/b/hook").unwrap().is_hook);
  }
}
