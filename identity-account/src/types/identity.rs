// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::ops::Deref;
use core::ops::DerefMut;
use hashbrown::HashSet;
use identity_core::crypto::JcsEd25519;
use identity_core::crypto::KeyType;
use identity_core::crypto::SetSignature;
use identity_core::crypto::Signer;
use identity_did::error::Error as DIDError;
use identity_did::verification::MethodQuery;
use identity_did::verification::MethodType;
use identity_iota::did::Document;
use identity_iota::did::Method;
use serde::Serialize;

use crate::crypto::RemoteEd25519;
use crate::crypto::RemoteKey;
use crate::error::Result;
use crate::storage::StorageHandle;
use crate::types::IdentityMetadata;
use crate::types::KeyLocation;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Identity {
  base: Document,
}

impl Identity {
  pub const fn new(base: Document) -> Self {
    Self { base }
  }

  // Note: This needs to match Document::sign_this
  pub(crate) async fn sign_this(&mut self, metadata: &IdentityMetadata, storage: &StorageHandle) -> Result<()> {
    let method: &Method = self.base.authentication();
    let fragment: String = method.try_into_fragment()?;

    match method.key_type() {
      MethodType::Ed25519VerificationKey2018 => {
        let location: KeyLocation<'_> = KeyLocation::borrowed(KeyType::Ed25519, metadata.index(), &fragment);
        let secret: RemoteKey<'_> = RemoteKey::new(storage, location);

        JcsEd25519::<RemoteEd25519>::create_signature(&mut self.base, &fragment, &secret)?;

        Ok(())
      }
      MethodType::MerkleKeyCollection2021 => {
        // Documents can't be signed with Merkle Key Collections
        Err(DIDError::InvalidMethodType.into())
      }
      _ => Err(DIDError::UnknownMethodType.into()),
    }
  }

  pub(crate) fn verify_this(&self) -> Result<()> {
    self.base.verify_this().map_err(Into::into)
  }

  pub(crate) async fn sign_that<'a, X, T>(
    &self,
    metadata: &IdentityMetadata,
    storage: &StorageHandle,
    query: T,
    data: &mut X,
  ) -> Result<()>
  where
    X: Serialize + SetSignature,
    T: Into<MethodQuery<'a>>,
  {
    let method: _ = self.base.try_resolve(query)?;
    let fragment: String = method.try_into_fragment()?;

    match method.key_type() {
      MethodType::Ed25519VerificationKey2018 => {
        let location: KeyLocation<'_> = KeyLocation::borrowed(KeyType::Ed25519, metadata.index(), &fragment);
        let secret: RemoteKey<'_> = RemoteKey::new(storage, location);

        JcsEd25519::<RemoteEd25519>::create_signature(data, &fragment, &secret)?;

        Ok(())
      }
      MethodType::MerkleKeyCollection2021 => {
        todo!("Key Collection Locations")
      }
      _ => Err(DIDError::UnknownMethodType.into()),
    }
  }

  pub(crate) fn append_fragments(
    &self,
    metadata: &IdentityMetadata,
    keys: &mut HashSet<KeyLocation<'static>>,
  ) -> Result<()> {
    for method in self.base.methods() {
      let fragment: String = method.try_into_fragment()?;

      match method.key_type() {
        MethodType::Ed25519VerificationKey2018 => {
          keys.insert(KeyLocation::owned(KeyType::Ed25519, metadata.index(), fragment));
        }
        MethodType::MerkleKeyCollection2021 => {
          todo!("TODO: Handle Merkle Key Collection")
        }
        _ => return Err(DIDError::UnknownMethodType.into()),
      }
    }

    Ok(())
  }
}

impl Deref for Identity {
  type Target = Document;

  fn deref(&self) -> &Self::Target {
    &self.base
  }
}

impl DerefMut for Identity {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.base
  }
}
