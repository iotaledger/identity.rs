// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::verification::MethodScope;
use crate::verification::MethodType;

/// Holds additional options for verifying a signature with [`DocumentVerifier`](crate::verifiable::DocumentVerifier).
#[derive(Debug, Clone, Default)]
pub struct VerifierOptions<'base> {
  /// Verify the signing verification method relationship matches this.
  pub method_scope: Option<MethodScope>,
  /// Verify the signing verification method type matches one specified.
  pub method_type: Option<&'base [MethodType]>,
  /// Verify the [`Signature::challenge`] field matches this.
  pub challenge: Option<&'base str>,
  /// Verify the [`Signature::domain`] field matches this.
  pub domain: Option<&'base str>,
  /// Verify the [`Signature::purpose`] field matches this.
  pub purpose: Option<&'base str>,
  /// Determines whether to error if the current time exceeds the [`Signature::expires`] field.
  ///
  /// Default: false (reject expired signatures).
  pub allow_expired: Option<bool>,
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

  /// Verify the signing verification method relationship matches this.
  pub fn method_scope(mut self, method_scope: MethodScope) -> Self {
    self.method_scope = Some(method_scope);
    self
  }

  /// Verify the signing verification method type matches one specified.
  pub fn method_type(mut self, method_type: &'base [MethodType]) -> Self {
    self.method_type = Some(method_type);
    self
  }

  /// Verify the [`Signature::challenge`] field matches this.
  pub fn challenge(mut self, challenge: &'base str) -> Self {
    self.challenge = Some(challenge);
    self
  }

  /// Verify the [`Signature::domain`] field matches this.
  pub fn domain(mut self, domain: &'base str) -> Self {
    self.domain = Some(domain);
    self
  }

  /// Verify the [`Signature::purpose`] field matches this.
  pub fn purpose(mut self, purpose: &'base str) -> Self {
    self.purpose = Some(purpose);
    self
  }

  /// Determines whether to error if the current time exceeds the [`Signature::expires`] field.
  ///
  /// Default: false (reject expired signatures).
  pub fn allow_expired(mut self, allow_expired: bool) -> Self {
    self.allow_expired = Some(allow_expired);
    self
  }
}
