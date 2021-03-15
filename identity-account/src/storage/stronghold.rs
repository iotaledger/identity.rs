// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crypto::keys::slip10::Chain;
use identity_core::crypto::KeyType;
use identity_core::crypto::PublicKey;
use iota_stronghold::Location;
use iota_stronghold::SLIP10DeriveInput;
use std::path::Path;

use crate::error::Result;
use crate::storage::StorageAdapter;
use crate::storage::VaultAdapter;
use crate::stronghold::default_hint;
use crate::stronghold::Records;
use crate::stronghold::Snapshot;
use crate::stronghold::Vault;
use crate::types::KeyLocation;
use crate::types::ResourceId;
use crate::types::ResourceType;
use crate::types::Signature;
use crate::utils::EncryptionKey;

const KEYS: &str = "$_KEYS";

#[derive(Debug)]
pub struct StrongholdAdapter {
  snapshot: Snapshot,
}

impl StrongholdAdapter {
  pub async fn new<P>(path: &P, password: Option<EncryptionKey>) -> Result<Self>
  where
    P: AsRef<Path> + ?Sized,
  {
    let snapshot: Snapshot = Snapshot::new(path);

    if let Some(password) = password {
      snapshot.load(password).await?;
    }

    Ok(Self { snapshot })
  }

  fn records(&self, name: &str) -> Records<'_> {
    self.snapshot.records(name, &[])
  }

  fn vault(&self, name: &str) -> Vault<'_> {
    self.snapshot.vault(name, &[])
  }
}

const fn key(type_: ResourceType) -> &'static str {
  match type_ {
    ResourceType::Identity => "$_Identity",
    ResourceType::IdentityMeta => "$_IdentityMeta",
    ResourceType::IdentityDiff => "$_IdentityDiff",
    ResourceType::IdentityDiffMeta => "$_IdentityDiffMeta",
    ResourceType::Credential => "$_Credential",
    ResourceType::CredentialMeta => "$_CredentialMeta",
    ResourceType::Presentation => "$_Presentation",
    ResourceType::PresentationMeta => "$_PresentationMeta",
  }
}

#[async_trait::async_trait]
impl StorageAdapter for StrongholdAdapter {
  async fn all(&mut self, type_: ResourceType) -> Result<Vec<Vec<u8>>> {
    self.records(key(type_)).all().await
  }

  async fn get(&mut self, id: ResourceId<'_>) -> Result<Vec<u8>> {
    self.records(key(id.type_())).get(id.data()).await
  }

  async fn set(&mut self, id: ResourceId<'_>, data: Vec<u8>) -> Result<()> {
    self.records(key(id.type_())).set(id.data(), &data).await?;
    self.snapshot.save().await?;

    Ok(())
  }

  async fn del(&mut self, id: ResourceId<'_>) -> Result<()> {
    self.records(key(id.type_())).del(id.data()).await?;
    self.snapshot.save().await?;

    Ok(())
  }

  fn storage_path(&self) -> &Path {
    self.snapshot.path()
  }
}

#[async_trait::async_trait]
impl VaultAdapter for StrongholdAdapter {
  async fn set_password(&mut self, password: EncryptionKey) -> Result<()> {
    self.snapshot.set_password(password)
  }

  async fn key_new(&mut self, location: &KeyLocation<'_>) -> Result<PublicKey> {
    let vault: Vault<'_> = self.vault(KEYS);

    let public: PublicKey = match location.type_() {
      KeyType::Ed25519 => generate_ed25519(&vault, location).await?,
    };

    self.snapshot.save().await?;

    Ok(public)
  }

  async fn key_get(&mut self, location: &KeyLocation<'_>) -> Result<PublicKey> {
    let vault: Vault<'_> = self.vault(KEYS);

    match location.type_() {
      KeyType::Ed25519 => retrieve_ed25519(&vault, location).await,
    }
  }

  async fn key_del(&mut self, location: &KeyLocation<'_>) -> Result<()> {
    let vault: Vault<'_> = self.vault(KEYS);

    match location.type_() {
      KeyType::Ed25519 => {
        vault.delete(seed_location(location), false).await?;
        vault.delete(skey_location(location), false).await?;
      }
    }

    self.snapshot.save().await?;

    Ok(())
  }

  async fn key_sign(&mut self, location: &KeyLocation<'_>, payload: Vec<u8>) -> Result<Signature> {
    let vault: Vault<'_> = self.vault(KEYS);

    match location.type_() {
      KeyType::Ed25519 => sign_ed25519(&vault, payload, location).await,
    }
  }
}

async fn generate_ed25519(vault: &Vault<'_>, location: &KeyLocation<'_>) -> Result<PublicKey> {
  // Generate a SLIP10 seed as the private key
  vault
    .slip10_generate(seed_location(location), default_hint(), None)
    .await?;

  let chain: Chain = Chain::from_u32_hardened(vec![0, 0, 0]);
  let seed: SLIP10DeriveInput = SLIP10DeriveInput::Seed(seed_location(location));

  // Use the SLIP10 seed to derive a child key
  vault
    .slip10_derive(chain, seed, skey_location(location), default_hint())
    .await?;

  // Retrieve the public key of the derived child key
  retrieve_ed25519(vault, location).await
}

async fn retrieve_ed25519(vault: &Vault<'_>, location: &KeyLocation<'_>) -> Result<PublicKey> {
  vault
    .ed25519_public_key(skey_location(location))
    .await
    .map(|public| public.to_vec().into())
}

async fn sign_ed25519(vault: &Vault<'_>, payload: Vec<u8>, location: &KeyLocation<'_>) -> Result<Signature> {
  let public_key: PublicKey = retrieve_ed25519(&vault, location).await?;
  let signature: [u8; 64] = vault.ed25519_sign(payload, skey_location(location)).await?;

  Ok(Signature::new(public_key, signature.into()))
}

fn seed_location(location: &KeyLocation<'_>) -> Location {
  Location::generic("vault:seed", key_location(location))
}

fn skey_location(location: &KeyLocation<'_>) -> Location {
  Location::generic("vault:skey", key_location(location))
}

fn key_location(location: &KeyLocation<'_>) -> String {
  format!("{}:{}", location.identity(), location.fragment())
}
