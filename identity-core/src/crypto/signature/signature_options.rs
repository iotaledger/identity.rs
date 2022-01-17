// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt;
use core::fmt::Display;
use core::str::FromStr;

use serde;
use serde::Deserialize;
use serde::Serialize;

use crate::common::Timestamp;
use crate::convert::FmtJson;
use crate::Error;

/// Holds attributes for a new [`Signature`](crate::crypto::Signature).
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SignatureOptions {
  /// [`Signature::created`](crate::crypto::Signature::created)
  pub created: Option<Timestamp>,
  /// [`Signature::expires`](crate::crypto::Signature::expires)
  pub expires: Option<Timestamp>,
  /// [`Signature::challenge`](crate::crypto::Signature::challenge)
  pub challenge: Option<String>,
  /// [`Signature::domain`](crate::crypto::Signature::domain)
  pub domain: Option<String>,
  /// [`Signature::purpose`](crate::crypto::Signature::purpose)
  pub purpose: Option<ProofPurpose>,
}

impl SignatureOptions {
  /// Creates a new `SignatureOptions` with all options unset.
  pub fn new() -> Self {
    Self {
      created: None,
      expires: None,
      challenge: None,
      domain: None,
      purpose: None,
    }
  }

  /// Sets the [`Signature::created`](crate::crypto::Signature::created) field.
  #[must_use]
  pub fn created(mut self, created: Timestamp) -> Self {
    self.created = Some(created);
    self
  }

  /// Sets the [`Signature::expires`](crate::crypto::Signature::expires) field.
  /// The signature will fail validation after the specified datetime.
  #[must_use]
  pub fn expires(mut self, expires: Timestamp) -> Self {
    self.expires = Some(expires);
    self
  }

  /// Sets the [`Signature::challenge`](crate::crypto::Signature::challenge) field.
  #[must_use]
  pub fn challenge(mut self, challenge: String) -> Self {
    self.challenge = Some(challenge);
    self
  }

  /// Sets the [`Signature::domain`](crate::crypto::Signature::domain) field.
  #[must_use]
  pub fn domain(mut self, domain: String) -> Self {
    self.domain = Some(domain);
    self
  }

  /// Sets the [`Signature::purpose`](crate::crypto::Signature::purpose) field.
  #[must_use]
  pub fn purpose(mut self, purpose: ProofPurpose) -> Self {
    self.purpose = Some(purpose);
    self
  }
}

/// Associates a purpose with a [`Signature`](crate::crypto::Signature).
///
/// See https://w3c-ccg.github.io/security-vocab/#proofPurpose
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ProofPurpose {
  /// Purpose is to assert a claim.
  /// See https://www.w3.org/TR/did-core/#assertion
  #[serde(rename = "assertionMethod")]
  AssertionMethod,
  /// Purpose is to authenticate the signer.
  /// See https://www.w3.org/TR/did-core/#authentication
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
