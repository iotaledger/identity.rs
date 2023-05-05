// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/// A wrapper around a JSON Web Signature (JWS).
pub struct Jws(String);

impl Jws {
  /// Creates a new `Jws`.
  pub fn new(jwt_string: String) -> Self {
    Self(jwt_string)
  }

  /// Returns a clone of the JWS string.
  pub fn to_string(&self) -> String {
    self.0.clone()
  }

  /// Converts `Jws` into a string.
  pub fn into_string(self) -> String {
    self.0
  }

  /// Returns a reference of the JWS string.
  pub fn as_string(&self) -> &String {
    &self.0
  }
}
