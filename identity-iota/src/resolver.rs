// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;
use identity_core::convert::SerdeInto;
use identity_did::did::CoreDID;
use identity_did::error::Error;
use identity_did::error::Result;
use identity_did::resolution::DocumentMetadata;
use identity_did::resolution::InputMetadata;
use identity_did::resolution::MetaDocument;
use identity_did::resolution::ResolverMethod;

use crate::did::IotaDID;
use crate::did::IotaDocument;
use crate::tangle::Client;
use crate::tangle::ClientMap;
use crate::tangle::TangleResolve;

#[async_trait(?Send)]
impl ResolverMethod for Client {
  fn is_supported(&self, did: &CoreDID) -> bool {
    IotaDID::check_validity(did).is_ok()
  }

  async fn read(&self, did: &CoreDID, _input: InputMetadata) -> Result<Option<MetaDocument>> {
    let document: IotaDocument = IotaDID::try_from_borrowed(did)
      .map_err(|_| Error::MissingResolutionDID)
      .map(|did| self.resolve(did))?
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

#[async_trait(?Send)]
impl ResolverMethod for ClientMap {
  fn is_supported(&self, did: &CoreDID) -> bool {
    IotaDID::check_validity(did).is_ok()
  }

  async fn read(&self, did: &CoreDID, input: InputMetadata) -> Result<Option<MetaDocument>> {
    let iota_did: &IotaDID = IotaDID::try_from_borrowed(did).map_err(|_| Error::MissingResolutionDID)?;
    let network = iota_did.network().map_err(|_| Error::MissingResolutionDID)?;

    self
      .client(network)
      .await
      .map_err(|_| Error::MissingResolutionDocument)?
      .read(did, input)
      .await
  }
}
