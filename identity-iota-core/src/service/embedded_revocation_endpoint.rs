// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;
use core::fmt::Display;
use core::fmt::Formatter;
use core::ops::Deref;
use core::ops::DerefMut;

use identity_core::common::Url;
use serde;
use serde::Deserialize;
use serde::Serialize;

use super::error::Result;
use super::ServiceError;

/// A parsed data url.
#[derive(Clone, PartialEq, Deserialize, Serialize)]
#[repr(transparent)]
#[serde(transparent)]
pub struct EmbeddedRevocationEndpoint(pub(crate) String);

impl EmbeddedRevocationEndpoint {
  /// Parses an [`EmbeddedRevocationEndpoint`] from the given input string.
  pub fn parse(input: &str) -> Result<Self> {
    let input = input.to_string();
    if !input.starts_with("data:,") {
      return Err(ServiceError::InvalidServiceEndpoint(input));
    }
    let _ =
      data_url::DataUrl::process(&input.clone()).map_err(|_| ServiceError::InvalidServiceEndpoint(input.clone()))?;
    Ok(EmbeddedRevocationEndpoint(input))
  }

  /// Consumes the [`EmbeddedRevocationEndpoint`] and returns the value as a `String`.
  pub fn into_string(self) -> String {
    self.0
  }

  /// Returns the data from the [`EmbeddedRevocationEndpoint`].
  pub fn data(&self) -> &str {
    self.0.split_at(6).1
  }
}

impl TryFrom<Url> for EmbeddedRevocationEndpoint {
  type Error = ServiceError;

  fn try_from(other: Url) -> Result<Self> {
    Self::parse(&other.into_string())
  }
}

impl TryFrom<EmbeddedRevocationEndpoint> for Url {
  type Error = ServiceError;

  fn try_from(other: EmbeddedRevocationEndpoint) -> Result<Self> {
    let url_string: String = other.into_string();
    Self::parse(url_string.clone()).map_err(|_| ServiceError::InvalidServiceEndpoint(url_string))
  }
}

impl Debug for EmbeddedRevocationEndpoint {
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    f.write_fmt(format_args!("EmbeddedRevocationEndpoint({})", self.0.as_str()))
  }
}

impl Display for EmbeddedRevocationEndpoint {
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    f.write_str(self.0.as_str())
  }
}

impl Deref for EmbeddedRevocationEndpoint {
  type Target = String;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for EmbeddedRevocationEndpoint {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl<T> PartialEq<T> for EmbeddedRevocationEndpoint
where
  T: AsRef<str> + ?Sized,
{
  fn eq(&self, other: &T) -> bool {
    self.as_str() == other.as_ref()
  }
}
