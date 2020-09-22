use core::convert::TryFrom;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::from_slice;
use serde_json::to_vec;

use crate::error::Error;
use crate::error::Result;
use crate::jws::Decoder;
use crate::jws::Encoder;
use crate::jws::JwsHeader;
use crate::jws::JwsSigner;
use crate::jws::JwsVerifier;
use crate::jwt::JwtClaims;
use crate::utils::Empty;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct JwsRawToken<T = Empty> {
  pub header: JwsHeader<T>,
  pub claims: Vec<u8>,
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct JwsToken<T = Empty, U = Empty> {
  header: JwsHeader<T>,
  claims: JwtClaims<U>,
}

impl<T, U> JwsToken<T, U> {
  pub const fn new(header: JwsHeader<T>, claims: JwtClaims<U>) -> Self {
    Self { header, claims }
  }

  pub const fn header(&self) -> &JwsHeader<T> {
    &self.header
  }

  pub fn header_mut(&mut self) -> &mut JwsHeader<T> {
    &mut self.header
  }

  pub const fn claims(&self) -> &JwtClaims<U> {
    &self.claims
  }

  pub fn claims_mut(&mut self) -> &mut JwtClaims<U> {
    &mut self.claims
  }

  pub fn encode_compact(&self, signer: &dyn JwsSigner) -> Result<String>
  where
    T: Serialize,
    U: Serialize,
  {
    Encoder::encode_compact(&to_vec(&self.claims)?, &self.header, signer)
  }

  pub fn decode_compact(data: impl AsRef<[u8]>, verifier: &dyn JwsVerifier) -> Result<Self>
  where
    T: DeserializeOwned,
    U: DeserializeOwned,
  {
    Decoder::decode_compact(data, verifier).and_then(Self::try_from)
  }
}

impl<T, U> TryFrom<JwsRawToken<T>> for JwsToken<T, U>
where
  U: DeserializeOwned,
{
  type Error = Error;

  fn try_from(other: JwsRawToken<T>) -> Result<Self, Self::Error> {
    Ok(Self {
      header: other.header,
      claims: from_slice(&other.claims)?,
    })
  }
}
