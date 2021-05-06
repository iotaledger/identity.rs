// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;
use futures::stream::BoxStream;
use futures::TryStreamExt;
use identity_core::crypto::PublicKey;

use crate::error::Result;
use crate::events::Commit;
use crate::identity::IdentityId;
use crate::identity::IdentityIndex;
use crate::identity::IdentitySnapshot;
use crate::types::Generation;
use crate::types::KeyLocation;
use crate::types::Signature;
use crate::utils::EncryptionKey;

/// An interface for Identity Account storage implementations.
///
/// See [MemStore][crate::storage::MemStore] for a test/example implementation.
#[async_trait::async_trait]
pub trait Storage: Debug + Send + Sync + 'static {
  /// Sets the account password.
  async fn set_password(&self, password: EncryptionKey) -> Result<()>;

  /// Write any unsaved changes to disk.
  async fn flush_changes(&self) -> Result<()>;

  /// Creates a new keypair at the specified `location`
  async fn key_new(&self, id: IdentityId, location: &KeyLocation) -> Result<PublicKey>;

  /// Retrieves the public key at the specified `location`.
  async fn key_get(&self, id: IdentityId, location: &KeyLocation) -> Result<PublicKey>;

  /// Deletes the keypair specified by `location`.
  async fn key_del(&self, id: IdentityId, location: &KeyLocation) -> Result<()>;

  /// Signs `data` with the private key at the specified `location`.
  async fn key_sign(&self, id: IdentityId, location: &KeyLocation, data: Vec<u8>) -> Result<Signature>;

  /// Returns `true` if a keypair exists at the specified `location`.
  async fn key_exists(&self, id: IdentityId, location: &KeyLocation) -> Result<bool>;

  /// Returns the account identity index.
  async fn index(&self) -> Result<IdentityIndex>;

  /// Sets a new account identity index.
  async fn set_index(&self, index: &IdentityIndex) -> Result<()>;

  /// Returns the state snapshot of the identity specified by `id`.
  async fn snapshot(&self, id: IdentityId) -> Result<Option<IdentitySnapshot>>;

  /// Sets a new state snapshot for the identity specified by `id`.
  async fn set_snapshot(&self, id: IdentityId, snapshot: &IdentitySnapshot) -> Result<()>;

  /// Appends a set of commits to the event stream for the identity specified by `id`.
  async fn append(&self, id: IdentityId, commits: &[Commit]) -> Result<()>;

  /// Returns a stream of commits for the identity specified by `id`.
  ///
  /// The stream may be offset by `index`.
  async fn stream(&self, id: IdentityId, index: Generation) -> Result<BoxStream<'_, Result<Commit>>>;

  /// Returns a list of all commits for the identity specified by `id`.
  ///
  /// The list may be offset by `index`.
  async fn collect(&self, id: IdentityId, index: Generation) -> Result<Vec<Commit>> {
    self.stream(id, index).await?.try_collect().await
  }

  /// Removes the event stream and state snapshot for the identity specified by `id`.
  async fn purge(&self, id: IdentityId) -> Result<()>;
}

#[async_trait::async_trait]
impl Storage for Box<dyn Storage> {
  async fn set_password(&self, password: EncryptionKey) -> Result<()> {
    (**self).set_password(password).await
  }

  async fn flush_changes(&self) -> Result<()> {
    (**self).flush_changes().await
  }

  async fn key_new(&self, id: IdentityId, location: &KeyLocation) -> Result<PublicKey> {
    (**self).key_new(id, location).await
  }

  async fn key_get(&self, id: IdentityId, location: &KeyLocation) -> Result<PublicKey> {
    (**self).key_get(id, location).await
  }

  async fn key_del(&self, id: IdentityId, location: &KeyLocation) -> Result<()> {
    (**self).key_del(id, location).await
  }

  async fn key_sign(&self, id: IdentityId, location: &KeyLocation, data: Vec<u8>) -> Result<Signature> {
    (**self).key_sign(id, location, data).await
  }

  async fn key_exists(&self, id: IdentityId, location: &KeyLocation) -> Result<bool> {
    (**self).key_exists(id, location).await
  }

  async fn index(&self) -> Result<IdentityIndex> {
    (**self).index().await
  }

  async fn set_index(&self, index: &IdentityIndex) -> Result<()> {
    (**self).set_index(index).await
  }

  async fn snapshot(&self, id: IdentityId) -> Result<Option<IdentitySnapshot>> {
    (**self).snapshot(id).await
  }

  async fn set_snapshot(&self, id: IdentityId, snapshot: &IdentitySnapshot) -> Result<()> {
    (**self).set_snapshot(id, snapshot).await
  }

  async fn append(&self, id: IdentityId, commits: &[Commit]) -> Result<()> {
    (**self).append(id, commits).await
  }

  async fn stream(&self, id: IdentityId, index: Generation) -> Result<BoxStream<'_, Result<Commit>>> {
    (**self).stream(id, index).await
  }

  async fn purge(&self, id: IdentityId) -> Result<()> {
    (**self).purge(id).await
  }
}
