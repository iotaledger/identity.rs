// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;
use futures::stream::LocalBoxStream;
use futures::TryStreamExt;
use identity_core::crypto::PublicKey;

use crate::chain::ChainIndex;
use crate::chain::ChainKey;
use crate::error::Result;
use crate::events::Commit;
use crate::events::Snapshot;
use crate::types::ChainId;
use crate::types::Signature;
use crate::utils::EncryptionKey;

#[async_trait::async_trait(?Send)]
pub trait Storage: Debug {
  /// Sets the account password.
  async fn set_password(&self, password: EncryptionKey) -> Result<()>;

  /// Write any unsaved changes to disk.
  async fn flush_changes(&self) -> Result<()>;

  /// Creates a new keypair at the specified `location`
  async fn key_new(&self, chain: ChainId, location: &ChainKey) -> Result<PublicKey>;

  /// Retrieves the public key at the specified `location`.
  async fn key_get(&self, chain: ChainId, location: &ChainKey) -> Result<PublicKey>;

  /// Deletes the keypair specified by `location`.
  async fn key_del(&self, chain: ChainId, location: &ChainKey) -> Result<()>;

  /// Signs `data` with the private key at the specified `location`.
  async fn key_sign(&self, chain: ChainId, location: &ChainKey, _data: Vec<u8>) -> Result<Signature>;

  async fn get_chain_index(&self) -> Result<ChainIndex>;

  async fn set_chain_index(&self, index: &ChainIndex) -> Result<()>;

  async fn get_snapshot(&self, chain: ChainId) -> Result<Option<Snapshot>>;

  async fn set_snapshot(&self, chain: ChainId, _snapshot: &Snapshot) -> Result<()>;

  async fn append(&self, chain: ChainId, _commits: &[Commit]) -> Result<()>;

  async fn stream(&self, chain: ChainId) -> Result<LocalBoxStream<'_, Result<Commit>>>;

  async fn collect(&self, chain: ChainId) -> Result<Vec<Commit>> {
    self.stream(chain).await?.try_collect().await
  }
}
