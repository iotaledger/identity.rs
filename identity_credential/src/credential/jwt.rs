// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::Deserialize;
use serde::Serialize;

/// A wrapper around a JSON Web Token (JWK).
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct Jwt(String);

impl Jwt {
  /// Creates a new `Jwt` from the given string.
  pub fn new(jwt_string: String) -> Self {
    Self(jwt_string)
  }

  /// Returns a reference of the JWT string.
  pub fn as_str(&self) -> &str {
    &self.0
  }
}

impl From<String> for Jwt {
  fn from(jwt: String) -> Self {
    Self::new(jwt)
  }
}

impl From<Jwt> for String {
  fn from(jwt: Jwt) -> Self {
    jwt.0
  }
}
