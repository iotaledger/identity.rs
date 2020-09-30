/// Possible values of the JOSE "typ" header parameter
///
/// [More Info](https://tools.ietf.org/html/rfc7519#section-5.1)
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum JoseTokenType {
  /// Indicates that the token is a JSON Web Token.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7519#section-5.1)
  JWT,
  /// Indicates the token is a JSON Web Message.
  ///
  /// [More Info](https://tools.ietf.org/id/draft-looker-jwm-01.html#rfc.section.4.1)
  JWM,
  /// Indicates that the token is a JWE/JWS using compact serialization.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7515#section-4.1.9)
  JOSE,
  /// Indicates that the token is a JWE/JWS using JSON serialization.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7515#section-4.1.9)
  #[serde(rename = "JOSE+JSON")]
  JOSE_JSON,
}
