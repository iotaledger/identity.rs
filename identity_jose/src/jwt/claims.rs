// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::OneOrMany;

/// JSON Web Token Claims
///
/// [More Info](https://tools.ietf.org/html/rfc7519#section-4)
#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct JwtClaims<T = ()> {
  /// Identifies the principal that issued the JWT
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7519#section-4.1.1)
  #[serde(skip_serializing_if = "Option::is_none")]
  iss: Option<String>, // Issuer
  /// Identifies the principal that is the subject of the JWT.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7519#section-4.1.2)
  #[serde(skip_serializing_if = "Option::is_none")]
  sub: Option<String>, // Subject
  /// Identifies the recipients that the JWT is intended for.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7519#section-4.1.3)
  #[serde(skip_serializing_if = "Option::is_none")]
  aud: Option<OneOrMany<String>>, // Audience
  /// Identifies the expiration time on or after which the JWT MUST NOT be
  /// accepted for processing.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7519#section-4.1.4)
  #[serde(skip_serializing_if = "Option::is_none")]
  exp: Option<i64>, // Expiration Time
  /// Identifies the time before which the JWT MUST NOT be accepted for
  /// processing.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7519#section-4.1.5)
  #[serde(skip_serializing_if = "Option::is_none")]
  nbf: Option<i64>, // Not Before
  /// Identifies the time at which the JWT was issued.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7519#section-4.1.6)
  #[serde(skip_serializing_if = "Option::is_none")]
  iat: Option<i64>, // Issued At
  /// Provides a unique identifier for the JWT.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7519#section-4.1.7)
  #[serde(skip_serializing_if = "Option::is_none")]
  jti: Option<String>, // JWT ID
  /// Public/Private Claim Names
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7519#section-4.2)
  #[serde(flatten, skip_serializing_if = "Option::is_none")]
  custom: Option<T>,
}

impl<T> JwtClaims<T> {
  /// Create a new `JwtClaims` set.
  pub const fn new() -> Self {
    Self {
      iss: None,
      sub: None,
      aud: None,
      exp: None,
      nbf: None,
      iat: None,
      jti: None,
      custom: None,
    }
  }

  /// Returns the value for the issuer claim (iss).
  pub fn iss(&self) -> Option<&str> {
    self.iss.as_deref()
  }

  /// Sets a value for the issuer claim (iss).
  pub fn set_iss(&mut self, value: impl Into<String>) {
    self.iss = Some(value.into());
  }

  /// Returns the value for the subject claim (sub).
  pub fn sub(&self) -> Option<&str> {
    self.sub.as_deref()
  }

  /// Sets a value for the subject claim (sub).
  pub fn set_sub(&mut self, value: impl Into<String>) {
    self.sub = Some(value.into());
  }

  /// Returns the values for the audience claim (aud).
  pub fn aud(&self) -> Option<&[String]> {
    self.aud.as_deref()
  }

  /// Sets values for the audience claim (aud).
  pub fn set_aud(&mut self, value: impl IntoIterator<Item = impl Into<String>>) {
    self.aud = Some(value.into_iter().map(Into::into).collect());
  }

  /// Returns the time for the expires at claim (exp).
  pub fn exp(&self) -> Option<i64> {
    self.exp
  }

  /// Sets a time for the expires at claim (exp).
  pub fn set_exp(&mut self, value: impl Into<i64>) {
    self.exp = Some(value.into());
  }

  /// Returns the time for the not before claim (nbf).
  pub fn nbf(&self) -> Option<i64> {
    self.nbf
  }

  /// Sets a time for the not before claim (nbf).
  pub fn set_nbf(&mut self, value: impl Into<i64>) {
    self.nbf = Some(value.into());
  }

  /// Returns the time for the issued at claim (iat).
  pub fn iat(&self) -> Option<i64> {
    self.iat
  }

  /// Sets a time for the issued at claim (iat).
  pub fn set_iat(&mut self, value: impl Into<i64>) {
    self.iat = Some(value.into());
  }

  /// Returns the value for the JWT ID claim (jti).
  pub fn jti(&self) -> Option<&str> {
    self.jti.as_deref()
  }

  /// Sets a value for the JWT ID claim (jti).
  pub fn set_jti(&mut self, value: impl Into<String>) {
    self.jti = Some(value.into());
  }

  /// Returns a reference to the custom JWT claims.
  pub fn custom(&self) -> Option<&T> {
    self.custom.as_ref()
  }

  /// Returns a mutable reference to the custom JWT claims.
  pub fn custom_mut(&mut self) -> Option<&mut T> {
    self.custom.as_mut()
  }

  /// Sets the value of the custom JWT claims.
  pub fn set_custom(&mut self, value: impl Into<T>) {
    self.custom = Some(value.into());
  }
}
