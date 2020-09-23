use core::iter::FromIterator;
use url::Url;

use crate::alloc::String;
use crate::alloc::Vec;
use crate::jwk::Jwk;
use crate::jws::JwsAlgorithm;
use crate::utils::Empty;

/// JSON Web Signature JOSE Header.
///
/// [More Info](https://tools.ietf.org/html/rfc7515#section-4)
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct JwsHeader<T = Empty> {
  /// Algorithm.
  ///
  /// Identifies the cryptographic algorithm used to secure the JWS.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7515#section-4.1.1)
  alg: JwsAlgorithm,
  /// JWK Set URL.
  ///
  /// Refers to a resource for a set of JSON-encoded public keys, one of which
  /// corresponds to the key used to digitally sign the JWS.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7515#section-4.1.2)
  #[serde(skip_serializing_if = "Option::is_none")]
  jku: Option<Url>,
  /// JSON Web Key.
  ///
  /// The public key that corresponds to the key used to digitally sign the JWS.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7515#section-4.1.3)
  #[serde(skip_serializing_if = "Option::is_none")]
  jwk: Option<Jwk>,
  /// Key ID.
  ///
  /// A hint indicating which key was used to secure the JWS.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7515#section-4.1.4)
  #[serde(skip_serializing_if = "Option::is_none")]
  kid: Option<String>,
  /// X.509 URL.
  ///
  /// A URI that refers to a resource for the X.509 public key certificate or
  /// certificate chain corresponding to the key used to digitally sign the JWS.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7515#section-4.1.5)
  #[serde(skip_serializing_if = "Option::is_none")]
  x5u: Option<Url>,
  /// X.509 Certificate Chain.
  ///
  /// Contains the X.509 public key certificate or certificate chain
  /// corresponding to the key used to digitally sign the JWS.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7515#section-4.1.6)
  #[serde(skip_serializing_if = "Option::is_none")]
  x5c: Option<Vec<String>>,
  /// X.509 Certificate SHA-1 Thumbprint.
  ///
  /// A base64url-encoded SHA-1 thumbprint of the DER encoding of the X.509
  /// certificate corresponding to the key used to digitally sign the JWS.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7515#section-4.1.7)
  #[serde(skip_serializing_if = "Option::is_none")]
  x5t: Option<String>,
  /// X.509 Certificate SHA-256 Thumbprint.
  ///
  /// A base64url-encoded SHA-256 thumbprint of the DER encoding of the X.509
  /// certificate corresponding to the key used to digitally sign the JWS.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7515#section-4.1.8)
  #[serde(rename = "x5t#S256", skip_serializing_if = "Option::is_none")]
  x5t_s256: Option<String>,
  /// Type.
  ///
  /// Used by JWS applications to declare the media type of this complete JWS.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7515#section-4.1.9)
  #[serde(skip_serializing_if = "Option::is_none")]
  typ: Option<String>,
  /// Content Type.
  ///
  /// Used by JWS applications to declare the media type of the secured content.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7515#section-4.1.10)
  #[serde(skip_serializing_if = "Option::is_none")]
  cty: Option<String>,
  /// Critical.
  ///
  /// Indicates that JWS extensions are being used that MUST be understood and
  /// processed.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7515#section-4.1.11)
  #[serde(skip_serializing_if = "Option::is_none")]
  crit: Option<Vec<String>>,
  /// Base64url-Encode Payload.
  ///
  /// Determines whether the payload is represented in the JWS and the JWS
  /// signing input as ASCII(BASE64URL(JWS Payload)) or as the JWS Payload
  /// value itself with no encoding performed.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7797#section-3)
  ///
  /// The following table shows the JWS Signing Input computation, depending
  /// upon the value of this parameter:
  ///
  /// +-------+-----------------------------------------------------------+
  /// | "b64" | JWS Signing Input Formula                                 |
  /// +-------+-----------------------------------------------------------+
  /// | true  | ASCII(BASE64URL(UTF8(JWS Protected Header)) || '.' ||     |
  /// |       | BASE64URL(JWS Payload))                                   |
  /// |       |                                                           |
  /// | false | ASCII(BASE64URL(UTF8(JWS Protected Header)) || '.') ||    |
  /// |       | JWS Payload                                               |
  /// +-------+-----------------------------------------------------------+
  #[serde(skip_serializing_if = "Option::is_none")]
  b64: Option<bool>,
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
  nonce: Option<String>,
  /// PASSporT extension identifier.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc8225#section-8.1)
  #[serde(skip_serializing_if = "Option::is_none")]
  ppt: Option<String>,
  /// Public/Private Claim Names
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7515#section-4.2)
  #[serde(flatten, skip_serializing_if = "Option::is_none")]
  custom: Option<T>,
}

impl<T> JwsHeader<T> {
  /// Create a new `JwsHeader`.
  pub const fn new() -> Self {
    Self::with_alg(JwsAlgorithm::NONE)
  }

  /// Create a new `JwsHeader` with the given `alg` claim.
  pub const fn with_alg(alg: JwsAlgorithm) -> Self {
    Self {
      alg,
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
      b64: None,
      url: None,
      nonce: None,
      ppt: None,
      custom: None,
    }
  }

  /// Returns the value for the algorithm claim (alg).
  pub fn alg(&self) -> JwsAlgorithm {
    self.alg
  }

  /// Sets a value for the algorithm claim (alg).
  pub fn set_alg(&mut self, value: impl Into<JwsAlgorithm>) {
    self.alg = value.into();
  }

  /// Returns the value of the JWK Set URL claim (jku).
  pub fn jku(&self) -> Option<&Url> {
    self.jku.as_ref()
  }

  /// Sets a value for the JWK Set URL claim (jku).
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
  pub fn x5c(&self) -> Option<&[String]> {
    self.x5c.as_deref()
  }

  /// Sets values for the X.509 certificate chain claim (x5c).
  pub fn set_x5c(&mut self, value: impl IntoIterator<Item = impl Into<String>>) {
    self.x5c = Some(value.into_iter().map(Into::into).collect());
  }

  /// Returns the value of the X.509 certificate SHA-1 thumbprint claim (x5t).
  pub fn x5t(&self) -> Option<&str> {
    self.x5t.as_deref()
  }

  /// Sets a value for the X.509 certificate SHA-1 thumbprint claim (x5t).
  pub fn set_x5t(&mut self, value: impl Into<String>) {
    self.x5t = Some(value.into());
  }

  /// Returns the value of the X.509 certificate SHA-256 thumbprint claim
  /// (x5t#S256).
  pub fn x5t_s256(&self) -> Option<&str> {
    self.x5t_s256.as_deref()
  }

  /// Sets a value for the X.509 certificate SHA-256 thumbprint claim
  /// (x5t#S256).
  pub fn set_x5t_s256(&mut self, value: impl Into<String>) {
    self.x5t_s256 = Some(value.into());
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

  /// Returns the value of the base64url-encode payload claim (b64).
  pub fn b64(&self) -> Option<bool> {
    self.b64
  }

  /// Sets a value for the base64url-encode payload claim (b64).
  pub fn set_b64(&mut self, value: impl Into<bool>) {
    self.b64 = Some(value.into());
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
  pub fn nonce(&self) -> Option<&str> {
    self.nonce.as_deref()
  }

  /// Sets a value for the nonce claim (nonce).
  pub fn set_nonce(&mut self, value: impl Into<String>) {
    self.nonce = Some(value.into());
  }

  /// Returns the value of the passport extension claim (ppt).
  pub fn ppt(&self) -> Option<&str> {
    self.ppt.as_deref()
  }

  /// Sets a value for the passport extension claim (ppt).
  pub fn set_ppt(&mut self, value: impl Into<String>) {
    self.ppt = Some(value.into());
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
