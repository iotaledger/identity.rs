// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::errors::{Error, Result};
use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Endpoint {
  name: String,
  handler: String,
  hook: Option<String>,
}

impl Endpoint {
  pub fn new(string: impl AsRef<str>) -> Result<Self> {
    let mut split = string.as_ref().split('/');

    let name = split.next().unwrap().to_owned();
    let handler = split.next().ok_or(Error::InvalidEndpoint)?.to_owned();
    let hook = split.next().map(ToOwned::to_owned);

    if name.is_empty() || handler.is_empty() || hook.is_some() && hook.as_ref().unwrap().is_empty() {
      return Err(Error::InvalidEndpoint);
    }

    if hook.is_some() && split.next().is_some() {
      return Err(Error::InvalidEndpoint);
    }

    Ok(Self { name, handler, hook })
  }

  pub fn to_catch_all(self) -> Self {
    Self {
      name: self.name,
      handler: "*".to_owned(),
      hook: None,
    }
  }
}

impl Display for Endpoint {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}/{}", self.name, self.handler)?;
    if let Some(hook) = &self.hook {
      write!(f, "/{}", hook)?;
    }
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::{endpoint::Endpoint, errors::Error};

  #[test]
  fn invalid_endpoints() {
    assert!(matches!(Endpoint::new("").unwrap_err(), Error::InvalidEndpoint));
    assert!(matches!(Endpoint::new("/").unwrap_err(), Error::InvalidEndpoint));
    assert!(matches!(Endpoint::new("//").unwrap_err(), Error::InvalidEndpoint));
    assert!(matches!(Endpoint::new("a/").unwrap_err(), Error::InvalidEndpoint));
    assert!(matches!(Endpoint::new("/b").unwrap_err(), Error::InvalidEndpoint));
    assert!(matches!(Endpoint::new("a/b/c/d").unwrap_err(), Error::InvalidEndpoint));
  }

  #[test]
  fn valid_endpoints() {
    assert!(Endpoint::new("a/b").is_ok());
    assert!(Endpoint::new("a/b/c").is_ok());
    assert!(Endpoint::new("a/*").is_ok());
  }
}
