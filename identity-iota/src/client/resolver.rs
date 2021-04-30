// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;
use identity_core::convert::SerdeInto;
use identity_did::did::DID as CoreDID;
use identity_did::error::Error;
use identity_did::error::Result;
use identity_did::resolution::DocumentMetadata;
use identity_did::resolution::InputMetadata;
use identity_did::resolution::MetaDocument;
use identity_did::resolution::ResolverMethod;

use crate::client::Client;
use crate::did::IotaDID;
use crate::did::IotaDocument;

#[async_trait(?Send)]
impl ResolverMethod for Client {
  fn is_supported(&self, did: &CoreDID) -> bool {
    IotaDID::try_from_borrowed(did)
      .map(|did| self.check_network(did).is_ok())
      .unwrap_or(false)
  }

  async fn read(&self, did: &CoreDID, _input: InputMetadata) -> Result<Option<MetaDocument>> {
    let document: IotaDocument = IotaDID::try_from_borrowed(did)
      .map_err(|_| Error::MissingResolutionDID)
      .map(|did| self.read_document(&did))?
      .await
      .map_err(|_| Error::MissingResolutionDocument)?;

    let mut meta: DocumentMetadata = DocumentMetadata::new();
    meta.created = Some(document.created());
    meta.updated = Some(document.updated());

    Ok(Some(MetaDocument {
      data: document.serde_into()?,
      meta,
    }))
  }
}
