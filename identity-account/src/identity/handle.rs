// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Timestamp;
use identity_iota::did::Document;
use identity_iota::did::DID;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::sync::RwLockReadGuard;
use tokio::sync::RwLockWriteGuard;

use crate::error::Result;
use crate::identity::Identity;
use crate::storage::StorageHandle;

#[derive(Clone, Debug)]
pub struct IdentityHandle(Arc<RwLock<Identity>>);

impl IdentityHandle {
  pub(crate) fn new(identity: Identity) -> Self {
    Self(Arc::new(RwLock::new(identity)))
  }

  pub(crate) async fn read(&self) -> RwLockReadGuard<'_, Identity> {
    self.0.read().await
  }

  pub(crate) async fn write(&self) -> RwLockWriteGuard<'_, Identity> {
    self.0.write().await
  }

  pub async fn into_inner(&self) -> Identity {
    self.0.read().await.clone()
  }

  pub async fn id(&self) -> DID {
    self.0.read().await.id().clone()
  }

  pub async fn index(&self) -> u32 {
    self.0.read().await.index
  }

  pub async fn name(&self) -> String {
    self.0.read().await.name.to_string()
  }

  pub async fn created_at(&self) -> Timestamp {
    self.0.read().await.created_at
  }

  pub async fn updated_at(&self) -> Timestamp {
    self.0.read().await.updated_at
  }

  pub async fn document(&self) -> Document {
    self.0.read().await.document.clone()
  }

  pub(crate) async fn save(&self, storage: &StorageHandle) -> Result<()> {
    self.0.read().await.save(storage).await
  }
}
