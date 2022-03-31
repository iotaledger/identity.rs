// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/// Verification relationships.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize, strum::IntoStaticStr)]
pub enum MethodRelationship {
  Authentication,
  AssertionMethod,
  KeyAgreement,
  CapabilityDelegation,
  CapabilityInvocation,
}
