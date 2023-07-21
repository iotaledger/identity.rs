// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/// Verification relationships.
///
/// See also: <https://www.w3.org/TR/did-core/#verification-relationships>.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize, strum::IntoStaticStr)]
pub enum MethodRelationship {
  /// The authentication verification relationship.
  Authentication,
  /// The assertion method verification relationship.
  AssertionMethod,
  /// The key agreement verification relationship.
  KeyAgreement,
  /// The capability delegation verification relationship.
  CapabilityDelegation,
  /// The capability invocation verification relationship.
  CapabilityInvocation,
}
