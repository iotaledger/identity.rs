// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Display;
use core::fmt::Formatter;

use identity_core::convert::ToJson;
use once_cell::sync::Lazy;
use serde::Deserialize;
use serde::Serialize;

use identity_core::common::Context;
use identity_core::common::Object;
use identity_core::common::OneOrMany;
use identity_core::common::Timestamp;
use identity_core::common::Url;
use identity_core::convert::FmtJson;

use crate::credential::CredentialBuilder;
use crate::credential::Evidence;
use crate::credential::Issuer;
use crate::credential::Policy;
use crate::credential::RefreshService;
use crate::credential::Schema;
use crate::credential::Status;
use crate::credential::Subject;
use crate::error::Error;
use crate::error::Result;

use super::jwt_serialization::CredentialJwtClaims;
use super::Proof;

static BASE_CONTEXT: Lazy<Context> =
  Lazy::new(|| Context::Url(Url::parse("https://www.w3.org/2018/credentials/v1").unwrap()));

/// Represents a set of claims describing an entity.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Credential<T = Object> {
  /// The JSON-LD context(s) applicable to the `Credential`.
  #[serde(rename = "@context")]
  pub context: OneOrMany<Context>,
  /// A unique `URI` that may be used to identify the `Credential`.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub id: Option<Url>,
  /// One or more URIs defining the type of the `Credential`.
  #[serde(rename = "type")]
  pub types: OneOrMany<String>,
  /// One or more `Object`s representing the `Credential` subject(s).
  #[serde(rename = "credentialSubject")]
  pub credential_subject: OneOrMany<Subject>,
  /// A reference to the issuer of the `Credential`.
  pub issuer: Issuer,
  /// A timestamp of when the `Credential` becomes valid.
  #[serde(rename = "issuanceDate")]
  pub issuance_date: Timestamp,
  /// A timestamp of when the `Credential` should no longer be considered valid.
  #[serde(rename = "expirationDate", skip_serializing_if = "Option::is_none")]
  pub expiration_date: Option<Timestamp>,
  /// Information used to determine the current status of the `Credential`.
  #[serde(default, rename = "credentialStatus", skip_serializing_if = "Option::is_none")]
  pub credential_status: Option<Status>,
  /// Information used to assist in the enforcement of a specific `Credential` structure.
  #[serde(default, rename = "credentialSchema", skip_serializing_if = "OneOrMany::is_empty")]
  pub credential_schema: OneOrMany<Schema>,
  /// Service(s) used to refresh an expired `Credential`.
  #[serde(default, rename = "refreshService", skip_serializing_if = "OneOrMany::is_empty")]
  pub refresh_service: OneOrMany<RefreshService>,
  /// Terms-of-use specified by the `Credential` issuer.
  #[serde(default, rename = "termsOfUse", skip_serializing_if = "OneOrMany::is_empty")]
  pub terms_of_use: OneOrMany<Policy>,
  /// Human-readable evidence used to support the claims within the `Credential`.
  #[serde(default, skip_serializing_if = "OneOrMany::is_empty")]
  pub evidence: OneOrMany<Evidence>,
  /// Indicates that the `Credential` must only be contained within a
  /// [`Presentation`][crate::presentation::Presentation] with a proof issued from the `Credential` subject.
  #[serde(rename = "nonTransferable", skip_serializing_if = "Option::is_none")]
  pub non_transferable: Option<bool>,
  /// Miscellaneous properties.
  #[serde(flatten)]
  pub properties: T,
  /// Optional cryptographic proof, unrelated to JWT.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub proof: Option<Proof>,
}

impl<T> Credential<T> {
  /// Returns the base JSON-LD context.
  pub fn base_context() -> &'static Context {
    &BASE_CONTEXT
  }

  /// Returns the base type.
  pub const fn base_type() -> &'static str {
    "VerifiableCredential"
  }

  /// Creates a new `CredentialBuilder` to configure a `Credential`.
  ///
  /// This is the same as [CredentialBuilder::new].
  pub fn builder(properties: T) -> CredentialBuilder<T> {
    CredentialBuilder::new(properties)
  }

  /// Returns a new `Credential` based on the `CredentialBuilder` configuration.
  pub fn from_builder(builder: CredentialBuilder<T>) -> Result<Self> {
    let this: Self = Self {
      context: builder.context.into(),
      id: builder.id,
      types: builder.types.into(),
      credential_subject: builder.subject.into(),
      issuer: builder.issuer.ok_or(Error::MissingIssuer)?,
      issuance_date: builder.issuance_date.unwrap_or_default(),
      expiration_date: builder.expiration_date,
      credential_status: builder.status,
      credential_schema: builder.schema.into(),
      refresh_service: builder.refresh_service.into(),
      terms_of_use: builder.terms_of_use.into(),
      evidence: builder.evidence.into(),
      non_transferable: builder.non_transferable,
      properties: builder.properties,
      proof: builder.proof,
    };

    this.check_structure()?;

    Ok(this)
  }

  /// Validates the semantic structure of the `Credential`.
  pub fn check_structure(&self) -> Result<()> {
    // Ensure the base context is present and in the correct location
    match self.context.get(0) {
      Some(context) if context == Self::base_context() => {}
      Some(_) | None => return Err(Error::MissingBaseContext),
    }

    // The set of types MUST contain the base type
    if !self.types.iter().any(|type_| type_ == Self::base_type()) {
      return Err(Error::MissingBaseType);
    }

    // Credentials MUST have at least one subject
    if self.credential_subject.is_empty() {
      return Err(Error::MissingSubject);
    }

    // Each subject is defined as one or more properties - no empty objects
    for subject in self.credential_subject.iter() {
      if subject.id.is_none() && subject.properties.is_empty() {
        return Err(Error::InvalidSubject);
      }
    }

    Ok(())
  }

  /// Sets the proof property of the `Credential`.
  ///
  /// Note that this proof is not related to JWT.
  pub fn set_proof(&mut self, proof: Option<Proof>) {
    self.proof = proof;
  }

  /// Serializes the [`Credential`] as a JWT claims set
  /// in accordance with [VC Data Model v1.1](https://www.w3.org/TR/vc-data-model/#json-web-token).
  ///
  /// The resulting string can be used as the payload of a JWS when issuing the credential.  
  pub fn serialize_jwt(&self, custom_claims: Option<Object>) -> Result<String>
  where
    T: ToOwned<Owned = T> + serde::Serialize + serde::de::DeserializeOwned,
  {
    let jwt_representation: CredentialJwtClaims<'_, T> = CredentialJwtClaims::new(self, custom_claims)?;
    jwt_representation
      .to_json()
      .map_err(|err| Error::JwtClaimsSetSerializationError(err.into()))
  }
}

impl<T> Display for Credential<T>
where
  T: Serialize,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    self.fmt_json(f)
  }
}

#[cfg(test)]
mod tests {
  use identity_core::convert::FromJson;

  use crate::credential::Credential;

  const JSON1: &str = include_str!("../../tests/fixtures/credential-1.json");
  const JSON2: &str = include_str!("../../tests/fixtures/credential-2.json");
  const JSON3: &str = include_str!("../../tests/fixtures/credential-3.json");
  const JSON4: &str = include_str!("../../tests/fixtures/credential-4.json");
  const JSON5: &str = include_str!("../../tests/fixtures/credential-5.json");
  const JSON6: &str = include_str!("../../tests/fixtures/credential-6.json");
  const JSON7: &str = include_str!("../../tests/fixtures/credential-7.json");
  const JSON8: &str = include_str!("../../tests/fixtures/credential-8.json");
  const JSON9: &str = include_str!("../../tests/fixtures/credential-9.json");
  const JSON10: &str = include_str!("../../tests/fixtures/credential-10.json");
  const JSON11: &str = include_str!("../../tests/fixtures/credential-11.json");
  const JSON12: &str = include_str!("../../tests/fixtures/credential-12.json");

  #[test]
  fn test_from_json() {
    let _credential: Credential = Credential::from_json(JSON1).unwrap();
    let _credential: Credential = Credential::from_json(JSON2).unwrap();
    let _credential: Credential = Credential::from_json(JSON3).unwrap();
    let _credential: Credential = Credential::from_json(JSON4).unwrap();
    let _credential: Credential = Credential::from_json(JSON5).unwrap();
    let _credential: Credential = Credential::from_json(JSON6).unwrap();
    let _credential: Credential = Credential::from_json(JSON7).unwrap();
    let _credential: Credential = Credential::from_json(JSON8).unwrap();
    let _credential: Credential = Credential::from_json(JSON9).unwrap();
    let _credential: Credential = Credential::from_json(JSON10).unwrap();
    let _credential: Credential = Credential::from_json(JSON11).unwrap();
    let _credential: Credential = Credential::from_json(JSON12).unwrap();
  }
}
