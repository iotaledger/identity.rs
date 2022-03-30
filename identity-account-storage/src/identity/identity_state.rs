// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use hashbrown::HashMap;
use identity_core::common::Fragment;
use identity_core::crypto::ProofOptions;
use identity_core::crypto::SetSignature;
use identity_did::did::DID;
use identity_did::verification::MethodType;
use identity_iota_core::did::IotaDID;
use identity_iota_core::did::IotaDIDUrl;
use identity_iota_core::document::IotaDocument;
use serde::Deserialize;
use serde::Serialize;

use crate::crypto::RemoteEd25519;
use crate::crypto::RemoteKey;
use crate::error::Error;
use crate::error::Result;
use crate::storage::Storage;
use crate::types::Generation;
use crate::types::KeyLocation;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct IdentityState {
  generation: Generation,
  #[serde(skip_serializing_if = "HashMap::is_empty")]
  method_generations: HashMap<Fragment, Generation>,
  document: IotaDocument,
}

impl IdentityState {
  pub fn new(document: IotaDocument) -> Self {
    Self {
      generation: Generation::new(),
      method_generations: HashMap::new(),
      document,
    }
  }

  // ===========================================================================
  // Internal State
  // ===========================================================================

  /// Returns the current generation of the identity integration chain.
  pub fn generation(&self) -> Generation {
    self.generation
  }

  /// Increments the generation of the identity diff chain.
  pub fn increment_generation(&mut self) -> Result<()> {
    self.generation = self.generation.try_increment()?;

    Ok(())
  }

  /// Stores the generations at which the method was inserted.
  pub fn store_method_generations(&mut self, fragment: Fragment) {
    self.method_generations.insert(fragment, self.generation());
  }

  /// Return the `KeyLocation` of the given method.
  pub fn method_location(&self, method_type: MethodType, fragment: String) -> Result<KeyLocation> {
    let fragment = Fragment::new(fragment);
    // We don't return `MethodNotFound`, as the `KeyNotFound` error might occur when a method exists
    // in the document, but the key is not present locally (e.g. in a distributed setup).
    let generation = self.method_generations.get(&fragment).ok_or(Error::KeyNotFound)?;

    Ok(KeyLocation::new(method_type, fragment.into(), *generation))
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

  /// Returns a key location suitable for the specified `fragment`.
  pub fn key_location(&self, method: MethodType, fragment: String) -> Result<KeyLocation> {
    Ok(KeyLocation::new(method, fragment, self.generation()))
  }

  pub async fn sign_data<U>(
    &self,
    did: &IotaDID,
    store: &dyn Storage,
    location: &KeyLocation,
    data: &mut U,
    options: ProofOptions,
  ) -> Result<()>
  where
    U: Serialize + SetSignature,
  {
    // Create a private key suitable for identity_core::crypto
    let private: RemoteKey<'_> = RemoteKey::new(did, location, store);

    // Create the Verification Method identifier
    let fragment: &str = location.fragment().identifier();
    let method_url: IotaDIDUrl = self.document.id().to_url().join(fragment)?;

    match location.method() {
      MethodType::Ed25519VerificationKey2018 => {
        RemoteEd25519::create_signature(data, method_url.to_string(), &private, options).await?;
      }
      MethodType::X25519KeyAgreementKey2019 => return Err(identity_did::Error::InvalidMethodType.into()),
    }

    Ok(())
  }
}
