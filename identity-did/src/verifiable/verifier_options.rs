// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::verification::MethodScope;
use crate::verification::MethodType;
use identity_core::crypto::ProofPurpose;

/// Holds additional options for verifying a signature with [`DocumentVerifier`](crate::verifiable::DocumentVerifier).
#[derive(Debug, Clone, Default)]
pub struct VerifierOptions<'base> {
  pub(crate) method_scope: Option<MethodScope>,
  pub(crate) method_type: Option<&'base [MethodType]>,
  pub(crate) challenge: Option<&'base str>,
  pub(crate) domain: Option<&'base str>,
  pub(crate) purpose: Option<ProofPurpose>,
  pub(crate) allow_expired: Option<bool>,
}

impl<'base> VerifierOptions<'base> {
  /// Creates a new `VerifierOptions` with all options unset.
  pub fn new() -> Self {
    Self {
      method_scope: None,
      method_type: None,
      challenge: None,
      domain: None,
      purpose: None,
      allow_expired: None,
    }
  }

  /// See [`DocumentVerifier::method_scope'].
  pub fn method_scope(mut self, method_scope: MethodScope) -> Self {
    self.method_scope = Some(method_scope);
    self
  }

  /// See [`DocumentVerifier::method_type'].
  pub fn method_type(mut self, method_type: &'base [MethodType]) -> Self {
    self.method_type = Some(method_type);
    self
  }

  /// See [`DocumentVerifier::challenge'].
  pub fn challenge(mut self, challenge: &'base str) -> Self {
    self.challenge = Some(challenge);
    self
  }

  /// See [`DocumentVerifier::domain'].
  pub fn domain(mut self, domain: &'base str) -> Self {
    self.domain = Some(domain);
    self
  }

  /// See [`DocumentVerifier::purpose'].
  pub fn purpose(mut self, purpose: ProofPurpose) -> Self {
    self.purpose = Some(purpose);
    self
  }

  /// See [`DocumentVerifier::allow_expired'].
  pub fn allow_expired(mut self, allow_expired: bool) -> Self {
    self.allow_expired = Some(allow_expired);
    self
  }
}
