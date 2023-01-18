// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt;
use core::fmt::Display;
use core::str::FromStr;

use identity_core::convert::FmtJson;

use crate::error::Error;
use crate::error::Result;
use crate::verification_method::MethodRelationship;

/// Verification method group used to refine the scope of a method query.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub enum MethodScope {
  VerificationMethod,
  VerificationRelationship(MethodRelationship),
}

impl MethodScope {
  pub fn as_str(&self) -> &'static str {
    match self {
      Self::VerificationMethod => "VerificationMethod",
      Self::VerificationRelationship(relationship) => relationship.into(),
    }
  }

  pub const fn authentication() -> Self {
    Self::VerificationRelationship(MethodRelationship::Authentication)
  }

  pub const fn capability_delegation() -> Self {
    Self::VerificationRelationship(MethodRelationship::CapabilityDelegation)
  }

  pub const fn capability_invocation() -> Self {
    Self::VerificationRelationship(MethodRelationship::CapabilityInvocation)
  }

  pub const fn assertion_method() -> Self {
    Self::VerificationRelationship(MethodRelationship::AssertionMethod)
  }

  pub const fn key_agreement() -> Self {
    Self::VerificationRelationship(MethodRelationship::KeyAgreement)
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
      "Authentication" => Ok(Self::VerificationRelationship(MethodRelationship::Authentication)),
      "AssertionMethod" => Ok(Self::VerificationRelationship(MethodRelationship::AssertionMethod)),
      "KeyAgreement" => Ok(Self::VerificationRelationship(MethodRelationship::KeyAgreement)),
      "CapabilityDelegation" => Ok(Self::VerificationRelationship(MethodRelationship::CapabilityDelegation)),
      "CapabilityInvocation" => Ok(Self::VerificationRelationship(MethodRelationship::CapabilityInvocation)),
      _ => Err(Error::UnknownMethodScope),
    }
  }
}

impl From<MethodRelationship> for MethodScope {
  fn from(relationship: MethodRelationship) -> Self {
    Self::VerificationRelationship(relationship)
  }
}

impl Display for MethodScope {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    self.fmt_json(f)
  }
}
