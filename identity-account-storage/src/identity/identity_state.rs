// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Fragment;
use identity_core::crypto::SetSignature;
use identity_core::crypto::SignatureOptions;
use identity_did::did::DID;
use identity_did::verification::MethodType;
use identity_iota_core::did::IotaDID;
use identity_iota_core::did::IotaDIDUrl;
use identity_iota_core::document::IotaDocument;
use serde::Deserialize;
use serde::Serialize;

use crate::crypto::RemoteEd25519;
use crate::crypto::RemoteKey;
use crate::error::Result;
use crate::storage::Storage;
use crate::types::KeyLocation;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct IdentityState {
  document: IotaDocument,
}

impl IdentityState {
  pub fn new(document: IotaDocument) -> Self {
    Self { document }
  }

  // ===========================================================================
  // Document State
  // ===========================================================================

  pub fn document(&self) -> &IotaDocument {
    &self.document
  }

  pub fn document_mut(&mut self) -> &mut IotaDocument {
    &mut self.document
  }

  pub async fn sign_data<U>(
    &self,
    did: &IotaDID,
    store: &dyn Storage,
    location: &KeyLocation,
    data: &mut U,
    options: SignatureOptions,
  ) -> Result<()>
  where
    U: Serialize + SetSignature,
  {
    // Create a private key suitable for identity_core::crypto
    let private: RemoteKey<'_> = RemoteKey::new(did, location, store);

    // Create the Verification Method identifier
    let fragment: Fragment = Fragment::new(location.fragment.clone());
    let method_url: IotaDIDUrl = self.document.id().to_url().join(fragment.identifier())?;

    match location.method() {
      MethodType::Ed25519VerificationKey2018 => {
        RemoteEd25519::create_signature(data, method_url.to_string(), &private, options).await?;
      }
      MethodType::MerkleKeyCollection2021 => {
        todo!("Handle MerkleKeyCollection2021")
      }
    }

    Ok(())
  }
}
