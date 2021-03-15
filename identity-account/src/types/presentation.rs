// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::ops::Deref;
use identity_credential::presentation::VerifiablePresentation;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Presentation {
  base: VerifiablePresentation,
}

impl Presentation {
  pub const fn new(base: VerifiablePresentation) -> Self {
    Self { base }
  }
}

impl Deref for Presentation {
  type Target = VerifiablePresentation;

  fn deref(&self) -> &Self::Target {
    &self.base
  }
}
