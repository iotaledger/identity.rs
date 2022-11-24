// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Context;
use identity_core::common::Object;
use identity_core::common::OneOrMany;
use identity_core::common::Timestamp;
use identity_core::common::Url;
use identity_core::convert::FmtJson;
use std::fmt::Display;
use std::fmt::Formatter;

use crate::credential::Credential;
use crate::credential::Issuer;
use crate::credential::Subject;
use crate::error::Result;
use crate::Error;

static BASE_CONTEXT: &str = "https://www.w3.org/2018/credentials/v1";
static WELL_KNOWN_CONTEXT: &str = "https://identity.foundation/.well-known/did-configuration/v1";
static TYPE_1: &str = "VerifiableCredential";
static TYPE_2: &str = "DomainLinkageCredential";

/// DID Configuration Resource which contains Domain Linkage Credentials.
///
/// See: <https://identity.foundation/.well-known/resources/did-configuration/#did-configuration-resource>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DIDConfigurationResource {
  #[serde(rename = "@context")]
  context: Context,
  linked_dids: Vec<Credential>,
}

impl Display for DIDConfigurationResource {
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    self.fmt_json(f)
  }
}

impl DIDConfigurationResource {
  /// Creates a new DID Configuration Resource.
  pub fn new(linked_dids: Vec<Credential>) -> Self {
    Self {
      context: Context::Url(Url::parse(WELL_KNOWN_CONTEXT).unwrap()),
      linked_dids,
    }
  }

  /// List of domain Linkage Credentials.
  pub fn linked_dids(&self) -> &Vec<Credential> {
    &self.linked_dids
  }

  /// List of domain Linkage Credentials.
  pub fn linked_dids_mut(&mut self) -> &mut Vec<Credential> {
    &mut self.linked_dids
  }
}

/// Convenient builder to create a spec compliant Linked Data Domain Linkage Credential.
///
/// See: <https://identity.foundation/.well-known/resources/did-configuration/#linked-data-proof-format>
#[derive(Debug, Default)]
pub struct DomainLinkageCredentialBuilder {
  pub(crate) issuer: Option<Url>,
  pub(crate) issuance_date: Option<Timestamp>,
  pub(crate) expiration_date: Option<Timestamp>,
  pub(crate) origin: Option<Url>,
}

impl DomainLinkageCredentialBuilder {
  /// Creates a new `DomainLinkageCredentialBuilder`.
  pub fn new() -> Self {
    Self::default()
  }

  fn base_context() -> Context {
    Context::Url(Url::parse(BASE_CONTEXT).unwrap())
  }

  fn well_known_context() -> Context {
    Context::Url(Url::parse(WELL_KNOWN_CONTEXT).unwrap())
  }

  /// Sets the value of the `issuer`, only the URL is used, other properties are ignored.
  #[must_use]
  pub fn issuer(mut self, value: Issuer) -> Self {
    let issuer: Url = match value {
      Issuer::Url(url) => url,
      Issuer::Obj(data) => data.id,
    };
    self.issuer = Some(issuer);
    self
  }

  /// Sets the value of the `Credential` `issuanceDate`.
  #[must_use]
  pub fn issuance_date(mut self, value: Timestamp) -> Self {
    self.issuance_date = Some(value);
    self
  }

  /// Sets the value of the `Credential` `expirationDate`.
  #[must_use]
  pub fn expiration_date(mut self, value: Timestamp) -> Self {
    self.expiration_date = Some(value);
    self
  }

  /// Sets the origin in `credentialSubject`.
  #[must_use]
  pub fn origin(mut self, value: Url) -> Self {
    self.origin = Some(value);
    self
  }

  /// Returns a new `Credential` based on the `DomainLinkageCredentialBuilder` configuration.
  pub fn build(self) -> Result<Credential<Object>> {
    let origin: Url = self.origin.ok_or(Error::MissingOrigin)?;
    let mut properties: Object = Object::new();
    properties.insert("origin".into(), origin.into_string().into());
    let issuer: Url = self.issuer.ok_or(Error::MissingIssuer)?;

    Ok(Credential {
      context: OneOrMany::Many(vec![Self::base_context(), Self::well_known_context()]),
      id: None,
      types: OneOrMany::Many(vec![TYPE_1.to_owned(), TYPE_2.to_owned()]),
      credential_subject: OneOrMany::One(Subject::with_id_and_properties(issuer.clone(), properties)),
      issuer: Issuer::Url(issuer),
      issuance_date: self.issuance_date.unwrap_or_else(Timestamp::now_utc),
      expiration_date: Some(self.expiration_date.ok_or(Error::MissingExpirationDate)?),
      credential_status: None,
      credential_schema: Vec::new().into(),
      refresh_service: Vec::new().into(),
      terms_of_use: Vec::new().into(),
      evidence: Vec::new().into(),
      non_transferable: None,
      properties: Object::new(),
      proof: None,
    })
  }
}

#[cfg(test)]
mod tests {
  use crate::credential::domain_linkage_credential::DIDConfigurationResource;
  use crate::credential::domain_linkage_credential::DomainLinkageCredentialBuilder;
  use crate::credential::Credential;
  use crate::credential::Issuer;
  use crate::error::Result;
  use crate::Error;
  use identity_core::common::Timestamp;
  use identity_core::common::Url;
  use identity_core::convert::FromJson;

  #[test]
  fn builder() {
    let issuer = Issuer::Url(Url::parse("did:example:issuer").unwrap());
    let credential: Credential = DomainLinkageCredentialBuilder::new()
      .issuance_date(Timestamp::now_utc())
      .expiration_date(Timestamp::now_utc())
      .issuer(issuer)
      .origin(Url::parse("http://www.example.com").unwrap())
      .build()
      .unwrap();
    let _config: DIDConfigurationResource = DIDConfigurationResource::new(vec![credential]);
  }

  #[test]
  fn builder_no_issuer() {
    let credential: Result<Credential> = DomainLinkageCredentialBuilder::new()
      .issuance_date(Timestamp::now_utc())
      .expiration_date(Timestamp::now_utc())
      .origin(Url::parse("http://www.example.com").unwrap())
      .build();

    assert!(matches!(credential, Err(Error::MissingIssuer)));
  }

  #[test]
  fn builder_no_origin() {
    let issuer = Issuer::Url(Url::parse("did:example:issuer").unwrap());
    let credential: Result<Credential> = DomainLinkageCredentialBuilder::new()
      .issuance_date(Timestamp::now_utc())
      .expiration_date(Timestamp::now_utc())
      .issuer(issuer)
      .build();

    assert!(matches!(credential, Err(Error::MissingOrigin)));
  }

  #[test]
  fn builder_no_expiration_date() {
    let issuer = Issuer::Url(Url::parse("did:example:issuer").unwrap());
    let credential: Result<Credential> = DomainLinkageCredentialBuilder::new()
      .issuance_date(Timestamp::now_utc())
      .issuer(issuer)
      .origin(Url::parse("http://www.example.com").unwrap())
      .build();

    assert!(matches!(credential, Err(Error::MissingExpirationDate)));
  }

  #[test]
  fn from_json() {
    const JSON1: &str = include_str!("../../tests/fixtures/well-known-configuration.json");
    let _well_known: DIDConfigurationResource = DIDConfigurationResource::from_json(JSON1).unwrap();
  }
}
