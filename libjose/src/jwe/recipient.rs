// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::jwe::JweHeader;
use crate::jwk::EcdhCurve;
use crate::jwk::EcxCurve;
use crate::utils::Secret;

#[derive(Clone)]
pub struct Recipient<'a> {
  /// The curve used for Ecdh key agreements.
  pub ecdh_curve: EcdhCurve,
  /// The public key used for key agreements and content encryption.
  pub public: Secret<'a>,
  /// The non integrity-protected JOSE header.
  pub header: Option<JweHeader>,
}

impl<'a> Recipient<'a> {
  pub fn new(public: impl Into<Secret<'a>>) -> Self {
    Self {
      ecdh_curve: EcdhCurve::Ecx(EcxCurve::X25519),
      public: public.into(),
      header: None,
    }
  }

  pub fn ecdh_curve(mut self, value: impl Into<EcdhCurve>) -> Self {
    self.ecdh_curve = value.into();
    self
  }

  pub fn header(mut self, value: &JweHeader) -> Self {
    // TODO: Fix clone if kept.
    self.header = Some(value.clone());
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

impl<'a, T> From<(T, &'a JweHeader)> for Recipient<'a>
where
  T: Into<Secret<'a>>,
{
  fn from(other: (T, &'a JweHeader)) -> Self {
    Self::new(other.0).header(other.1)
  }
}
