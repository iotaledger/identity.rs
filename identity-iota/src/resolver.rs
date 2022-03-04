// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;
use identity_did::did::CoreDID;
use identity_did::error::Error;
use identity_did::error::Result;
use identity_did::resolution::DocumentMetadata;
use identity_did::resolution::InputMetadata;
use identity_did::resolution::MetaDocument;
use identity_did::resolution::ResolverMethod;
use identity_iota_core::did::IotaDID;

use crate::document::ResolvedIotaDocument;
use crate::tangle::Client;
use crate::tangle::TangleResolve;

#[async_trait(?Send)]
impl ResolverMethod for Client {
  fn is_supported(&self, did: &CoreDID) -> bool {
    IotaDID::check_validity(did).is_ok()
  }

  async fn read(&self, did: &CoreDID, _input: InputMetadata) -> Result<Option<MetaDocument>> {
    let iota_did: IotaDID = IotaDID::try_from_core(did.clone()).map_err(|_| Error::MissingResolutionDID)?;
    let mut resolved: ResolvedIotaDocument = self
      .resolve(&iota_did)
      .await
      .map_err(|_| Error::MissingResolutionDocument)?;

    let mut metadata: DocumentMetadata = DocumentMetadata::new();
    metadata.created = Some(resolved.document.metadata.created);
    metadata.updated = Some(resolved.document.metadata.updated);

    let core_document = std::mem::take(resolved.document.core_document_mut());
    Ok(Some(MetaDocument {
      data: core_document.map(CoreDID::from, |properties| properties),
      meta: metadata,
    }))
  }
}
