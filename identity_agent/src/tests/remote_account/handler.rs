// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use dashmap::DashMap;
use identity_iota_core::IotaDID;
use identity_iota_core::IotaDocument;

use crate::agent::Handler;
use crate::agent::RequestContext;
use crate::tests::remote_account::IdentityCreate;
use crate::tests::remote_account::IdentityGet;
use crate::tests::remote_account::IdentityList;
use crate::tests::remote_account::RemoteAccountError;

/// A proof-of-concept implementation of a remote account
/// which holds and manages a collection of DID documents.
#[derive(Debug, Clone)]
pub(crate) struct RemoteAccount {
  documents: Arc<DashMap<IotaDID, IotaDocument>>,
}

#[async_trait::async_trait]
impl Handler<IdentityList> for RemoteAccount {
  async fn handle(&self, _: RequestContext<IdentityList>) -> Vec<IotaDID> {
    self.documents.iter().map(|entry| entry.key().to_owned()).collect()
  }
}

#[async_trait::async_trait]
impl Handler<IdentityCreate> for RemoteAccount {
  async fn handle(&self, request: RequestContext<IdentityCreate>) -> Result<(), RemoteAccountError> {
    let document = request.input.0;

    if document.id().is_placeholder() {
      return Err(RemoteAccountError::PlaceholderDID);
    }

    self.documents.insert(document.id().to_owned(), document);
    Ok(())
  }
}

#[async_trait::async_trait]
impl Handler<IdentityGet> for RemoteAccount {
  async fn handle(&self, request: RequestContext<IdentityGet>) -> Result<IotaDocument, RemoteAccountError> {
    self
      .documents
      .get(&request.input.0)
      .map(|document| document.to_owned())
      .ok_or(RemoteAccountError::IdentityNotFound)
  }
}

impl RemoteAccount {
  pub(crate) fn new() -> Self {
    Self {
      documents: Arc::new(DashMap::new()),
    }
  }
}
