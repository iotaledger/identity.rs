// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::borrow::Cow;

use serde::Deserialize;
use serde::Serialize;

use identity_core::common::Context;
use identity_core::common::Object;
use identity_core::common::OneOrMany;
use identity_core::common::Timestamp;
use identity_core::common::Url;
use identity_core::crypto::Proof;
use serde::de::DeserializeOwned;

use crate::credential::Credential;
use crate::credential::Evidence;
use crate::credential::Issuer;
use crate::credential::Policy;
use crate::credential::RefreshService;
use crate::credential::Schema;
use crate::credential::Status;
use crate::credential::Subject;
use crate::Error;
use crate::Result;

// Implementation of JWT Encoding/Decoding according to https://w3c.github.io/vc-jwt/#version-1.1.
// Note that version 2 seems to be work in progress and meant for the VC 2.0 Credentials.
#[derive(Serialize, Deserialize)]
pub(super) struct VerifiableCredentialJwtClaims<'credential, T = Object>
where
  T: ToOwned + Serialize,
  <T as ToOwned>::Owned: DeserializeOwned,
{
  /// Represents the expirationDate encoded as a UNIX timestamp.  
  #[serde(skip_serializing_if = "Option::is_none")]
  exp: Option<i64>,
  /// Represents the issuer.
  iss: Cow<'credential, Issuer>,

  /// Represents the issuanceDate encoded as a UNIX timestamp.
  nbf: i64,

  /// Represents the id property of the credential.
  #[serde(skip_serializing_if = "Option::is_none")]
  jti: Option<Cow<'credential, Url>>,

  /// Represents the subject's id.
  #[serde(skip_serializing_if = "Option::is_none")]
  sub: Option<Cow<'credential, Url>>,

  vc: InnerCredential<'credential, T>,
}

impl<'credential, T> VerifiableCredentialJwtClaims<'credential, T>
where
  T: ToOwned + Serialize,
  <T as ToOwned>::Owned: DeserializeOwned,
{
  /// Checks whether the fields that are set in the `vc` object are consistent with the corresponding values
  /// set for the registered claims.
  fn check_consistency(&self) -> Result<()> {
    if !self
      .vc
      .issuance_date
      .map(|value| value.to_unix() == self.nbf)
      .unwrap_or(true)
    {
      return Err(Error::InconsistentCredentialJwtClaims("inconsistent issuanceDate"));
    };

    if !self
      .vc
      .expiration_date
      .map(|value| self.exp.filter(|exp| *exp == value.to_unix()).is_some())
      .unwrap_or(true)
    {
      return Err(Error::InconsistentCredentialJwtClaims(
        "inconsistent credential expirationDate",
      ));
    };

    if !self
      .vc
      .id
      .as_ref()
      .map(|value| self.jti.as_ref().filter(|jti| jti.as_ref() == value).is_some())
      .unwrap_or(true)
    {
      return Err(Error::InconsistentCredentialJwtClaims("inconsistent credential id"));
    };
    Ok(())
  }

  /// Converts the JWT representation into a [`Credential`].
  ///
  /// # Errors
  /// Errors if either timestamp conversion or [`Self::check_consistency`] fails.
  pub(crate) fn try_into_credential(self) -> Result<Credential<T::Owned>> {
    self.check_consistency()?;

    let Self {
      exp,
      iss,
      nbf,
      jti,
      sub,
      vc,
    } = self;

    let InnerCredential {
      context,
      id: _,
      types,
      credential_subject,
      credential_status,
      credential_schema,
      refresh_service,
      terms_of_use,
      evidence,
      non_transferable,
      properties,
      proof,
      issuance_date: _,
      expiration_date: _,
    } = vc;

    Ok(Credential {
      context: context.into_owned(),
      id: jti.map(Cow::into_owned),
      types: types.into_owned(),
      credential_subject: {
        OneOrMany::One(Subject {
          id: sub.map(Cow::into_owned),
          properties: credential_subject.properties.into_owned(),
        })
      },
      issuer: iss.into_owned(),
      issuance_date: Timestamp::from_unix(nbf).map_err(|_| Error::TimestampConversionError)?,
      expiration_date: exp
        .map(Timestamp::from_unix)
        .transpose()
        .map_err(|_| Error::TimestampConversionError)?,
      credential_status: credential_status.map(Cow::into_owned),
      credential_schema: credential_schema.into_owned(),
      refresh_service: refresh_service.into_owned(),
      terms_of_use: terms_of_use.into_owned(),
      evidence: evidence.into_owned(),
      non_transferable,
      properties: properties.into_owned(),
      proof: proof.map(Cow::into_owned),
    })
  }

  pub(super) fn new(credential: &'credential Credential<T>) -> Result<Self> {
    let Credential {
        context,
        id,
        types,
        credential_subject: OneOrMany::One(subject),
        issuer,
        issuance_date,
        expiration_date,
        credential_status,
        credential_schema,
        refresh_service,
        terms_of_use,
        evidence,
        non_transferable,
        properties,
        proof
        } = credential else {
            return Err(Error::MoreThanOneSubjectInJwt)
        };

    Ok(Self {
      exp: expiration_date.map(|value| Timestamp::to_unix(&value)),
      iss: Cow::Borrowed(issuer),
      nbf: issuance_date.to_unix(),
      jti: id.as_ref().map(Cow::Borrowed),
      sub: subject.id.as_ref().map(Cow::Borrowed),
      vc: InnerCredential {
        context: Cow::Borrowed(context),
        id: None,
        types: Cow::Borrowed(types),
        credential_subject: SubjectWithoutId::new(subject),
        issuance_date: None,
        expiration_date: None,
        credential_schema: Cow::Borrowed(credential_schema),
        credential_status: credential_status.as_ref().map(Cow::Borrowed),
        refresh_service: Cow::Borrowed(refresh_service),
        terms_of_use: Cow::Borrowed(terms_of_use),
        evidence: Cow::Borrowed(evidence),
        non_transferable: *non_transferable,
        properties: Cow::Borrowed(properties),
        proof: proof.as_ref().map(Cow::Borrowed),
      },
    })
  }
}

#[derive(Serialize, Deserialize)]
struct SubjectWithoutId<'credential> {
  #[serde(flatten)]
  properties: Cow<'credential, Object>,
}
impl<'credential> SubjectWithoutId<'credential> {
  fn new(subject: &'credential Subject) -> Self {
    let Subject { id: _, properties } = subject;
    Self {
      properties: Cow::Borrowed(properties),
    }
  }
}

/// Mostly copied from [`VerifiableCredential`] with the entries corresponding
/// to registered claims being the exception.
#[derive(Serialize, Deserialize)]
struct InnerCredential<'credential, T = Object>
where
  T: ToOwned + Serialize,
  <T as ToOwned>::Owned: DeserializeOwned,
{
  /// The JSON-LD context(s) applicable to the `Credential`.
  #[serde(rename = "@context")]
  context: Cow<'credential, OneOrMany<Context>>,
  /// A unique `URI` that may be used to identify the `Credential`.
  #[serde(skip_serializing_if = "Option::is_none")]
  id: Option<Url>,
  /// One or more URIs defining the type of the `Credential`.
  #[serde(rename = "type")]
  types: Cow<'credential, OneOrMany<String>>,
  /// One or more `Object`s representing the `Credential` subject(s).
  #[serde(rename = "credentialSubject")]
  credential_subject: SubjectWithoutId<'credential>,
  /// A timestamp of when the `Credential` becomes valid.
  #[serde(rename = "issuanceDate", skip_serializing_if = "Option::is_none")]
  issuance_date: Option<Timestamp>,
  /// A timestamp of when the `Credential` should no longer be considered valid.
  #[serde(rename = "expirationDate", skip_serializing_if = "Option::is_none")]
  expiration_date: Option<Timestamp>,
  /// Information used to determine the current status of the `Credential`.
  #[serde(default, rename = "credentialStatus", skip_serializing_if = "Option::is_none")]
  credential_status: Option<Cow<'credential, Status>>,
  /// Information used to assist in the enforcement of a specific `Credential` structure.
  #[serde(default, rename = "credentialSchema", skip_serializing_if = "OneOrMany::is_empty")]
  credential_schema: Cow<'credential, OneOrMany<Schema>>,
  /// Service(s) used to refresh an expired `Credential`.
  #[serde(default, rename = "refreshService", skip_serializing_if = "OneOrMany::is_empty")]
  refresh_service: Cow<'credential, OneOrMany<RefreshService>>,
  /// Terms-of-use specified by the `Credential` issuer.
  #[serde(default, rename = "termsOfUse", skip_serializing_if = "OneOrMany::is_empty")]
  terms_of_use: Cow<'credential, OneOrMany<Policy>>,
  /// Human-readable evidence used to support the claims within the `Credential`.
  #[serde(default, skip_serializing_if = "OneOrMany::is_empty")]
  evidence: Cow<'credential, OneOrMany<Evidence>>,
  /// Indicates that the `Credential` must only be contained within a
  /// [`Presentation`][crate::presentation::Presentation] with a proof issued from the `Credential` subject.
  #[serde(rename = "nonTransferable", skip_serializing_if = "Option::is_none")]
  non_transferable: Option<bool>,
  /// Miscellaneous properties.
  #[serde(flatten)]
  properties: Cow<'credential, T>,
  /// Proof(s) used to verify a `Credential`
  #[serde(skip_serializing_if = "Option::is_none")]
  proof: Option<Cow<'credential, Proof>>,
}
