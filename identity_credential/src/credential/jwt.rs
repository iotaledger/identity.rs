// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// TODO:
// Jwt should be decoded using the facilities we already have in the library (identity_jose::jws::decoder).
// Decoder gives us all we need. We need to unpack it into a JWS (alg, sig_input, sig_challenge) and claims.
// The claims will be parsed into a JwtCredentialClaims.

use std::fmt::Display;
use std::ops::Deref;
use std::str::FromStr;

use identity_core::common::Object;
use identity_core::common::Timestamp;
use identity_core::common::Url;
use identity_jose::error::Error as JoseError;
use identity_jose::jws::DecodedHeaders;
use identity_jose::jws::Decoder as JwsDecoder;
use serde::Deserialize;
use serde::Serialize;
use serde_with::DeserializeFromStr;
use serde_with::SerializeDisplay;
use thiserror::Error;

use super::CredentialT;
use super::Issuer;
use super::ProofT;
use super::VerifiableCredentialT;

#[derive(Error, Debug)]
pub enum JwtError {
  #[error(transparent)]
  DecodingError(#[from] JoseError),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecodedJws {
  headers: DecodedHeaders,
  signing_input: Box<[u8]>,
  raw_claims: Box<[u8]>,
  signature: Box<[u8]>,
}

impl ProofT for DecodedJws {
  type VerificationMethod = Option<Url>;

  fn algorithm(&self) -> &str {
    self
      .headers
      .protected_header()
      .and_then(|header| header.alg())
      .unwrap_or(identity_jose::jws::JwsAlgorithm::NONE)
      .name()
  }
  fn signature(&self) -> &[u8] {
    self.signature.as_ref()
  }
  fn signing_input(&self) -> &[u8] {
    self.signing_input.as_ref()
  }
  fn verification_method(&self) -> Self::VerificationMethod {
    self
      .headers
      .protected_header()
      .and_then(|header| header.kid())
      .and_then(|kid| Url::parse(kid).ok())
  }
}

#[derive(Debug, Clone, PartialEq, Eq, SerializeDisplay, DeserializeFromStr)]
pub struct Jwt {
  inner: String,
  decoded_jws: DecodedJws,
}

impl FromStr for Jwt {
  type Err = JwtError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Self::parse(s.to_owned())
  }
}

impl Display for Jwt {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", &self.inner)
  }
}

impl Jwt {
  pub fn parse(token: String) -> Result<Self, JwtError> {
    let decoder = JwsDecoder::new();
    let decoded_jws = decoder.decode_compact_serialization(token.as_bytes(), None)?;
    let (headers, signing_input, signature, raw_claims) = decoded_jws.into_parts();
    let decoded_jws = DecodedJws {
      headers,
      signing_input,
      raw_claims,
      signature,
    };

    Ok(Self {
      inner: token,
      decoded_jws,
    })
  }
  pub fn as_str(&self) -> &str {
    self.inner.as_str()
  }
}

#[derive(Debug, Error)]
pub enum JwtCredentialError {
  #[error(transparent)]
  DeserializationError(#[from] serde_json::Error),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(transparent)]
#[serde(try_from = "i64", into = "i64")]
struct UnixTimestampWrapper(Timestamp);

impl Deref for UnixTimestampWrapper {
  type Target = Timestamp;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl TryFrom<i64> for UnixTimestampWrapper {
  type Error = identity_core::Error;
  fn try_from(value: i64) -> Result<Self, Self::Error> {
    Timestamp::from_unix(value).map(Self)
  }
}

impl From<UnixTimestampWrapper> for i64 {
  fn from(value: UnixTimestampWrapper) -> Self {
    value.0.to_unix()
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum IssuanceDate {
  Iat(UnixTimestampWrapper),
  Nbf(UnixTimestampWrapper),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtCredentialClaims {
  /// Represents the expirationDate encoded as a UNIX timestamp.  
  pub exp: Option<UnixTimestampWrapper>,
  /// Represents the issuer.
  pub iss: Issuer,
  /// Represents the issuanceDate encoded as a UNIX timestamp.
  issuance_date: IssuanceDate,
  /// Represents the id property of the credential.
  pub jti: Url,
  /// Represents the subject's id.
  pub sub: Option<Url>,
  pub vc: Object,
  pub custom: Option<Object>,
}

impl JwtCredentialClaims {
  pub fn issuance_date(&self) -> Timestamp {
    match self.issuance_date {
      IssuanceDate::Iat(t) => *t,
      IssuanceDate::Nbf(t) => *t,
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "Jwt", into = "Jwt")]
pub struct JwtCredential {
  inner: String,
  parsed_claims: JwtCredentialClaims,
  decoded_jws: DecodedJws,
}

impl TryFrom<Jwt> for JwtCredential {
  type Error = JwtCredentialError;

  fn try_from(jwt: Jwt) -> Result<Self, Self::Error> {
    let Jwt { inner, decoded_jws } = jwt;
    let parsed_claims = serde_json::from_slice(&decoded_jws.raw_claims)?;
    Ok(Self {
      inner,
      decoded_jws,
      parsed_claims,
    })
  }
}

impl From<JwtCredential> for Jwt {
  fn from(value: JwtCredential) -> Self {
    Jwt {
      inner: value.inner,
      decoded_jws: value.decoded_jws,
    }
  }
}

impl CredentialT for JwtCredential {
  type Claim = JwtCredentialClaims;
  type Issuer = Issuer;

  fn id(&self) -> &Url {
    &self.parsed_claims.jti
  }
  fn issuer(&self) -> &Self::Issuer {
    &self.parsed_claims.iss
  }
  fn claim(&self) -> &Self::Claim {
    &self.parsed_claims
  }
  fn valid_from(&self) -> Timestamp {
    self.parsed_claims.issuance_date()
  }
  fn valid_until(&self) -> Option<Timestamp> {
    self.parsed_claims.exp.as_deref().copied()
  }
}

impl<'c> VerifiableCredentialT<'c> for JwtCredential {
  type Proof = &'c DecodedJws;

  fn proof(&'c self) -> Self::Proof {
    &self.decoded_jws
  }
}

impl JwtCredential {
  pub fn parse(jwt: Jwt) -> Result<Self, JwtCredentialError> {
    Self::try_from(jwt)
  }
}
