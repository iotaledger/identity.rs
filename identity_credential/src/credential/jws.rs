// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/// A wrapper around a JSON Web Signature (JWS).
pub struct Jws(String);

impl Jws {
  /// Creates a new `Jws`.
  pub fn new(jwt_string: String) -> Self {
    Self(jwt_string)
  }

  /// Returns a reference of the JWS string.
  pub fn as_str(&self) -> &str {
    &self.0
  }
}

impl From<String> for Jws {
  fn from(jws: String) -> Self {
    Self::new(jws)
  }
}
impl From<Jws> for String {
  fn from(jws: Jws) -> Self {
    jws.0
  }
}
