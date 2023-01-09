// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::KeyComparable;
use identity_core::common::OrderedSet;
use identity_data_integrity::proof::ProofPurpose;

use super::MethodRef;
use super::VerificationMethod;
use crate::did::DID;
use crate::document::CoreDocument;

/// Verification relationships.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize, strum::IntoStaticStr)]
pub enum MethodRelationship {
  Authentication,
  AssertionMethod,
  KeyAgreement,
  CapabilityDelegation,
  CapabilityInvocation,
}

impl MethodRelationship {
  /// Extract embedded verification methods
  pub fn extract_methods<'a, D, T, U, V>(&self, doc: &'a CoreDocument<D, T, U, V>) -> &'a OrderedSet<MethodRef<D, U>>
  where
    D: DID + KeyComparable,
  {
    match self {
      Self::AssertionMethod => doc.assertion_method(),
      Self::Authentication => doc.authentication(),
      Self::CapabilityDelegation => doc.capability_delegation(),
      Self::CapabilityInvocation => doc.capability_invocation(),
      Self::KeyAgreement => doc.key_agreement(),
    }
  }
}

impl From<MethodRelationship> for ProofPurpose {
  fn from(value: MethodRelationship) -> Self {
    match value {
      MethodRelationship::AssertionMethod => ProofPurpose::AssertionMethod,
      MethodRelationship::Authentication => ProofPurpose::Authentication,
      MethodRelationship::CapabilityDelegation => ProofPurpose::CapabilityDelegation,
      MethodRelationship::CapabilityInvocation => ProofPurpose::CapabilityInvocation,
      MethodRelationship::KeyAgreement => ProofPurpose::KeyAgreement,
    }
  }
}

#[derive(Debug, thiserror::Error)]
#[error("could not convert type to MethodRelationship")]
pub struct MethodRelationshipConversionError;

impl TryFrom<&ProofPurpose> for MethodRelationship {
  type Error = MethodRelationshipConversionError;
  fn try_from(value: &ProofPurpose) -> Result<Self, Self::Error> {
    [
      (&ProofPurpose::AssertionMethod, MethodRelationship::AssertionMethod),
      (&ProofPurpose::Authentication, MethodRelationship::Authentication),
      (
        &ProofPurpose::CapabilityInvocation,
        MethodRelationship::CapabilityInvocation,
      ),
      (
        &ProofPurpose::CapabilityDelegation,
        MethodRelationship::CapabilityDelegation,
      ),
      (&ProofPurpose::KeyAgreement, MethodRelationship::KeyAgreement),
    ]
    .into_iter()
    .find_map(|(proof_purpose, method_relationship)| (proof_purpose == value).then_some(method_relationship))
    .ok_or(MethodRelationshipConversionError)
  }
}
