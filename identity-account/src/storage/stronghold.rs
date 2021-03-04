// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::crypto::KeyType;
use identity_core::crypto::PublicKey;
use iota_stronghold::hd::Chain;
use iota_stronghold::Location;
use iota_stronghold::SLIP10DeriveInput;
use std::path::Path;

use crate::error::Result;
use crate::storage::KeyLocation;
use crate::storage::Signature;
use crate::storage::StorageAdapter;
use crate::storage::VaultAdapter;
use crate::stronghold::default_hint;
use crate::stronghold::Password;
use crate::stronghold::Records;
use crate::stronghold::Snapshot;
use crate::stronghold::Vault;

#[derive(Debug)]
pub struct StrongholdAdapter {
  snapshot: Snapshot,
}

impl StrongholdAdapter {
  pub async fn new<P>(path: &P, password: Option<Password>) -> Result<Self>
  where
    P: AsRef<Path> + ?Sized,
  {
    let snapshot: Snapshot = Snapshot::new(path);

    if let Some(password) = password {
      snapshot.load(password).await?;
    }

    Ok(Self { snapshot })
  }

  fn records(&self) -> Records<'_> {
    self.snapshot.records("identity", &[])
  }

  fn key_vault(&self, location: KeyLocation<'_>) -> Vault<'_> {
    self.snapshot.vault(&self.key_scope(location), &[])
  }

  fn key_scope(&self, location: KeyLocation<'_>) -> Vec<u8> {
    format!("keystore:{}", location.identity()).into_bytes()
  }
}

#[async_trait::async_trait]
impl StorageAdapter for StrongholdAdapter {
  async fn all(&mut self) -> Result<Vec<Vec<u8>>> {
    self.records().all().await
  }

  async fn get(&mut self, resource_id: &[u8]) -> Result<Vec<u8>> {
    self.records().get(resource_id).await
  }

  async fn set(&mut self, resource_id: &[u8], resource: &[u8]) -> Result<()> {
    self.records().set(resource_id, resource).await?;
    self.snapshot.save().await?;

    Ok(())
  }

  async fn del(&mut self, resource_id: &[u8]) -> Result<()> {
    self.records().del(resource_id).await?;
    self.snapshot.save().await?;

    Ok(())
  }

  fn storage_path(&self) -> &Path {
    self.snapshot.path()
  }
}

#[async_trait::async_trait]
impl VaultAdapter for StrongholdAdapter {
  async fn set_password(&self, password: Password) -> Result<()> {
    self.snapshot.set_password(password)
  }

  async fn generate_public_key(&mut self, location: KeyLocation<'_>) -> Result<PublicKey> {
    let vault: Vault<'_> = self.key_vault(location);

    let public: PublicKey = match location.type_() {
      KeyType::Ed25519 => generate_ed25519(&vault, location).await?,
    };

    self.snapshot.save().await?;

    Ok(public)
  }

  async fn retrieve_public_key(&mut self, location: KeyLocation<'_>) -> Result<PublicKey> {
    let vault: Vault<'_> = self.key_vault(location);

    match location.type_() {
      KeyType::Ed25519 => retrieve_ed25519(&vault, location).await,
    }
  }

  async fn generate_signature(&mut self, payload: Vec<u8>, location: KeyLocation<'_>) -> Result<Signature> {
    let vault: Vault<'_> = self.key_vault(location);

    match location.type_() {
      KeyType::Ed25519 => sign_ed25519(&vault, payload, location).await,
    }
  }
}

async fn generate_ed25519(vault: &Vault<'_>, location: KeyLocation<'_>) -> Result<PublicKey> {
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

async fn retrieve_ed25519(vault: &Vault<'_>, location: KeyLocation<'_>) -> Result<PublicKey> {
  vault
    .ed25519_public_key(skey_location(location))
    .await
    .map(|public| public.to_vec().into())
}

async fn sign_ed25519(vault: &Vault<'_>, payload: Vec<u8>, location: KeyLocation<'_>) -> Result<Signature> {
  let public_key: PublicKey = retrieve_ed25519(&vault, location).await?;
  let signature: [u8; 64] = vault.ed25519_sign(payload, skey_location(location)).await?;

  Ok(Signature::new(public_key, signature.into()))
}

fn seed_location(location: KeyLocation<'_>) -> Location {
  Location::generic("vault:seed", location.fragment())
}

fn skey_location(location: KeyLocation<'_>) -> Location {
  Location::generic("vault:skey", location.fragment())
}

#[cfg(test)]
mod tests {
  use core::convert::TryInto;
  use crypto::ed25519;
  use futures::executor::block_on;
  use hashbrown::HashMap;
  use identity_core::crypto::KeyType;
  use identity_core::crypto::PublicKey;
  use rusty_fork::rusty_fork_test;
  use std::fs;
  use std::path::Path;

  use crate::storage::KeyLocation;
  use crate::storage::Signature;
  use crate::storage::StrongholdAdapter;
  use crate::storage::VaultAdapter;
  use crate::utils::derive_encryption_key;
  use crate::utils::EncryptionKey;

  const MAX_I: u32 = 2;
  const MAX_K: u32 = 3;

  rusty_fork_test! {
    #[test]
    fn test_stronghold_adapter() {
      block_on(async {
        let filename: &Path = "./test-storage/snapshot.stronghold".as_ref();
        let password: EncryptionKey = derive_encryption_key("my-password");

        if filename.exists() {
          fs::remove_file(filename).unwrap();
        }

        let mut adapter: _ = StrongholdAdapter::new(filename, Some(password)).await.unwrap();

        let mut out_keys: HashMap<u32, HashMap<u32, PublicKey>> = HashMap::new();
        let mut out_sigs: HashMap<u32, HashMap<u32, Signature>> = HashMap::new();

        for identity in 0..MAX_I {
          for key in 0..MAX_K {
            let fragment: String = format!("key-{}", key);
            let location: KeyLocation = KeyLocation::new(identity, &fragment, KeyType::Ed25519);

            let public_a: PublicKey = adapter.generate_public_key(location).await.unwrap();
            let public_b: PublicKey = adapter.retrieve_public_key(location).await.unwrap();

            assert_eq!(public_a.as_ref(), public_b.as_ref());

            out_keys.entry(identity).or_default().insert(key, public_a);
          }
        }

        for identity in 0..MAX_I {
          for key in 0..MAX_K {
            let fragment: String = format!("key-{}", key);
            let location: KeyLocation = KeyLocation::new(identity, &fragment, KeyType::Ed25519);

            let signature: Signature = adapter.generate_signature(b"IOTA".to_vec(), location).await.unwrap();

            assert_eq!(out_keys[&identity][&key].as_ref(), signature.public_key().as_ref());

            out_sigs.entry(identity).or_default().insert(key, signature);
          }
        }

        for identity in 0..MAX_I {
          for key in 0..MAX_K {
            let public_key: &PublicKey = &out_keys[&identity][&key];
            let signature: &Signature = &out_sigs[&identity][&key];

            let public: ed25519::PublicKey = {
              let bytes: [u8; 32] = public_key.as_ref().try_into().unwrap();
              ed25519::PublicKey::from_compressed_bytes(bytes).unwrap()
            };

            let signature: ed25519::Signature = {
              let bytes: [u8; 64] = signature.signature().try_into().unwrap();
              ed25519::Signature::from_bytes(bytes)
            };

            assert_eq!(ed25519::verify(&public, &signature, b"IOTA"), true);
          }
        }
      })
    }
  }
}
