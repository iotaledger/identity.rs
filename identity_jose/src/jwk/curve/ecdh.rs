// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result;

use crate::jwk::EcCurve;
use crate::jwk::EcxCurve;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum EcdhCurve {
  Ec(EcCurve),
  Ecx(EcxCurve),
}

impl EcdhCurve {
  pub const fn name(self) -> &'static str {
    match self {
      Self::Ec(inner) => inner.name(),
      Self::Ecx(inner) => inner.name(),
    }
  }
}

impl Display for EcdhCurve {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.write_str(self.name())
  }
}

impl From<EcCurve> for EcdhCurve {
  fn from(other: EcCurve) -> Self {
    Self::Ec(other)
  }
}

impl From<EcxCurve> for EcdhCurve {
  fn from(other: EcxCurve) -> Self {
    Self::Ecx(other)
  }
}
