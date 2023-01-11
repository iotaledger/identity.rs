// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::credential::Credential;
use crate::error::Result;

use identity_core::common::Context;
use identity_core::common::Object;

use identity_core::common::Url;
use identity_core::convert::FmtJson;

use identity_did::did::DID;

#[cfg(feature = "domain-linkage-fetch")]
use reqwest::redirect::Policy;
#[cfg(feature = "domain-linkage-fetch")]
use reqwest::Client;
use std::fmt::Display;
use std::fmt::Formatter;

use crate::Error::DomainLinkageError;

lazy_static! {
  static ref WELL_KNOWN_CONTEXT: Context =
    Context::Url(Url::parse("https://identity.foundation/.well-known/did-configuration/v1").unwrap());
}

/// DID Configuration Resource which contains Domain Linkage Credentials.
///
/// It can be placed in an origin's `.well-known` directory to prove linkage between the origin and a DID.
///
/// See: <https://identity.foundation/.well-known/resources/did-configuration/#did-configuration-resource>
///
/// Note:
/// - Only [Linked Data Proof Format](https://identity.foundation/.well-known/resources/did-configuration/#linked-data-proof-format)
///   is supported.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DomainLinkageConfiguration {
  /// Fixed context.
  #[serde(rename = "@context")]
  context: Context,
  /// Linked credentials.
  linked_dids: Vec<Credential>,
  /// Further properties, must be empty.
  #[serde(flatten)]
  pub properties: Object,
}

impl Display for DomainLinkageConfiguration {
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    self.fmt_json(f)
  }
}

impl DomainLinkageConfiguration {
  /// Creates a new DID Configuration Resource.
  pub fn new(linked_dids: Vec<Credential>) -> Self {
    Self {
      context: Self::well_known_context().clone(),
      linked_dids,
      properties: Object::new(),
    }
  }

  pub(crate) fn well_known_context() -> &'static Context {
    &WELL_KNOWN_CONTEXT
  }

  pub(crate) const fn domain_linkage_type() -> &'static str {
    "DomainLinkageCredential"
  }

  /// Validates the semantic structure.
  pub fn check_structure(&self) -> Result<()> {
    if &self.context != Self::well_known_context() {
      return Err(DomainLinkageError("invalid context".into()));
    }

    if !self.properties.is_empty() {
      return Err(DomainLinkageError(
        "domain linkage configuration contains invalid properties".into(),
      ));
    }

    if self.linked_dids.is_empty() {
      return Err(DomainLinkageError("property `linked_dids` is invalid".into()));
    }
    Ok(())
  }

  /// Fetches the the DID Configuration resource via a GET request at the
  /// well-known location: `origin`/.well-known/did-configuration.json.
  #[cfg(feature = "domain-linkage-fetch")]
  pub async fn fetch_configuration(mut origin: Url) -> Result<DomainLinkageConfiguration> {
    if origin.scheme() != "https" {
      return Err(DomainLinkageError("origin` does not use `https` protocol".into()));
    }
    let client: Client = reqwest::ClientBuilder::new()
      .redirect(Policy::none())
      .build()
      .map_err(|err| DomainLinkageError(Box::new(err)))?;

    origin.set_path(".well-known/did-configuration.json");
    let domain_linkage_configuration: DomainLinkageConfiguration = client
      .get(origin.to_string())
      .send()
      .await
      .map_err(|err| DomainLinkageError(Box::new(err)))?
      .json::<DomainLinkageConfiguration>()
      .await
      .map_err(|err| DomainLinkageError(Box::new(err)))?;
    Ok(domain_linkage_configuration)
  }

  /// List of domain Linkage Credentials.
  pub fn linked_dids(&self) -> &Vec<Credential> {
    &self.linked_dids
  }

  /// List of domain Linkage Credentials.
  pub fn linked_dids_mut(&mut self) -> &mut Vec<Credential> {
    &mut self.linked_dids
  }

  /// List of extra properties (should be empty to be spec compliant).
  pub fn properties(&self) -> &Object {
    &self.properties
  }

  /// List of extra properties (should be empty to be spec compliant).
  pub fn properties_mut(&mut self) -> &mut Object {
    &mut self.properties
  }
}

#[cfg(test)]
mod tests {
  use crate::credential::domain_linkage_configuration::DomainLinkageConfiguration;
  use identity_core::convert::FromJson;

  #[test]
  fn from_json() {
    const JSON1: &str = include_str!("../../tests/fixtures/dn-config.json");
    DomainLinkageConfiguration::from_json(JSON1).unwrap();
  }

  #[test]
  fn from_json_invalid_context() {
    const JSON1: &str = include_str!("../../tests/fixtures/dn-config-invalid-context.json");
    let config = DomainLinkageConfiguration::from_json(JSON1).unwrap();
    assert!(config.check_structure().is_err());
  }
}
