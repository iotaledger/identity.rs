// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::borrow::Cow;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct MethodType1(Cow<'static, str>);

impl MethodType1 {
  pub const fn ed25519_verification_key_2018() -> Self {
    Self(Cow::Borrowed("Ed25519VerificationKey2018"))
  }

  pub const fn x25519_verification_key_2018() -> Self {
    Self(Cow::Borrowed("X25519VerificationKey2018"))
  }

  pub fn as_str(&self) -> &str {
    self.0.as_ref()
  }
}

impl<S: Into<String>> From<S> for MethodType1 {
  fn from(string: S) -> Self {
    Self(Cow::Owned(string.into()))
  }
}

impl Display for MethodType1 {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    Display::fmt(&self.0, f)
  }
}
