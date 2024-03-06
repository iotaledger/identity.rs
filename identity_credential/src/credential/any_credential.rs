use std::error::Error as StdError;
use std::str::FromStr;

use identity_core::convert::FromJson;
use serde::{Deserialize, Serialize};

#[cfg(feature = "sd-jwt")]
use super::sd_jwt::SdJwtCredential;
use super::vc2_0::Vc2_0;
use super::{Credential as Vc1_1, Jwt, JwtCredential, JwtCredentialClaims};

#[derive(Debug, thiserror::Error)]
#[error("Failed to parse into any credential")]
pub struct Error(#[source] Box<dyn StdError + Send + Sync + 'static>);

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AnyCredentialModel {
  Vc1_1(Vc1_1),
  Vc2_0(Vc2_0),
}

impl<'a> TryFrom<&'a JwtCredentialClaims> for AnyCredentialModel {
  type Error = (); // TODO: proper error
  fn try_from(value: &'a JwtCredentialClaims) -> Result<Self, Self::Error> {
    Vc1_1::try_from(value)
      .map(Self::Vc1_1)
      .map_err(|_| ())
      .or(Vc2_0::try_from(value).map(Self::Vc2_0).map_err(|_| ()))
  }
}

#[derive(Debug)]
pub enum AnyCredential {
  Plaintext(AnyCredentialModel),
  Jwt(JwtCredential<AnyCredentialModel>),
  #[cfg(feature = "sd-jwt")]
  SdJwt(SdJwtCredential<AnyCredentialModel>),
}

impl AnyCredential {
  pub fn parse_plaintext(s: &str) -> Result<Self, Error> {
    AnyCredentialModel::from_json(s)
      .map(Self::Plaintext)
      .map_err(|e| Error(e.into()))
  }
  pub fn parse_jwt(s: &str) -> Result<Self, Error> {
    s.parse::<Jwt>()
      .map_err(|e| Error(e.into()))
      .and_then(|jwt| JwtCredential::try_from(jwt).map_err(|e| Error(e.into())))
      .map(Self::Jwt)
  }
  #[cfg(feature = "sd-jwt")]
  pub fn parse_sd_jwt(s: &str) -> Result<Self, Error> {
    use sd_jwt_payload::SdJwt;

    SdJwt::parse(s)
      .map_err(|e| Error(e.into()))
      .and_then(|sd_jwt| SdJwtCredential::try_from(sd_jwt).map_err(|_| todo!("error handling")))
      .map(Self::SdJwt)
  }
}

impl FromStr for AnyCredential {
  type Err = Error;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let sd_jwt = if cfg!(feature = "sd-jwt") {
      AnyCredential::parse_sd_jwt(s)
    } else {
      todo!("proper error handling")
    };
    sd_jwt
      .or(AnyCredential::parse_plaintext(s))
      .or(AnyCredential::parse_jwt(s))
  }
}

#[cfg(test)]
mod tests {
  use crate::credential::{AnyCredential, AnyCredentialModel};

  const JSON1: &str = include_str!("../../tests/fixtures/credential-1.json");
  const JWT_VC1_1: &str = "eyJraWQiOiJkaWQ6aW90YTpzbmQ6MHhhY2Q0MTQzYjVjNzg5NWUzMDRlNjQyYTEyNWQwOWFlNTNlMjNiY2U3NWZmMGYwZGFiNzNiY2FmYjZjYmUxMjAxI2dJT1ZPSHlQM25BN3c4Yl9NMTFhcVVYaXBqSnc0ZzVtWnF0ZlJKa1IzWFUiLCJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSJ9.eyJpc3MiOiJkaWQ6aW90YTpzbmQ6MHhhY2Q0MTQzYjVjNzg5NWUzMDRlNjQyYTEyNWQwOWFlNTNlMjNiY2U3NWZmMGYwZGFiNzNiY2FmYjZjYmUxMjAxIiwibmJmIjoxNzA5NzI4NDA3LCJqdGkiOiJodHRwczovL2V4YW1wbGUuZWR1L2NyZWRlbnRpYWxzLzM3MzIiLCJzdWIiOiJkaWQ6aW90YTpzbmQ6MHg5YzVhMGQyMTUxOWYxMjhlZDAwOTNiNDBiMjVhM2ZjMWFhOGNjZDQ3ZTA1ZDczMjlkY2Q1M2I2ZWY5OTAwZGM1IiwidmMiOnsiQGNvbnRleHQiOiJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiLCJVbml2ZXJzaXR5RGVncmVlQ3JlZGVudGlhbCJdLCJjcmVkZW50aWFsU3ViamVjdCI6eyJHUEEiOiI0LjAiLCJkZWdyZWUiOnsibmFtZSI6IkJhY2hlbG9yIG9mIFNjaWVuY2UgYW5kIEFydHMiLCJ0eXBlIjoiQmFjaGVsb3JEZWdyZWUifSwibmFtZSI6IkFsaWNlIn19fQ.8URsHoPW6xl1ic66Vq5iUEG5s-IVuQvFilR_olgeuip-0L2_myATHmrk1iBvPLtZvCyjChzzXq1pe9e0qYv5DA";

  #[test]
  fn vc1_1_deserialization() {
    let cred = JSON1.parse::<AnyCredential>().unwrap();
    assert!(matches!(cred, AnyCredential::Plaintext(AnyCredentialModel::Vc1_1(_))));
  }
  #[test]
  fn jwt_vc1_1_deserialization() {
    let cred = JWT_VC1_1.parse::<AnyCredential>().unwrap();
    let AnyCredential::Jwt(jwt) = cred else { panic!("WOOT") };
    assert!(matches!(jwt.as_ref(), &AnyCredentialModel::Vc1_1(_)));
  }
}
