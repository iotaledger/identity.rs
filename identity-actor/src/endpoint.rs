// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::errors::{Error, Result};
use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Endpoint {
  name: String,
  handler: String,
  hook: bool,
}

impl Endpoint {
  pub fn new(string: impl AsRef<str>) -> Result<Self> {
    let mut split = string.as_ref().split('/');

    let name = split.next().unwrap().to_owned();
    let handler = split.next().ok_or(Error::InvalidEndpoint)?.to_owned();
    let hook = split.next().map(ToOwned::to_owned);

    if name.is_empty() || handler.is_empty() {
      return Err(Error::InvalidEndpoint);
    }

    if let Some(hook) = hook {
      if hook != "hook" {
        return Err(Error::InvalidEndpoint);
      }
    }

    if split.next().is_some() {
      return Err(Error::InvalidEndpoint);
    }

    Ok(Self {
      name,
      handler,
      hook: false,
    })
  }

  pub fn new_hook(string: impl AsRef<str>) -> Result<Self> {
    let mut endpoint = Self::new(string)?;
    endpoint.hook = true;
    Ok(endpoint)
  }

  pub fn set_hook(&mut self, hook: bool) {
    self.hook = hook;
  }

  pub fn hook(&self) -> bool {
    self.hook
  }

  pub fn to_catch_all(self) -> Self {
    Self {
      handler: "*".to_owned(),
      ..self
    }
  }
}

impl Display for Endpoint {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}/{}", self.name, self.handler)?;
    if self.hook {
      write!(f, "/hook")?;
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
    assert!(matches!(Endpoint::new("a/b/c").unwrap_err(), Error::InvalidEndpoint));
    assert!(matches!(Endpoint::new("a/b/c/d").unwrap_err(), Error::InvalidEndpoint));
  }

  #[test]
  fn valid_endpoints() {
    assert!(Endpoint::new("a/b").is_ok());
    assert!(Endpoint::new("a/b/hook").is_ok());
    assert!(Endpoint::new("a/*").is_ok());
  }
}
