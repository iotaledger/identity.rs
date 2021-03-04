// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Timestamp;
use identity_core::crypto::KeyPair;
use identity_core::crypto::KeyType;
use identity_core::crypto::PublicKey;
use identity_iota::did::Document;
use identity_iota::did::Method;
use identity_iota::did::DID;

use crate::error::Error;
use crate::error::Result;
use crate::identity::Identity;
use crate::identity::IdentityHandle;
use crate::storage::KeyLocation;
use crate::storage::StorageHandle;
use crate::utils::GenericCache;

const TAG: &str = "authentication";

#[derive(Debug)]
pub struct IdentityBuilder<'a> {
  store: &'a StorageHandle,
  cache: &'a GenericCache<IdentityHandle>,
  name: Option<String>,
  key_type: KeyType,
}

impl<'a> IdentityBuilder<'a> {
  pub fn new(store: &'a StorageHandle, cache: &'a GenericCache<IdentityHandle>) -> Self {
    Self {
      store,
      cache,
      name: None,
      key_type: KeyType::Ed25519,
    }
  }
}

impl IdentityBuilder<'_> {
  pub fn name<T>(mut self, value: T) -> Self
  where
    T: Into<String>,
  {
    self.name = Some(value.into());
    self
  }

  pub fn key_type(mut self, value: KeyType) -> Self {
    self.key_type = value;
    self
  }

  pub async fn build(self) -> Result<IdentityHandle> {
    let cache: _ = self.cache.read()?;

    // Find the next unique index in the list of identities
    //
    // The cache is expected to contain all identities in the account
    let mut index: u32 = 0;
    for identity in cache.values() {
      let current: u32 = identity.index().await;

      if index <= current {
        index = current.checked_add(1).ok_or(Error::IdentityOverload)?;
      }
    }

    // Take the user-provided name or generate one from the index
    let name: String = self.name.unwrap_or_else(|| format!("Identity {}", index));

    // Ensure the name is unique
    for identity in cache.values() {
      if identity.name().await == name {
        return Err(Error::IdentityDuplicateName);
      }
    }

    // Generate a new DID document
    let document: Document = 'outer: loop {
      let location: KeyLocation = KeyLocation::new(index, TAG, self.key_type);
      let public: PublicKey = self.store.generate_public_key(location).await?;

      // Generate a new DID URL from the public key
      //
      // TODO: Allow creating DIDs from KeyType/PublicKey
      let keypair: KeyPair = (self.key_type, public, Vec::new().into()).into();
      let did: DID = DID::new(keypair.public().as_ref())?;

      // Ensure we didn't generate a duplicate DID
      for identity in cache.values() {
        if identity.document().await.id().tag() == did.tag() {
          continue 'outer;
        }
      }

      // Generate an authentication method
      let method: Method = Method::from_did(did, &keypair, TAG)?;

      // Finally, create a DID Document
      // SAFETY: We just created a valid authentication method.
      break unsafe { Document::from_authentication_unchecked(method) };
    };

    let identity: Identity = Identity {
      id: document.id().clone(),
      index,
      name,
      created_at: Timestamp::now(),
      updated_at: Timestamp::now(),
      document,
    };

    // Save the newly created Identity
    identity.save(&self.store).await?;

    // Convert to a thread-safe handle; keep an owned copy of the id
    let id: String = identity.id.to_string();
    let handle: IdentityHandle = IdentityHandle::new(identity);

    // Drop the read-only cache guard
    drop(cache);

    // Add the new Identity to the cache
    self.cache.write()?.insert(id, handle.clone());

    // TODO: Publish to Tangle

    Ok(handle)
  }
}
