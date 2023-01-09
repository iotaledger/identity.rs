// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt;
use core::fmt::Display;
use core::str::FromStr;

use identity_core::common::Timestamp;
use identity_core::convert::FmtJson;
use identity_core::Error;
use serde;
use serde::Deserialize;
use serde::Serialize;

// Implementation details
// This type is implemented according to https://www.w3.org/TR/vc-data-integrity/#generate-proof.
// - The type is meant to be user-facing, so UX is taken into account..
/// Holds attributes for a new [`Proof`](crate::crypto::Proof).
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ProofOptions {
  // TODO: Describe what requirements the fields must satisfy, e.g. purpose must match the verif. relationship.
  /// [`Proof::created`](crate::crypto::Proof::created)
  pub created: Option<Timestamp>,
  /// [`Proof::challenge`](crate::crypto::Proof::challenge)
  pub challenge: Option<String>,
  /// [`Proof::domain`](crate::crypto::Proof::domain)
  pub domain: Option<String>,
  /// [`Proof::purpose`](crate::crypto::Proof::purpose)
  pub purpose: Option<ProofPurpose>,
}

impl ProofOptions {
  /// Creates a new `ProofOptions` with all options unset.
  pub fn new() -> Self {
    Self {
      created: None,
      challenge: None,
      domain: None,
      purpose: None,
    }
  }

  /// Sets the [`Proof::created`](crate::crypto::Proof::created) field.
  #[must_use]
  pub fn created(mut self, created: Timestamp) -> Self {
    self.created = Some(created);
    self
  }

  /// Sets the [`Proof::challenge`](crate::crypto::Proof::challenge) field.
  #[must_use]
  pub fn challenge(mut self, challenge: String) -> Self {
    self.challenge = Some(challenge);
    self
  }

  /// Sets the [`Proof::domain`](crate::crypto::Proof::domain) field.
  #[must_use]
  pub fn domain(mut self, domain: String) -> Self {
    self.domain = Some(domain);
    self
  }

  /// Sets the [`Proof::purpose`](crate::crypto::Proof::purpose) field.
  #[must_use]
  pub fn purpose(mut self, purpose: ProofPurpose) -> Self {
    self.purpose = Some(purpose);
    self
  }
}

/// Associates a purpose with a [`Proof`](crate::crypto::Proof).
///
/// See the [W3C Security Vocabulary description](https://w3c-ccg.github.io/security-vocab/#proofPurpose).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ProofPurpose {
  /// Purpose is to assert a claim.
  /// See the [W3C Security Vocabulary description](https://www.w3.org/TR/did-core/#assertion).
  #[serde(rename = "assertionMethod")]
  AssertionMethod,
  /// Purpose is to authenticate the signer.
  /// See the [W3C Security Vocabulary description](https://www.w3.org/TR/did-core/#authentication).
  #[serde(rename = "authentication")]
  Authentication,
}

impl FromStr for ProofPurpose {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(match s {
      "assertionMethod" => ProofPurpose::AssertionMethod,
      "authentication" => ProofPurpose::Authentication,
      _ => return Err(Error::InvalidProofPurpose),
    })
  }
}

impl Display for ProofPurpose {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    self.fmt_json(f)
  }
}
