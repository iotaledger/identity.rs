// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;
use futures::stream::BoxStream;
use futures::TryStreamExt;
use identity_core::crypto::PrivateKey;
use identity_core::crypto::PublicKey;
use identity_iota::did::IotaDID;

use crate::error::Result;
use crate::events::Commit;
use crate::identity::DIDLease;
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

  /// Attempt to obtain the exclusive permission to modify the given did.
  /// The caller is expected to make no more modifications after the lease has been dropped.
  /// Returns an [`IdentityInUse`][crate::Error::IdentityInUse] error if already leased.
  async fn lease_did(&self, did: &IotaDID) -> Result<DIDLease>;

  /// Creates a new keypair at the specified `location`
  async fn key_new(&self, did: &IotaDID, location: &KeyLocation) -> Result<PublicKey>;

  /// Inserts a private key at the specified `location`.
  async fn key_insert(&self, did: &IotaDID, location: &KeyLocation, private_key: PrivateKey) -> Result<PublicKey>;

  /// Retrieves the public key at the specified `location`.
  async fn key_get(&self, did: &IotaDID, location: &KeyLocation) -> Result<PublicKey>;

  /// Deletes the keypair specified by `location`.
  async fn key_del(&self, did: &IotaDID, location: &KeyLocation) -> Result<()>;

  /// Signs `data` with the private key at the specified `location`.
  async fn key_sign(&self, did: &IotaDID, location: &KeyLocation, data: Vec<u8>) -> Result<Signature>;

  /// Returns `true` if a keypair exists at the specified `location`.
  async fn key_exists(&self, did: &IotaDID, location: &KeyLocation) -> Result<bool>;

  /// Returns the last generation that has been published to the tangle for the given `id`.
  async fn published_generation(&self, did: &IotaDID) -> Result<Option<Generation>>;

  /// Sets the last generation that has been published to the tangle for the given `id`.
  async fn set_published_generation(&self, did: &IotaDID, index: Generation) -> Result<()>;

  /// Returns the state snapshot of the identity specified by `id`.
  async fn snapshot(&self, did: &IotaDID) -> Result<Option<IdentitySnapshot>>;

  /// Sets a new state snapshot for the identity specified by `id`.
  async fn set_snapshot(&self, did: &IotaDID, snapshot: &IdentitySnapshot) -> Result<()>;

  /// Appends a set of commits to the event stream for the identity specified by `id`.
  async fn append(&self, did: &IotaDID, commits: &[Commit]) -> Result<()>;

  /// Returns a stream of commits for the identity specified by `id`.
  ///
  /// The stream may be offset by `index`.
  async fn stream(&self, did: &IotaDID, index: Generation) -> Result<BoxStream<'_, Result<Commit>>>;

  /// Returns a list of all commits for the identity specified by `id`.
  ///
  /// The list may be offset by `index`.
  async fn collect(&self, did: &IotaDID, index: Generation) -> Result<Vec<Commit>> {
    self.stream(did, index).await?.try_collect().await
  }

  /// Removes the event stream and state snapshot for the identity specified by `id`.
  async fn purge(&self, did: &IotaDID) -> Result<()>;
}

#[async_trait::async_trait]
impl Storage for Box<dyn Storage> {
  async fn set_password(&self, password: EncryptionKey) -> Result<()> {
    (**self).set_password(password).await
  }

  async fn flush_changes(&self) -> Result<()> {
    (**self).flush_changes().await
  }

  async fn lease_did(&self, did: &IotaDID) -> Result<DIDLease> {
    (**self).lease_did(did).await
  }

  async fn key_new(&self, did: &IotaDID, location: &KeyLocation) -> Result<PublicKey> {
    (**self).key_new(did, location).await
  }

  async fn key_insert(&self, did: &IotaDID, location: &KeyLocation, private_key: PrivateKey) -> Result<PublicKey> {
    (**self).key_insert(did, location, private_key).await
  }

  async fn key_get(&self, did: &IotaDID, location: &KeyLocation) -> Result<PublicKey> {
    (**self).key_get(did, location).await
  }

  async fn key_del(&self, did: &IotaDID, location: &KeyLocation) -> Result<()> {
    (**self).key_del(did, location).await
  }

  async fn key_sign(&self, did: &IotaDID, location: &KeyLocation, data: Vec<u8>) -> Result<Signature> {
    (**self).key_sign(did, location, data).await
  }

  async fn key_exists(&self, did: &IotaDID, location: &KeyLocation) -> Result<bool> {
    (**self).key_exists(did, location).await
  }

  async fn snapshot(&self, did: &IotaDID) -> Result<Option<IdentitySnapshot>> {
    (**self).snapshot(did).await
  }

  async fn set_snapshot(&self, did: &IotaDID, snapshot: &IdentitySnapshot) -> Result<()> {
    (**self).set_snapshot(did, snapshot).await
  }

  async fn append(&self, did: &IotaDID, commits: &[Commit]) -> Result<()> {
    (**self).append(did, commits).await
  }

  async fn stream(&self, did: &IotaDID, index: Generation) -> Result<BoxStream<'_, Result<Commit>>> {
    (**self).stream(did, index).await
  }

  async fn purge(&self, did: &IotaDID) -> Result<()> {
    (**self).purge(did).await
  }

  async fn published_generation(&self, did: &IotaDID) -> Result<Option<Generation>> {
    (**self).published_generation(did).await
  }

  async fn set_published_generation(&self, did: &IotaDID, index: Generation) -> Result<()> {
    (**self).set_published_generation(did, index).await
  }
}
