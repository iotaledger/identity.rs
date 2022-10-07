// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MethodType1(Cow<'static, str>);

impl MethodType1 {
  pub const fn ed_25519_verification_key_2018() -> Self {
    Self(Cow::Borrowed("Ed25519VerificationKey2018"))
  }

  pub const fn x_25519_verification_key_2018() -> Self {
    Self(Cow::Borrowed("X25519VerificationKey2018"))
  }

  pub fn as_str(&self) -> &str {
    self.0.as_ref()
  }
}

impl From<&str> for MethodType1 {
  fn from(string: &str) -> Self {
    Self(Cow::Owned(string.to_owned()))
  }
}
