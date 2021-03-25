// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::jwk::EdCurve;
use crate::jws::JwsHeader;
use crate::utils::Secret;

#[derive(Clone, Copy)]
pub struct Recipient<'a> {
  /// The curve used for EdDSA signatures.
  pub eddsa_curve: EdCurve,
  /// The private key used for signature creation.
  pub secret: Secret<'a>,
  /// The integrity-protected JOSE header.
  pub protected: Option<&'a JwsHeader>,
  /// The non integrity-protected JOSE header.
  pub unprotected: Option<&'a JwsHeader>,
}

impl<'a> Recipient<'a> {
  pub fn new(secret: impl Into<Secret<'a>>) -> Self {
    Self {
      eddsa_curve: EdCurve::Ed25519,
      secret: secret.into(),
      protected: None,
      unprotected: None,
    }
  }

  pub fn eddsa_curve(mut self, value: EdCurve) -> Self {
    self.eddsa_curve = value;
    self
  }

  pub fn protected(mut self, value: &'a JwsHeader) -> Self {
    self.protected = Some(value);
    self
  }

  pub fn unprotected(mut self, value: &'a JwsHeader) -> Self {
    self.unprotected = Some(value);
    self
  }
}

impl<'a, T> From<T> for Recipient<'a>
where
  T: Into<Secret<'a>>,
{
  fn from(other: T) -> Self {
    Self::new(other)
  }
}

impl<'a, T> From<(T, &'a JwsHeader)> for Recipient<'a>
where
  T: Into<Secret<'a>>,
{
  fn from(other: (T, &'a JwsHeader)) -> Self {
    Self::new(other.0).protected(other.1)
  }
}

impl<'a, T> From<(T, &'a JwsHeader, &'a JwsHeader)> for Recipient<'a>
where
  T: Into<Secret<'a>>,
{
  fn from(other: (T, &'a JwsHeader, &'a JwsHeader)) -> Self {
    Self::new(other.0).protected(other.1).unprotected(other.2)
  }
}
