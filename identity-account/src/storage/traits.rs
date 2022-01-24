// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;

use async_trait::async_trait;

use identity_core::crypto::PrivateKey;
use identity_core::crypto::PublicKey;
use identity_iota::did::IotaDID;

use crate::error::Result;
use crate::identity::ChainState;
use crate::identity::DIDLease;
use crate::identity::IdentityState;
use crate::types::Generation;
use crate::types::KeyLocation;
use crate::types::Signature;
use crate::utils::EncryptionKey;

macro_rules! storage_trait {
  ($( $x:ident ),*) => {
    /// An interface for Identity Account storage implementations.
    ///
    /// See [MemStore][crate::storage::MemStore] for a test/example implementation.
    #[cfg_attr(not(feature = "wasm"), async_trait)]
    #[cfg_attr(feature = "wasm", async_trait(?Send))]
    pub trait Storage: $($x + )* Debug + 'static {
      /// Sets the account password.
      async fn set_password(&self, password: EncryptionKey) -> Result<()>;

      /// Write any unsaved changes to disk.
      async fn flush_changes(&self) -> Result<()>;

      /// Attempt to obtain the exclusive permission to modify the given `did`.
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

      /// Returns the last generation that has been published to the tangle for the given `did`.
      async fn published_generation(&self, did: &IotaDID) -> Result<Option<Generation>>;

      /// Sets the last generation that has been published to the tangle for the given `did`.
      async fn set_published_generation(&self, did: &IotaDID, index: Generation) -> Result<()>;

      /// Returns the chain state of the identity specified by `did`.
      async fn chain_state(&self, did: &IotaDID) -> Result<Option<ChainState>>;

      /// Set the chain state of the identity specified by `did`.
      async fn set_chain_state(&self, did: &IotaDID, chain_state: &ChainState) -> Result<()>;

      /// Returns the state of the identity specified by `did`.
      async fn state(&self, did: &IotaDID) -> Result<Option<IdentityState>>;

      /// Sets a new state for the identity specified by `did`.
      async fn set_state(&self, did: &IotaDID, state: &IdentityState) -> Result<()>;

      /// Removes the keys and any state for the identity specified by `did`.
      async fn purge(&self, did: &IotaDID) -> Result<()>;
    }
  };
}

#[cfg(feature = "wasm")]
storage_trait!();

#[cfg(not(feature = "wasm"))]
storage_trait!(Send, Sync);
