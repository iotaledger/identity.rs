// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::time::Duration;

#[cfg(feature = "std")]
use std::time::SystemTime;
#[cfg(not(feature = "std"))]
type SystemTime = ();

use crate::error::Error;
use crate::error::Result;
use crate::jwt::JwtClaims;

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Validation {
  /// Indicates that the claim/property is NOT required.
  Optional,
  /// Indicates that the claim/property is required; the value is NOT validated.
  Required,
  /// Indicates that the claim/property is required AND the value must match the
  /// given string.
  Matching(String),
}

impl From<String> for Validation {
  fn from(other: String) -> Self {
    Self::Matching(other)
  }
}

#[derive(Clone, Debug)]
pub struct CoreProfile {
  rule_iss: Validation,
  rule_sub: Validation,
  rule_aud: Validation,
  rule_jti: Validation,
  rule_exp: Validation,
  rule_nbf: Validation,
  rule_iat: Validation,
  timecop: Option<TimeCop>,
}

impl CoreProfile {
  /// Creates a new `CoreProfile` validator.
  pub const fn new() -> Self {
    Self {
      rule_iss: Validation::Optional,
      rule_sub: Validation::Optional,
      rule_aud: Validation::Optional,
      rule_jti: Validation::Optional,
      rule_exp: Validation::Optional,
      rule_nbf: Validation::Optional,
      rule_iat: Validation::Optional,
      timecop: None,
    }
  }

  /// Sets validation rules for the issuer claim (iss).
  pub fn set_iss(&mut self, value: impl Into<Validation>) {
    self.rule_iss = value.into();
  }

  /// Sets validation rules for the subject claim (sub).
  pub fn set_sub(&mut self, value: impl Into<Validation>) {
    self.rule_sub = value.into();
  }

  /// Sets validation rules for the audience claim (aud).
  pub fn set_aud(&mut self, value: impl Into<Validation>) {
    self.rule_aud = value.into();
  }

  /// Sets validation rules for the token ID claim (jti).
  pub fn set_jti(&mut self, value: impl Into<Validation>) {
    self.rule_jti = value.into();
  }

  /// Sets validation rules for the expiration claim (exp).
  pub fn set_exp(&mut self, value: impl Into<Validation>) {
    self.rule_exp = value.into();
  }

  /// Sets validation rules for the not-before claim (nbf).
  pub fn set_nbf(&mut self, value: impl Into<Validation>) {
    self.rule_nbf = value.into();
  }

  /// Sets validation rules for the issued-at claim (iat).
  pub fn set_iat(&mut self, value: impl Into<Validation>) {
    self.rule_iat = value.into();
  }

  /// Sets options for timestamp validation.
  pub fn set_timecop(&mut self, value: impl Into<TimeCop>) {
    self.timecop = Some(value.into());
  }

  /// Validates the given claims with the current rule configuration.
  pub fn validate<T>(&self, claims: &JwtClaims<T>) -> Result<()> {
    // Validate registered claims with the current rules.
    self.validate_aud(claims)?;
    self.validate_iss(claims)?;
    self.validate_jti(claims)?;
    self.validate_sub(claims)?;

    #[cfg(feature = "std")]
    {
      // Check expiration/issuance time/etc.
      self.validate_timestamps(claims)?;
    }

    Ok(())
  }

  // Validates the audience (aud) claim value.
  fn validate_aud<T>(&self, claims: &JwtClaims<T>) -> Result<()> {
    match (&self.rule_aud, claims.aud()) {
      (Validation::Optional, _) => Ok(()),
      (Validation::Required, Some(_)) => Ok(()),
      (Validation::Matching(expected), Some(aud)) if aud.contains(expected) => Ok(()),
      (Validation::Required, _) => Err(Error::MissingClaim("aud")),
      (Validation::Matching(_), _) => Err(Error::InvalidClaim("aud")),
    }
  }

  // Validates the issuer (iss) claim value.
  fn validate_iss<T>(&self, claims: &JwtClaims<T>) -> Result<()> {
    match (&self.rule_iss, claims.iss()) {
      (Validation::Optional, _) => Ok(()),
      (Validation::Required, Some(_)) => Ok(()),
      (Validation::Matching(expected), Some(iss)) if iss == expected => Ok(()),
      (Validation::Required, _) => Err(Error::MissingClaim("iss")),
      (Validation::Matching(_), _) => Err(Error::InvalidClaim("iss")),
    }
  }

  // Validates the JWT ID (jti) claim value.
  fn validate_jti<T>(&self, claims: &JwtClaims<T>) -> Result<()> {
    match (&self.rule_jti, claims.jti()) {
      (Validation::Optional, _) => Ok(()),
      (Validation::Required, Some(_)) => Ok(()),
      (Validation::Matching(expected), Some(jti)) if jti == expected => Ok(()),
      (Validation::Required, _) => Err(Error::MissingClaim("jti")),
      (Validation::Matching(_), _) => Err(Error::InvalidClaim("jti")),
    }
  }

  // Validates the subject (sub) claim value.
  fn validate_sub<T>(&self, claims: &JwtClaims<T>) -> Result<()> {
    match (&self.rule_sub, claims.sub()) {
      (Validation::Optional, _) => Ok(()),
      (Validation::Required, Some(_)) => Ok(()),
      (Validation::Matching(expected), Some(sub)) if sub == expected => Ok(()),
      (Validation::Required, _) => Err(Error::MissingClaim("sub")),
      (Validation::Matching(_), _) => Err(Error::InvalidClaim("sub")),
    }
  }

  // Validates the registered timestamp claims (exp, nbf, iat)
  #[cfg(feature = "std")]
  fn validate_timestamps<T>(&self, claims: &JwtClaims<T>) -> Result<()> {
    fn timestamp(value: i64) -> Duration {
      use core::convert::TryFrom as _;
      u64::try_from(value).map(Duration::from_secs).unwrap_or_default()
    }

    let timecop: TimeCop = self.timecop.unwrap_or_else(TimeCop::new);
    let current: Duration = timecop.resolve_current();
    let min_iat: Duration = timecop.min_iat.unwrap_or_default();
    let max_iat: Duration = timecop.max_iat.unwrap_or(current);

    // Check the expiration claim and ensure the token hasn't expired
    match (&self.rule_exp, claims.exp()) {
      (Validation::Optional, _) => {}
      (Validation::Matching(_), _) => {}
      (Validation::Required, Some(exp)) if timestamp(exp) <= current => {
        return Err(Error::InvalidClaim("exp"));
      }
      (Validation::Required, Some(_)) => {}
      (Validation::Required, None) => {
        return Err(Error::MissingClaim("exp"));
      }
    }

    // Check the "not before" claim and ensure the token isn't used before intended
    match (&self.rule_nbf, claims.nbf()) {
      (Validation::Optional, _) => {}
      (Validation::Matching(_), _) => {}
      (Validation::Required, Some(nbf)) if timestamp(nbf) > current => {
        return Err(Error::InvalidClaim("exp"));
      }
      (Validation::Required, Some(_)) => {}
      (Validation::Required, None) => {
        return Err(Error::MissingClaim("nbf"));
      }
    }

    // Ensure the token was issued within the appropriate issuance period
    match (&self.rule_iat, claims.iat()) {
      (Validation::Optional, _) => {}
      (Validation::Matching(_), _) => {}
      (Validation::Required, Some(iat)) if timestamp(iat) < min_iat => {
        return Err(Error::InvalidClaim("iat"));
      }
      (Validation::Required, Some(iat)) if timestamp(iat) > max_iat => {
        return Err(Error::InvalidClaim("iat"));
      }
      (Validation::Required, Some(_)) => {}
      (Validation::Required, None) => {
        return Err(Error::MissingClaim("iat"));
      }
    }

    Ok(())
  }
}

/// Validation options for time-related claims and properties.
#[derive(Clone, Copy, Debug)]
pub struct TimeCop {
  /// A specific time for temporal validations. Defaults to `SystemTime::now()`.
  current: Option<SystemTime>,
  /// The maximum allowed time of the issued-at claim (iat).
  max_iat: Option<Duration>,
  /// The minimum allowed time of the issued-at claim (iat).
  min_iat: Option<Duration>,
}

impl TimeCop {
  /// Creates a new `TimeCop`.
  pub const fn new() -> Self {
    Self {
      current: None,
      max_iat: None,
      min_iat: None,
    }
  }

  pub fn set_current(&mut self, value: SystemTime) {
    self.current = Some(value);
  }

  pub fn set_max_iat(&mut self, value: impl Into<Duration>) {
    self.max_iat = Some(value.into());
  }

  pub fn set_min_iat(&mut self, value: impl Into<Duration>) {
    self.min_iat = Some(value.into());
  }

  #[cfg(feature = "std")]
  fn resolve_current(&self) -> Duration {
    self
      .current
      .unwrap_or_else(SystemTime::now)
      .duration_since(SystemTime::UNIX_EPOCH)
      .expect("Epoch Fail")
  }
}
