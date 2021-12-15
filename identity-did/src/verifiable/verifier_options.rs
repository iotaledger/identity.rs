// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::verification::MethodScope;
use crate::verification::MethodType;
use identity_core::crypto::ProofPurpose;

/// Holds additional options for verifying a signature with [`DocumentVerifier`](crate::verifiable::DocumentVerifier).
#[derive(Debug, Clone, Default)]
pub struct VerifierOptions {
  /// [`DocumentVerifier::method_scope'].
  pub method_scope: Option<MethodScope>,
  /// [`DocumentVerifier::method_type'].
  pub method_type: Option<Vec<MethodType>>,
  /// [`DocumentVerifier::challenge'].
  pub challenge: Option<String>,
  /// [`DocumentVerifier::domain'].
  pub domain: Option<String>,
  /// [`DocumentVerifier::purpose'].
  pub purpose: Option<ProofPurpose>,
  /// [`DocumentVerifier::allow_expired'].
  pub allow_expired: Option<bool>,
}

impl VerifierOptions {
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
  pub fn method_type(mut self, method_type: Vec<MethodType>) -> Self {
    self.method_type = Some(method_type);
    self
  }

  /// See [`DocumentVerifier::challenge'].
  pub fn challenge(mut self, challenge: String) -> Self {
    self.challenge = Some(challenge);
    self
  }

  /// See [`DocumentVerifier::domain'].
  pub fn domain(mut self, domain: String) -> Self {
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
