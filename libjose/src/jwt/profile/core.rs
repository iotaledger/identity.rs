use core::time::Duration;
use core::convert::TryFrom;
use std::time::SystemTime;

use crate::error::ValidationError;
use crate::error::Result;
// use crate::jwe::JweHeader;
// use crate::jws::JwsHeader;
use crate::jwt::JwtClaims;
use crate::utils::Empty;

bitflags::bitflags! {
  pub struct CoreClaim: u8 {
    const EMPTY = 0b00000000;
    const ISS   = 0b00000001;
    const SUB   = 0b00000010;
    const AUD   = 0b00000100;
    const EXP   = 0b00001000;
    const NBF   = 0b00010000;
    const IAT   = 0b00100000;
    const JTI   = 0b01000000;
  }
}

#[derive(Clone, Debug)]
pub struct CoreProfile {
  check_iss: Option<String>,
  check_sub: Option<String>,
  check_aud: Option<String>,
  check_jti: Option<String>,
  check_exp: Option<Empty>,
  check_nbf: Option<Empty>,
  check_iat: Option<Empty>,
  required: CoreClaim,
  timecop: Option<TimeCop>,
}

impl CoreProfile {
  /// Creates a new `CoreProfile` validator.
  pub const fn new() -> Self {
    Self {
      check_iss: None,
      check_sub: None,
      check_aud: None,
      check_jti: None,
      check_exp: None,
      check_nbf: None,
      check_iat: None,
      required: CoreClaim::EMPTY,
      timecop: None,
    }
  }

  /// Requires the presence of the issuer claim (iss).
  pub fn set_iss(&mut self, value: impl Into<String>) {
    self.check_iss = Some(value.into());
  }

  /// Requires the presence of the subject claim (sub).
  pub fn set_sub(&mut self, value: impl Into<String>) {
    self.check_sub = Some(value.into());
  }

  /// Requires the presence of the audience claim (aud).
  pub fn set_aud(&mut self, value: impl Into<String>) {
    self.check_aud = Some(value.into());
  }

  /// Requires the presence of the token ID claim (jti).
  pub fn set_jti(&mut self, value: impl Into<String>) {
    self.check_jti = Some(value.into());
  }

  /// Requires the presence of the expiration claim (exp).
  pub fn set_exp(&mut self) {
    self.check_exp = Some(Empty);
  }

  /// Requires the presence of the not-before claim (nbf).
  pub fn set_nbf(&mut self) {
    self.check_nbf = Some(Empty);
  }

  /// Requires the presence of the issued-at claim (iat).
  pub fn set_iat(&mut self) {
    self.check_iat = Some(Empty);
  }

  /// Sets options for timestamp validation.
  pub fn set_timecop(&mut self, value: impl Into<TimeCop>) {
    self.timecop = Some(value.into());
  }

  /// Validates the given claims with the current rule configuration.
  pub fn validate<T>(&self, claims: &JwtClaims<T>) -> Result<()> {
    // Ensure presence of all required claims
    self.validate_required_claims(claims)?;

    // Ensure strict values of specific registered claims
    self.validate_aud(claims)?;
    self.validate_iss(claims)?;
    self.validate_jti(claims)?;
    self.validate_sub(claims)?;

    // Check expiration/issuance time/etc.
    self.validate_timestamps(claims)?;

    Ok(())
  }

  // Ensures the configured registered claims are present in the JWT claims
  // object.
  //
  // Note: Only validates the presence of the claim, not the content.
  fn validate_required_claims<T>(&self, claims: &JwtClaims<T>) -> Result<()> {
    type Item = (CoreClaim, &'static str, bool);

    let filter_map = |(claim, name, exists): Item| -> Option<&'static str> {
      if self.required.contains(claim) && !exists {
        Some(name)
      } else {
        None
      }
    };

    let claims: Vec<Item> = vec![
      (CoreClaim::ISS, "iss", claims.iss().is_some()),
      (CoreClaim::SUB, "sub", claims.sub().is_some()),
      (CoreClaim::AUD, "aud", claims.aud().is_some()),
      (CoreClaim::EXP, "exp", claims.exp().is_some()),
      (CoreClaim::NBF, "nbf", claims.nbf().is_some()),
      (CoreClaim::IAT, "iat", claims.iat().is_some()),
      (CoreClaim::JTI, "jti", claims.jti().is_some()),
    ];

    let invalid: Vec<&'static str> = claims
      .into_iter()
      .filter_map(filter_map)
      .collect();

    if !invalid.is_empty() {
      return Err(ValidationError::MissingClaims(invalid).into());
    }

    Ok(())
  }

  // Validates the audience (aud) claim value.
  fn validate_aud<T>(&self, claims: &JwtClaims<T>) -> Result<()> {
    match (self.check_aud.as_ref(), claims.aud()) {
      (None, _) => Ok(()),
      (Some(aud), Some(value)) if value.contains(&aud) => Ok(()),
      (Some(_), Some(_)) => Err(ValidationError::InvalidAudience.into()),
      (Some(_), None) => Ok(()),
    }
  }

  // Validates the issuer (iss) claim value.
  fn validate_iss<T>(&self, claims: &JwtClaims<T>) -> Result<()> {
    match (self.check_iss.as_ref(), claims.iss()) {
      (None, _) => Ok(()),
      (Some(iss), Some(value)) if iss == value => Ok(()),
      (Some(_), Some(_)) => Err(ValidationError::InvalidIssuer.into()),
      (Some(_), None) => Ok(()),
    }
  }

  // Validates the JWT ID (jti) claim value.
  fn validate_jti<T>(&self, claims: &JwtClaims<T>) -> Result<()> {
    match (self.check_jti.as_ref(), claims.jti()) {
      (None, _) => Ok(()),
      (Some(jti), Some(value)) if jti == value => Ok(()),
      (Some(_), Some(_)) => Err(ValidationError::InvalidTokenId.into()),
      (Some(_), None) => Ok(()),
    }
  }

  // Validates the subject (sub) claim value.
  fn validate_sub<T>(&self, claims: &JwtClaims<T>) -> Result<()> {
    match (self.check_sub.as_ref(), claims.sub()) {
      (None, _) => Ok(()),
      (Some(sub), Some(value)) if sub == value => Ok(()),
      (Some(_), Some(_)) => Err(ValidationError::InvalidTokenId.into()),
      (Some(_), None) => Ok(()),
    }
  }

  // Validates the registered timestamp claims (exp, nbf, iat)
  fn validate_timestamps<T>(&self, claims: &JwtClaims<T>) -> Result<()> {
    fn timestamp(value: i64) -> Duration {
      u64::try_from(value).map(Duration::from_secs).unwrap_or_default()
    }

    let timecop: TimeCop = self.timecop.unwrap_or_else(TimeCop::new);
    let current: Duration = timecop.resolve_current();
    let min_iat: Duration = timecop.min_iat.unwrap_or_default();
    let max_iat: Duration = timecop.max_iat.unwrap_or(current);

    // Check the expiration claim and ensure the token hasn't expired
    if let Some((exp, _)) = claims.exp().zip(self.check_exp) {
      if timestamp(exp) <= current {
        return Err(ValidationError::TokenExpired.into());
      }
    }

    // Check the "not before" claim and ensure the token isn't used before intended
    if let Some((nbf, _)) = claims.nbf().zip(self.check_nbf) {
      if timestamp(nbf) > current {
        return Err(ValidationError::TokenNotYetValid.into());
      }
    }

    // Ensure the token was issued within the appropriate issuance period
    if let Some((iat, _)) = claims.iat().zip(self.check_iat) {
      if timestamp(iat) < min_iat {
        return Err(ValidationError::TokenNotYetValid.into());
      } else if timestamp(iat) > max_iat {
        return Err(ValidationError::TokenExpired.into());
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

  pub fn set_current(&mut self, value: impl Into<SystemTime>) {
    self.current = Some(value.into());
  }

  pub fn set_max_iat(&mut self, value: impl Into<Duration>) {
    self.max_iat = Some(value.into());
  }

  pub fn set_min_iat(&mut self, value: impl Into<Duration>) {
    self.min_iat = Some(value.into());
  }

  fn resolve_current(&self) -> Duration {
    self
      .current
      .unwrap_or_else(SystemTime::now)
      .duration_since(SystemTime::UNIX_EPOCH)
      .expect("Epoch Fail")
  }
}
