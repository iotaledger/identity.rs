use core::iter::FromIterator;
use url::Url;

use crate::jwe::JweAlgorithm;
use crate::jwe::JweCompression;
use crate::jwe::JweEncryption;
use crate::jwk::Jwk;
use crate::utils::Empty;

/// JSON Web Encryption JOSE Header.
///
/// [More Info](https://tools.ietf.org/html/rfc7516#section-4)
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct JweHeader<T = Empty> {
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
  /// JWK Set URL.
  ///
  /// A JWK Set containing the public key to which the JWE was encrypted; this
  /// can be used to determine the private key needed to decrypt the JWE.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7516#section-4.1.4)
  #[serde(skip_serializing_if = "Option::is_none")]
  jku: Option<Url>,
  /// JSON Web Key.
  ///
  /// The public key to which the JWE was encrypted; this can be used to
  /// determine the private key needed to decrypt the JWE.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7516#section-4.1.5)
  #[serde(skip_serializing_if = "Option::is_none")]
  jwk: Option<Jwk>,
  /// Key ID.
  ///
  /// A hint indicating which key was used to encrypt the JWE.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7516#section-4.1.6)
  #[serde(skip_serializing_if = "Option::is_none")]
  kid: Option<String>,
  /// X.509 URL.
  ///
  /// A URI that refers to a resource for the X.509 public key certificate or
  /// certificate chain corresponding to the key used to encrypt the JWE.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7516#section-4.1.7)
  #[serde(skip_serializing_if = "Option::is_none")]
  x5u: Option<Url>,
  /// X.509 Certificate Chain.
  ///
  /// Contains the X.509 public key certificate or certificate chain
  /// corresponding to the key used to encrypt the JWE.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7516#section-4.1.8)
  #[serde(skip_serializing_if = "Option::is_none")]
  x5c: Option<Vec<Vec<u8>>>,
  /// X.509 Certificate SHA-1 Thumbprint.
  ///
  /// A base64url-encoded SHA-1 thumbprint of the DER encoding of the X.509
  /// certificate corresponding to the key used to encrypt the JWE.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7516#section-4.1.9)
  #[serde(skip_serializing_if = "Option::is_none")]
  x5t: Option<Vec<u8>>,
  /// X.509 Certificate SHA-256 Thumbprint.
  ///
  /// A base64url-encoded SHA-256 thumbprint of the DER encoding of the X.509
  /// certificate corresponding to the key used to encrypt the JWE.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7516#section-4.1.10)
  #[serde(rename = "x5t#S256", skip_serializing_if = "Option::is_none")]
  x5t_s256: Option<Vec<u8>>,
  /// Type.
  ///
  /// Used by JWE applications to declare the media type of the
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7516#section-4.1.11)
  #[serde(skip_serializing_if = "Option::is_none")]
  typ: Option<String>,
  /// Content Type.
  ///
  /// Used by JWE applications to declare the media type of the secured content.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7516#section-4.1.12)
  #[serde(skip_serializing_if = "Option::is_none")]
  cty: Option<String>,
  /// Critical.
  ///
  /// Indicates that JWE extensions are being used that MUST be understood and
  /// processed.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7516#section-4.1.13)
  #[serde(skip_serializing_if = "Option::is_none")]
  crit: Option<Vec<String>>,
  /// URL.
  ///
  /// Specifies the URL to which this JWS object is directed.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc8555#section-6.4.1)
  #[serde(skip_serializing_if = "Option::is_none")]
  url: Option<Url>,
  /// Nonce.
  ///
  /// Provides a unique value that enables the verifier of a JWS to recognize
  /// when replay has occurred.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc8555#section-6.5.2)
  #[serde(skip_serializing_if = "Option::is_none")]
  nonce: Option<Vec<u8>>,
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
  /// Public/Private Claim Names
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7516#section-4.2)
  #[serde(flatten, skip_serializing_if = "Option::is_none")]
  custom: Option<T>,
}

impl<T> JweHeader<T> {
  /// Creates a new `JweHeader`.
  pub const fn new() -> Self {
    Self::with_alg_and_enc(JweAlgorithm::ECDH_ES, JweEncryption::A128CBC_HS256)
  }

  /// Creates a new `JweHeader` with the given `alg` claim.
  pub const fn with_alg(alg: JweAlgorithm) -> Self {
    Self::with_alg_and_enc(alg, JweEncryption::A128CBC_HS256)
  }

  /// Creates a new `JweHeader` with the given `enc` claim.
  pub const fn with_enc(enc: JweEncryption) -> Self {
    Self::with_alg_and_enc(JweAlgorithm::ECDH_ES, enc)
  }

  /// Creates a new `JweHeader` with the given `alg` and `enc` claims.
  pub const fn with_alg_and_enc(alg: JweAlgorithm, enc: JweEncryption) -> Self {
    Self {
      alg,
      enc,
      zip: None,
      jku: None,
      jwk: None,
      kid: None,
      x5u: None,
      x5c: None,
      x5t: None,
      x5t_s256: None,
      typ: None,
      cty: None,
      crit: None,
      url: None,
      nonce: None,
      epk: None,
      apu: None,
      apv: None,
      iv: None,
      tag: None,
      p2s: None,
      p2c: None,
      custom: None,
    }
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
  pub fn zip(&self) -> Option<&JweCompression> {
    self.zip.as_ref()
  }

  /// Sets a value for the compression claim (zip).
  pub fn set_zip(&mut self, value: impl Into<JweCompression>) {
    self.zip = Some(value.into());
  }

  /// Returns the value of the JWK set URL claim (jku).
  pub fn jku(&self) -> Option<&Url> {
    self.jku.as_ref()
  }

  /// Sets a value for the JWK set URL claim (jku).
  pub fn set_jku(&mut self, value: impl Into<Url>) {
    self.jku = Some(value.into());
  }

  /// Returns the value of the JWK claim (jwk).
  pub fn jwk(&self) -> Option<&Jwk> {
    self.jwk.as_ref()
  }

  /// Sets a value for the JWK claim (jwk).
  pub fn set_jwk(&mut self, value: impl Into<Jwk>) {
    self.jwk = Some(value.into());
  }

  /// Returns the value of the key ID claim (kid).
  pub fn kid(&self) -> Option<&str> {
    self.kid.as_deref()
  }

  /// Sets a value for the key ID claim (kid).
  pub fn set_kid(&mut self, value: impl Into<String>) {
    self.kid = Some(value.into());
  }

  /// Returns the value of the X.509 URL claim (x5u).
  pub fn x5u(&self) -> Option<&Url> {
    self.x5u.as_ref()
  }

  /// Sets a value for the X.509 URL claim (x5u).
  pub fn set_x5u(&mut self, value: impl Into<Url>) {
    self.x5u = Some(value.into());
  }

  /// Returns the value of the X.509 certificate chain claim (x5c).
  pub fn x5c(&self) -> Option<&[Vec<u8>]> {
    self.x5c.as_deref()
  }

  /// Sets values for the X.509 certificate chain claim (x5c).
  pub fn set_x5c(&mut self, value: impl IntoIterator<Item = impl IntoIterator<Item = u8>>) {
    self.x5c = Some(value.into_iter().map(Vec::from_iter).collect());
  }

  /// Returns the value of the X.509 certificate SHA-1 thumbprint claim (x5t).
  pub fn x5t(&self) -> Option<&[u8]> {
    self.x5t.as_deref()
  }

  /// Sets a value for the X.509 certificate SHA-1 thumbprint claim (x5t).
  pub fn set_x5t(&mut self, value: impl IntoIterator<Item = u8>) {
    self.x5t = Some(Vec::from_iter(value.into_iter()));
  }

  /// Returns the value of the X.509 certificate SHA-256 thumbprint claim
  /// (x5t#S256).
  pub fn x5t_s256(&self) -> Option<&[u8]> {
    self.x5t_s256.as_deref()
  }

  /// Sets a value for the X.509 certificate SHA-256 thumbprint claim
  /// (x5t#S256).
  pub fn set_x5t_s256(&mut self, value: impl IntoIterator<Item = u8>) {
    self.x5t_s256 = Some(Vec::from_iter(value.into_iter()));
  }

  /// Returns the value of the token type claim (typ).
  pub fn typ(&self) -> Option<&str> {
    self.typ.as_deref()
  }

  /// Sets a value for the token type claim (typ).
  pub fn set_typ(&mut self, value: impl Into<String>) {
    self.typ = Some(value.into());
  }

  /// Returns the value of the content type claim (cty).
  pub fn cty(&self) -> Option<&str> {
    self.cty.as_deref()
  }

  /// Sets a value for the content type claim (cty).
  pub fn set_cty(&mut self, value: impl Into<String>) {
    self.cty = Some(value.into());
  }

  /// Returns the value of the critical claim (crit).
  pub fn crit(&self) -> Option<&[String]> {
    self.crit.as_deref()
  }

  /// Sets values for the critical claim (crit).
  pub fn set_crit(&mut self, value: impl IntoIterator<Item = impl Into<String>>) {
    self.crit = Some(Vec::from_iter(value.into_iter().map(Into::into)));
  }

  /// Returns the value of the url claim (url).
  pub fn url(&self) -> Option<&Url> {
    self.url.as_ref()
  }

  /// Sets a value for the url claim (url).
  pub fn set_url(&mut self, value: impl Into<Url>) {
    self.url = Some(value.into());
  }

  /// Returns the value of the nonce claim (nonce).
  pub fn nonce(&self) -> Option<&[u8]> {
    self.nonce.as_deref()
  }

  /// Sets a value for the nonce claim (nonce).
  pub fn set_nonce(&mut self, value: impl IntoIterator<Item = u8>) {
    self.nonce = Some(Vec::from_iter(value.into_iter()));
  }

  /// Returns the value of the ephemeral public key claim (epk).
  pub fn epk(&self) -> Option<&Jwk> {
    self.epk.as_ref()
  }

  /// Sets a value for the ephemeral public key claim (epk).
  pub fn set_epk(&mut self, value: impl Into<Jwk>) {
    self.epk = Some(value.into());
  }

  /// Returns the value of the partyuinfo claim (apu).
  pub fn apu(&self) -> Option<&str> {
    self.apu.as_deref()
  }

  /// Sets a value for the partyuinfo claim (apu).
  pub fn set_apu(&mut self, value: impl Into<String>) {
    self.apu = Some(value.into());
  }

  /// Returns the value of the partyvinfo claim (apv).
  pub fn apv(&self) -> Option<&str> {
    self.apv.as_deref()
  }

  /// Sets a value for the partyvinfo claim (apv).
  pub fn set_apv(&mut self, value: impl Into<String>) {
    self.apv = Some(value.into());
  }

  /// Returns the value of the initialization vector claim (iv).
  pub fn iv(&self) -> Option<&str> {
    self.iv.as_deref()
  }

  /// Sets a value for the initialization vector claim (iv).
  pub fn set_iv(&mut self, value: impl Into<String>) {
    self.iv = Some(value.into());
  }

  /// Returns the value of the authentication tag claim (tag).
  pub fn tag(&self) -> Option<&str> {
    self.tag.as_deref()
  }

  /// Sets a value for the authentication tag claim (tag).
  pub fn set_tag(&mut self, value: impl Into<String>) {
    self.tag = Some(value.into());
  }

  /// Returns the value of the authentication pbes2 salt input claim (p2s).
  pub fn p2s(&self) -> Option<&str> {
    self.p2s.as_deref()
  }

  /// Sets a value for the authentication pbes2 salt input claim (p2s).
  pub fn set_p2s(&mut self, value: impl Into<String>) {
    self.p2s = Some(value.into());
  }

  /// Returns the value of the authentication pbes2 count claim (p2c).
  pub fn p2c(&self) -> Option<u64> {
    self.p2c
  }

  /// Sets a value for the authentication pbes2 count claim (p2c).
  pub fn set_p2c(&mut self, value: impl Into<u64>) {
    self.p2c = Some(value.into());
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
