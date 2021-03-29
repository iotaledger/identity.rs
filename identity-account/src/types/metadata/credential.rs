// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Url;
use identity_credential::credential::Credential;

use crate::types::ResourceType;
use crate::types::MetadataItem;
use crate::types::Identifier;
use crate::types::Timestamps;
use crate::types::Index;
use crate::error::Result;
use crate::error::Error;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CredentialMetadata {
  pub(crate) timestamps: Timestamps,
  pub(crate) identifier: Identifier,
  pub(crate) credential_id: String,
  pub(crate) credential_issuer_id: String,
}

impl CredentialMetadata {
  pub fn new<T>(
    ident: String,
    index: Index,
    credential: &Credential<T>,
  ) -> Result<Self> {
    let credential_id: &Url = credential
      .id
      .as_ref()
      .ok_or(Error::InvalidCredentialMissingId)?;

    Ok(Self {
      timestamps: Timestamps::now(),
      identifier: Identifier::new(ident, index),
      credential_id: credential_id.to_string(),
      credential_issuer_id: credential.issuer.url().to_string(),
    })
  }
}

impl MetadataItem for CredentialMetadata {
  const METADATA: ResourceType = ResourceType::CredentialMetadata;
  const RESOURCE: ResourceType = ResourceType::CredentialDocument;

  fn resource(&self) -> &[u8] {
    self.credential_id.as_bytes()
  }

  fn identifier(&self) -> &Identifier {
    &self.identifier
  }
}
