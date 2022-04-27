// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::ops::Deref;
use core::ops::DerefMut;

use crate::error::Error;
use crate::error::Result;
use crate::jose::JoseHeader;
use crate::jwe::JweAlgorithm;
use crate::jwe::JweCompression;
use crate::jwe::JweEncryption;
use crate::jwk::Jwk;
use crate::jwt::JwtHeader;
use crate::lib::*;

/// JSON Web Encryption JOSE Header.
///
/// [More Info](https://tools.ietf.org/html/rfc7516#section-4)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct JweHeader {
  /// Common JOSE Header Parameters.
  #[serde(flatten)]
  common: JwtHeader,
  /// Algorithm.
  ///
  /// Identifies the cryptographic algorithm used to secure the JWS.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7516#section-4.1.1)
  alg: JweAlgorithm,
  /// Encryption Algorithm.
  ///
  /// Identifies the content encryption algorithm used to perform
  /// authenticated encryption on the plaintext to produce the ciphertext and
  /// the Authentication Tag.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7516#section-4.1.2)
  enc: JweEncryption,
  /// Compression Algorithm.
  ///
  /// The compression algorithm applied to the plaintext before encryption.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7516#section-4.1.3)
  #[serde(skip_serializing_if = "Option::is_none")]
  zip: Option<JweCompression>,
  /// Ephemeral Public Key.
  ///
  /// Public key created by the originator for the use in key agreement
  /// algorithms.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7518#section-4.6.1.1)
  #[serde(skip_serializing_if = "Option::is_none")]
  epk: Option<Jwk>,
  /// Agreement PartyUInfo.
  ///
  /// Value used for key derivation via Concat KDF.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7518#section-4.6.1.2)
  #[serde(skip_serializing_if = "Option::is_none")]
  apu: Option<String>,
  /// Agreement PartyVInfo.
  ///
  /// Value used for key derivation via Concat KDF.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7518#section-4.6.1.3)
  #[serde(skip_serializing_if = "Option::is_none")]
  apv: Option<String>,
  /// Initialization Vector.
  ///
  /// The base64url-encoded representation of the 96-bit IV value used for the
  /// key encryption operation.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7518#section-4.7.1.1)
  #[serde(skip_serializing_if = "Option::is_none")]
  iv: Option<String>,
  /// Authentication Tag.
  ///
  /// The base64url-encoded representation of the 128-bit Authentication Tag
  /// value resulting from the key encryption operation.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7518#section-4.7.1.2)
  #[serde(skip_serializing_if = "Option::is_none")]
  tag: Option<String>,
  /// PBES2 Salt Input.
  ///
  /// A base64url-encoded value, which is used as part of the PBKDF2 salt value.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7518#section-4.8.1.1)
  #[serde(skip_serializing_if = "Option::is_none")]
  p2s: Option<String>,
  /// PBES2 Count.
  ///
  /// The PBKDF2 iteration count
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7518#section-4.8.1.2)
  #[serde(skip_serializing_if = "Option::is_none")]
  p2c: Option<u64>,
}

impl JweHeader {
  /// Creates a new `JweHeader` with the given `alg` and `enc` claims.
  pub const fn new(alg: JweAlgorithm, enc: JweEncryption) -> Self {
    Self {
      common: JwtHeader::new(),
      alg,
      enc,
      zip: None,
      epk: None,
      apu: None,
      apv: None,
      iv: None,
      tag: None,
      p2s: None,
      p2c: None,
    }
  }

  pub fn set_common(&mut self, jwt_header: JwtHeader) {
    self.common = jwt_header;
  }

  pub fn common(&self) -> &JwtHeader {
    &self.common
  }

  /// Returns the value for the algorithm claim (alg).
  pub fn alg(&self) -> JweAlgorithm {
    self.alg
  }

  /// Sets a value for the algorithm claim (alg).
  pub fn set_alg(&mut self, value: impl Into<JweAlgorithm>) {
    self.alg = value.into();
  }

  /// Returns the value of the encryption claim (enc).
  pub fn enc(&self) -> JweEncryption {
    self.enc
  }

  /// Sets a value for the encryption claim (enc).
  pub fn set_enc(&mut self, value: impl Into<JweEncryption>) {
    self.enc = value.into();
  }

  /// Returns the value of the compression claim (zip).
  pub fn zip(&self) -> Option<JweCompression> {
    self.zip
  }

  /// Returns the value of the compression claim (zip).
  pub fn try_zip(&self) -> Result<JweCompression> {
    self.zip().ok_or(Error::MissingParam("zip"))
  }

  /// Sets a value for the compression claim (zip).
  pub fn set_zip(&mut self, value: impl Into<JweCompression>) {
    self.zip = Some(value.into());
  }

  /// Returns the value of the ephemeral public key claim (epk).
  pub fn epk(&self) -> Option<&Jwk> {
    self.epk.as_ref()
  }

  /// Returns the value of the ephemeral public key claim (epk).
  pub fn try_epk(&self) -> Result<&Jwk> {
    self.epk().ok_or(Error::MissingParam("epk"))
  }

  /// Sets a value for the ephemeral public key claim (epk).
  pub fn set_epk(&mut self, value: impl Into<Jwk>) {
    self.epk = Some(value.into());
  }

  /// Returns the value of the partyuinfo claim (apu).
  pub fn apu(&self) -> Option<&str> {
    self.apu.as_deref()
  }

  /// Returns the value of the partyuinfo claim (apu).
  pub fn try_apu(&self) -> Result<&str> {
    self.apu().ok_or(Error::MissingParam("apu"))
  }

  /// Sets a value for the partyuinfo claim (apu).
  pub fn set_apu(&mut self, value: impl Into<String>) {
    self.apu = Some(value.into());
  }

  /// Returns the value of the partyvinfo claim (apv).
  pub fn apv(&self) -> Option<&str> {
    self.apv.as_deref()
  }

  /// Returns the value of the partyvinfo claim (apv).
  pub fn try_apv(&self) -> Result<&str> {
    self.apv().ok_or(Error::MissingParam("apv"))
  }

  /// Sets a value for the partyvinfo claim (apv).
  pub fn set_apv(&mut self, value: impl Into<String>) {
    self.apv = Some(value.into());
  }

  /// Returns the value of the initialization vector claim (iv).
  pub fn iv(&self) -> Option<&str> {
    self.iv.as_deref()
  }

  /// Returns the value of the initialization vector claim (iv).
  pub fn try_iv(&self) -> Result<&str> {
    self.iv().ok_or(Error::MissingParam("iv"))
  }

  /// Sets a value for the initialization vector claim (iv).
  pub fn set_iv(&mut self, value: impl Into<String>) {
    self.iv = Some(value.into());
  }

  /// Returns the value of the authentication tag claim (tag).
  pub fn tag(&self) -> Option<&str> {
    self.tag.as_deref()
  }

  /// Returns the value of the authentication tag claim (tag).
  pub fn try_tag(&self) -> Result<&str> {
    self.tag().ok_or(Error::MissingParam("tag"))
  }

  /// Sets a value for the authentication tag claim (tag).
  pub fn set_tag(&mut self, value: impl Into<String>) {
    self.tag = Some(value.into());
  }

  /// Returns the value of the authentication pbes2 salt input claim (p2s).
  pub fn p2s(&self) -> Option<&str> {
    self.p2s.as_deref()
  }

  /// Returns the value of the authentication pbes2 salt input claim (p2s).
  pub fn try_p2s(&self) -> Result<&str> {
    self.p2s().ok_or(Error::MissingParam("p2s"))
  }

  /// Sets a value for the authentication pbes2 salt input claim (p2s).
  pub fn set_p2s(&mut self, value: impl Into<String>) {
    self.p2s = Some(value.into());
  }

  /// Returns the value of the authentication pbes2 count claim (p2c).
  pub fn p2c(&self) -> Option<u64> {
    self.p2c
  }

  /// Returns the value of the authentication pbes2 count claim (p2c).
  pub fn try_p2c(&self) -> Result<u64> {
    self.p2c().ok_or(Error::MissingParam("p2c"))
  }

  /// Sets a value for the authentication pbes2 count claim (p2c).
  pub fn set_p2c(&mut self, value: impl Into<u64>) {
    self.p2c = Some(value.into());
  }

  // ===========================================================================
  // ===========================================================================

  pub fn has(&self, claim: &str) -> bool {
    match claim {
      "alg" => true, // we always have an algorithm
      "enc" => true, // we always have an encryption algorithm
      "zip" => self.zip().is_some(),
      "epk" => self.epk().is_some(),
      "apu" => self.apu().is_some(),
      "apv" => self.apv().is_some(),
      "iv" => self.iv().is_some(),
      "tag" => self.tag().is_some(),
      "p2s" => self.p2s().is_some(),
      "p2c" => self.p2c().is_some(),
      _ => self.common.has(claim),
    }
  }
}

impl Deref for JweHeader {
  type Target = JwtHeader;

  fn deref(&self) -> &Self::Target {
    &self.common
  }
}

impl DerefMut for JweHeader {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.common
  }
}

impl JoseHeader for JweHeader {
  fn common(&self) -> &JwtHeader {
    self
  }

  fn has_claim(&self, claim: &str) -> bool {
    self.has(claim)
  }
}
