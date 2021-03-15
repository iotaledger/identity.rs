// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::ops::Deref;
use identity_credential::credential::VerifiableCredential;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Credential {
  base: VerifiableCredential,
}

impl Credential {
  pub const fn new(base: VerifiableCredential) -> Self {
    Self { base }
  }
}

impl Deref for Credential {
  type Target = VerifiableCredential;

  fn deref(&self) -> &Self::Target {
    &self.base
  }
}
