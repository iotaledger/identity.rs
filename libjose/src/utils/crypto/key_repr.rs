use crate::error::Error;
use crate::error::Result;
use crate::jwk::Jwk;
use crate::lib::*;
use crate::utils;

#[derive(Clone, Copy, Debug)]
pub enum Secret<'a> {
  Arr(&'a [u8]),
  Jwk(&'a Jwk),
}

impl<'a> Secret<'a> {
  pub fn check_signing_key(&self, algorithm: &str) -> Result<()> {
    if let Self::Jwk(jwk) = self {
      jwk.check_signing_key(algorithm)?;
    }

    Ok(())
  }

  pub fn check_verifying_key(&self, algorithm: &str) -> Result<()> {
    if let Self::Jwk(jwk) = self {
      jwk.check_verifying_key(algorithm)?;
    }

    Ok(())
  }

  pub fn check_encryption_key(&self, algorithm: &str) -> Result<()> {
    if let Self::Jwk(jwk) = self {
      jwk.check_encryption_key(algorithm)?;
    }

    Ok(())
  }

  pub fn check_decryption_key(&self, algorithm: &str) -> Result<()> {
    if let Self::Jwk(jwk) = self {
      jwk.check_decryption_key(algorithm)?;
    }

    Ok(())
  }

  pub fn to_oct_key(self, key_len: usize) -> Result<Cow<'a, [u8]>> {
    utils::expand_oct_key(key_len, self)
  }

  pub fn to_rsa_public(self) -> Result<utils::RsaPublicKey> {
    utils::expand_rsa_public(self)
  }

  pub fn to_rsa_secret(self) -> Result<utils::RsaSecretKey> {
    utils::expand_rsa_secret(self)
  }

  pub fn to_p256_public(self) -> Result<utils::P256PublicKey> {
    utils::expand_p256_public(self)
  }

  pub fn to_p256_secret(self) -> Result<utils::P256SecretKey> {
    utils::expand_p256_secret(self)
  }

  pub fn to_k256_public(self) -> Result<utils::K256PublicKey> {
    utils::expand_k256_public(self)
  }

  pub fn to_k256_secret(self) -> Result<utils::K256SecretKey> {
    utils::expand_k256_secret(self)
  }

  pub fn to_ed25519_public(self) -> Result<utils::Ed25519PublicKey> {
    utils::expand_ed25519_public(self)
  }

  pub fn to_ed25519_secret(self) -> Result<utils::Ed25519SecretKey> {
    utils::expand_ed25519_secret(self)
  }

  pub fn to_x25519_public(self) -> Result<utils::X25519PublicKey> {
    utils::expand_x25519_public(self)
  }

  pub fn to_x25519_secret(self) -> Result<utils::X25519SecretKey> {
    utils::expand_x25519_secret(self)
  }

  pub fn to_x448_public(self) -> Result<utils::X448PublicKey> {
    utils::expand_x448_public(self)
  }

  pub fn to_x448_secret(self) -> Result<utils::X448SecretKey> {
    utils::expand_x448_secret(self)
  }

  pub(crate) fn expand<T, E>(
    self,
    expand_arr: impl Fn(&[u8]) -> Result<T, E>,
    expand_jwk: impl Fn(&Jwk) -> Result<Vec<u8>>,
  ) -> Result<T>
  where
    E: Into<Error>,
  {
    match self {
      Self::Arr(arr) => expand_arr(arr).map_err(Into::into),
      Self::Jwk(jwk) => expand_arr(&expand_jwk(jwk)?).map_err(Into::into),
    }
  }
}

impl<'a> From<&'a [u8]> for Secret<'a> {
  fn from(other: &'a [u8]) -> Self {
    Self::Arr(other)
  }
}

impl<'a> From<&'a Jwk> for Secret<'a> {
  fn from(other: &'a Jwk) -> Self {
    Self::Jwk(other)
  }
}
