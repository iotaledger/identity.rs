// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/// Verification relationships.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub enum MethodRelationship {
  Authentication,
  AssertionMethod,
  KeyAgreement,
  CapabilityDelegation,
  CapabilityInvocation,
}

impl MethodRelationship {
  pub const fn as_str(&self) -> &'static str {
    match self {
      Self::Authentication => "Authentication",
      Self::AssertionMethod => "AssertionMethod",
      Self::KeyAgreement => "KeyAgreement",
      Self::CapabilityDelegation => "CapabilityDelegation",
      Self::CapabilityInvocation => "CapabilityInvocation",
    }
  }
}
