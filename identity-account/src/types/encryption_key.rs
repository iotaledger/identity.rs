// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use identity_account_storage::storage::Storage;
use identity_account_storage::types::KeyLocation;
use identity_core::crypto::PublicKey;
use identity_did::verification::MethodScope;
use identity_iota_core::did::IotaDID;
use identity_iota_core::document::IotaDocument;
use identity_iota_core::document::IotaVerificationMethod;

use crate::Error;
use crate::Result;

/// 
#[derive(Clone, Debug)]
pub enum EncryptionKey {
  Ed25519,
  X25519(PublicKey),
}

impl EncryptionKey {
  pub async fn key_location(
    &self,
    fragment: &str,
    did: &IotaDID,
    document: &IotaDocument,
    storage: &Arc<dyn Storage>,
  ) -> Result<KeyLocation> {
    match self {
      Self::Ed25519 => {
        let method: &IotaVerificationMethod = document
          .resolve_method::<&str>(fragment.into(), None)
          .ok_or(Error::DIDError(identity_did::Error::MethodNotFound))?;
        KeyLocation::from_verification_method(method).map_err(Into::into)
      }
      Self::X25519(peer_public_key) => {
        let method: &IotaVerificationMethod = document
          .resolve_method::<&str>(fragment.into(), Some(MethodScope::key_agreement()))
          .ok_or(Error::DIDError(identity_did::Error::MethodNotFound))?;
        let location: KeyLocation = KeyLocation::from_verification_method(method)?;
        storage
          .key_exchange(did, &location, peer_public_key.clone(), &format!("{}-kex", fragment))
          .await
          .map_err(Into::into)
      }
    }
  }
}
