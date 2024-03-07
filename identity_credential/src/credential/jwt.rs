// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// TODO:
// Jwt should be decoded using the facilities we already have in the library (identity_jose::jws::decoder).
// Decoder gives us all we need. We need to unpack it into a JWS (alg, sig_input, sig_challenge) and claims.
// The claims will be parsed into a JwtCredentialClaims.

use std::fmt::Debug;
use std::fmt::Display;
use std::ops::Deref;
use std::str::FromStr;

use identity_core::common::Object;
use identity_core::common::Timestamp;
use identity_core::common::Url;
use identity_jose::error::Error as JoseError;
use identity_jose::jws::DecodedHeaders;
use identity_jose::jws::Decoder as JwsDecoder;
use identity_verification::ProofT;
use identity_verification::VerifierT;
use serde::Deserialize;
use serde::Serialize;
use serde_with::DeserializeFromStr;
use serde_with::SerializeDisplay;
use thiserror::Error;

use crate::revocation::StatusCredentialT;

use super::CredentialT;
use super::Issuer;
use identity_core::ResolverT;
use super::ValidableCredential;

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

impl DecodedJws {
  pub fn claims(&self) -> &[u8] {
    &self.raw_claims
  }
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
  pub(crate) inner: String,
  pub(crate) decoded_jws: DecodedJws,
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
  #[error("Failed to parse packed credential")]
  CredentialUnpackingError,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(transparent)]
#[serde(try_from = "i64", into = "i64")]
pub(crate) struct UnixTimestampWrapper(Timestamp);

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
  #[serde(flatten)]
  issuance_date: IssuanceDate,
  /// Represents the id property of the credential.
  pub jti: Option<Url>,
  /// Represents the subject's id.
  pub sub: Option<Url>,
  pub vc: Object,
  #[serde(flatten, skip_serializing_if = "Option::is_none")]
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

pub struct JwtCredential<C> {
  pub(crate) inner: String,
  pub(crate) credential: C,
  pub(crate) parsed_claims: JwtCredentialClaims,
  pub(crate) decoded_jws: DecodedJws,
}

impl<C: Debug> Debug for JwtCredential<C> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("JwtCredential")
      .field("inner", &self.inner.as_str())
      .field("credential", &self.credential)
      .field("parsed_claims", &self.parsed_claims)
      .field("decoded_jws", &self.decoded_jws)
      .finish()
  }
}

impl<C: Clone> Clone for JwtCredential<C> {
  fn clone(&self) -> Self {
    let JwtCredential {
      inner,
      credential,
      parsed_claims,
      decoded_jws,
    } = self;
    Self {
      inner: inner.clone(),
      credential: credential.clone(),
      parsed_claims: parsed_claims.clone(),
      decoded_jws: decoded_jws.clone(),
    }
  }
}

impl<C> serde::Serialize for JwtCredential<C> {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    self.inner.as_str().serialize(serializer)
  }
}

impl<'de, C> serde::Deserialize<'de> for JwtCredential<C>
where
  C: for<'a> TryFrom<&'a JwtCredentialClaims>,
{
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    use serde::de::Error;

    let jwt = Jwt::deserialize(deserializer)?;
    Self::try_from(jwt).map_err(|e| D::Error::custom(e))
  }
}

impl<C> TryFrom<Jwt> for JwtCredential<C>
where
  C: for<'a> TryFrom<&'a JwtCredentialClaims>,
{
  type Error = JwtCredentialError;

  fn try_from(jwt: Jwt) -> Result<Self, Self::Error> {
    let Jwt { inner, decoded_jws } = jwt;
    let parsed_claims = serde_json::from_slice(&decoded_jws.raw_claims)?;
    let credential = C::try_from(&parsed_claims).map_err(|_| JwtCredentialError::CredentialUnpackingError)?;
    Ok(Self {
      inner,
      decoded_jws,
      parsed_claims,
      credential,
    })
  }
}

impl<C> From<JwtCredential<C>> for Jwt
where
  C: Into<JwtCredentialClaims>,
{
  fn from(value: JwtCredential<C>) -> Self {
    Jwt {
      inner: value.inner,
      decoded_jws: value.decoded_jws,
    }
  }
}

impl<C> CredentialT for JwtCredential<C> {
  type Claim = JwtCredentialClaims;
  type Issuer = Issuer;
  type Id = Option<Url>;

  fn id(&self) -> &Self::Id {
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

impl<C> JwtCredential<C>
where
  C: for<'a> TryFrom<&'a JwtCredentialClaims>,
{
  pub fn parse(jwt: Jwt) -> Result<Self, JwtCredentialError> {
    Self::try_from(jwt)
  }
}

impl<C: StatusCredentialT> StatusCredentialT for JwtCredential<C> {
  type Status = C::Status;
  fn status(&self) -> Option<&Self::Status> {
    self.credential.status()
  }
}

impl<R, V, C, K> ValidableCredential<R, V, K> for JwtCredential<C>
where
  R: ResolverT<K>,
  R::Input: TryFrom<Url>,
  V: VerifierT<K>,
{
  async fn validate(&self, resolver: &R, verifier: &V) -> Result<(), ()> {
    if !self.check_validity_time_frame() {
      todo!("expired credential err");
    }
    let kid = self
      .decoded_jws
      .verification_method()
      .ok_or(())
      .and_then(|kid| R::Input::try_from(kid).map_err(|_| ()))?;
    let key = resolver.fetch(&kid).await.map_err(|_| ())?;
    verifier.verify(&self.decoded_jws, &key).map_err(|_| ())?;

    Ok(())
  }
}

impl<C> AsRef<C> for JwtCredential<C> {
  fn as_ref(&self) -> &C {
    &self.credential 
  }
}