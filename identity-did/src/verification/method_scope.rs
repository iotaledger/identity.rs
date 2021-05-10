// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::str::FromStr;

use crate::error::Error;
use crate::error::Result;

/// Verification method group used to refine the scope of a method query.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub enum MethodScope {
  VerificationMethod,
  Authentication,
  AssertionMethod,
  KeyAgreement,
  CapabilityDelegation,
  CapabilityInvocation,
}

impl MethodScope {
  pub const fn as_str(&self) -> &'static str {
    match self {
      Self::VerificationMethod => "VerificationMethod",
      Self::Authentication => "Authentication",
      Self::AssertionMethod => "AssertionMethod",
      Self::KeyAgreement => "KeyAgreement",
      Self::CapabilityDelegation => "CapabilityDelegation",
      Self::CapabilityInvocation => "CapabilityInvocation",
    }
  }
}

impl Default for MethodScope {
  fn default() -> Self {
    Self::VerificationMethod
  }
}

impl FromStr for MethodScope {
  type Err = Error;

  fn from_str(string: &str) -> Result<Self, Self::Err> {
    match string {
      "VerificationMethod" => Ok(Self::VerificationMethod),
      "Authentication" => Ok(Self::Authentication),
      "AssertionMethod" => Ok(Self::AssertionMethod),
      "KeyAgreement" => Ok(Self::KeyAgreement),
      "CapabilityDelegation" => Ok(Self::CapabilityDelegation),
      "CapabilityInvocation" => Ok(Self::CapabilityInvocation),
      _ => Err(Error::UnknownMethodScope),
    }
  }
}
