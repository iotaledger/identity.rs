use std::fmt::Debug;
use std::fmt::Display;
use std::str::FromStr;

use identity_jose::jwk::Jwk;
use identity_jose::jwu::decode_b64_json;

use crate::CoreDID;
use crate::Error;
use crate::DID;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Deserialize, serde::Serialize)]
#[repr(transparent)]
#[serde(into = "CoreDID", try_from = "CoreDID")]
/// A type representing a `did:jwk` DID.
pub struct DIDJwk(CoreDID);

impl DIDJwk {
  /// [`DIDKey`]'s method.
  pub const METHOD: &'static str = "jwk";

  /// Tries to parse a [`DIDKey`] from a string.
  pub fn parse(s: &str) -> Result<Self, Error> {
    s.parse()
  }

  /// Returns the JWK encoded inside this did:jwk.
  pub fn jwk(&self) -> Jwk {
    decode_b64_json(self.method_id()).expect("did:jwk encodes a valid JWK")
  }
}

impl AsRef<CoreDID> for DIDJwk {
  fn as_ref(&self) -> &CoreDID {
    &self.0
  }
}

impl From<DIDJwk> for CoreDID {
  fn from(value: DIDJwk) -> Self {
    value.0
  }
}

impl<'a> TryFrom<&'a str> for DIDJwk {
  type Error = Error;
  fn try_from(value: &'a str) -> Result<Self, Self::Error> {
    value.parse()
  }
}

impl Display for DIDJwk {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl FromStr for DIDJwk {
  type Err = Error;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    s.parse::<CoreDID>().and_then(TryFrom::try_from)
  }
}

impl From<DIDJwk> for String {
  fn from(value: DIDJwk) -> Self {
    value.to_string()
  }
}

impl TryFrom<CoreDID> for DIDJwk {
  type Error = Error;
  fn try_from(value: CoreDID) -> Result<Self, Self::Error> {
    let Self::METHOD = value.method() else {
      return Err(Error::InvalidMethodName);
    };
    decode_b64_json::<Jwk>(value.method_id())
      .map(|_| Self(value))
      .map_err(|_| Error::InvalidMethodId)
  }
}

#[cfg(test)]
mod tests {
  use identity_core::convert::FromJson;

  use super::*;

  #[test]
  fn test_valid_deserialization() -> Result<(), Error> {
    "did:jwk:eyJrdHkiOiJPS1AiLCJjcnYiOiJYMjU1MTkiLCJ1c2UiOiJlbmMiLCJ4IjoiM3A3YmZYdDl3YlRUVzJIQzdPUTFOei1EUThoYmVHZE5yZngtRkctSUswOCJ9".parse::<DIDJwk>()?;
    "did:jwk:eyJjcnYiOiJQLTI1NiIsImt0eSI6IkVDIiwieCI6ImFjYklRaXVNczNpOF91c3pFakoydHBUdFJNNEVVM3l6OTFQSDZDZEgyVjAiLCJ5IjoiX0tjeUxqOXZXTXB0bm1LdG00NkdxRHo4d2Y3NEk1TEtncmwyR3pIM25TRSJ9".parse::<DIDJwk>()?;

    Ok(())
  }

  #[test]
  fn test_jwk() {
    let did = DIDJwk::parse("did:jwk:eyJrdHkiOiJPS1AiLCJjcnYiOiJYMjU1MTkiLCJ1c2UiOiJlbmMiLCJ4IjoiM3A3YmZYdDl3YlRUVzJIQzdPUTFOei1EUThoYmVHZE5yZngtRkctSUswOCJ9").unwrap();
    let target_jwk = Jwk::from_json_value(serde_json::json!({
      "kty":"OKP","crv":"X25519","use":"enc","x":"3p7bfXt9wbTTW2HC7OQ1Nz-DQ8hbeGdNrfx-FG-IK08"
    }))
    .unwrap();

    assert_eq!(did.jwk(), target_jwk);
  }

  #[test]
  fn test_invalid_deserialization() {
    assert!(
      "did:iota:0xf4d6f08f5a1b80dd578da7dc1b49c886d580acd4cf7d48119dfeb82b538ad88a"
        .parse::<DIDJwk>()
        .is_err()
    );
    assert!("did:jwk:".parse::<DIDJwk>().is_err());
    assert!("did:jwk:z6MkiTBz1ymuepAQ4HEHYSF1H8quG5GLVVQR3djdX3mDooWp"
      .parse::<DIDJwk>()
      .is_err());
  }
}
