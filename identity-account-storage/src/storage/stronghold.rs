// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::path::Path;
use std::sync::Arc;

use async_trait::async_trait;
use futures::executor;
use iota_stronghold::procedures::Ed25519Sign;
use iota_stronghold::procedures::GenerateKey;
use iota_stronghold::Location;
use zeroize::Zeroize;

use identity_core::convert::FromJson;
use identity_core::convert::ToJson;
use identity_core::crypto::PrivateKey;
use identity_core::crypto::PublicKey;
use identity_did::did::DID;
use identity_did::verification::MethodType;
use identity_iota_core::did::IotaDID;

use crate::error::Result;
use crate::identity::ChainState;
use crate::identity::IdentityState;
use crate::storage::Storage;
use crate::stronghold::default_hint;
use crate::stronghold::Snapshot;
use crate::stronghold::Store;
use crate::stronghold::Vault;
use crate::types::KeyLocation;
use crate::types::Signature;
use crate::utils::derive_encryption_key;

#[derive(Debug)]
pub struct Stronghold {
  snapshot: Arc<Snapshot>,
  dropsave: bool,
}

impl Stronghold {
  /// Constructs a Stronghold storage instance.
  ///
  /// Arguments:
  ///
  /// * snapshot: path to a local Stronghold file, will be created if it does not exist.
  /// * password: password for the Stronghold file, optional.
  /// * dropsave: save all changes when the instance is dropped. Default: true.
  pub async fn new<'a, T, U>(snapshot: &T, password: U, dropsave: Option<bool>) -> Result<Self>
  where
    T: AsRef<Path> + ?Sized,
    U: Into<Option<String>>,
  {
    let snapshot: Snapshot = Snapshot::new(snapshot);

    if let Some(mut password) = password.into() {
      snapshot.load(derive_encryption_key(&password)).await?;
      password.zeroize();
    }

    Ok(Self {
      snapshot: Arc::new(snapshot),
      dropsave: dropsave.unwrap_or(true),
    })
  }

  fn store(&self, name: &str) -> Store<'_> {
    self.snapshot.store(name, &[])
  }

  fn vault(&self, id: &IotaDID) -> Vault<'_> {
    self.snapshot.vault(&fmt_did(id), &[])
  }

  /// Returns whether save-on-drop is enabled.
  pub fn dropsave(&self) -> bool {
    self.dropsave
  }

  /// Set whether to save the storage changes on drop.
  /// Default: true
  pub fn set_dropsave(&mut self, value: bool) {
    self.dropsave = value;
  }
}

#[cfg_attr(not(feature = "send-sync-storage"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync-storage", async_trait)]
impl Storage for Stronghold {
  async fn flush_changes(&self) -> Result<()> {
    self.snapshot.save().await?;
    Ok(())
  }

  async fn key_new(&self, did: &IotaDID, location: &KeyLocation) -> Result<PublicKey> {
    let vault: Vault<'_> = self.vault(did);

    let public: PublicKey = match location.method() {
      MethodType::Ed25519VerificationKey2018 | MethodType::X25519KeyAgreementKey2019 => {
        generate_private_key(&vault, location).await?
      }
    };

    Ok(public)
  }

  async fn key_insert(&self, did: &IotaDID, location: &KeyLocation, private_key: PrivateKey) -> Result<PublicKey> {
    let vault = self.vault(did);

    vault
      .insert(location_skey(location), private_key.as_ref(), default_hint(), &[])
      .await?;

    match location.method() {
      MethodType::Ed25519VerificationKey2018 | MethodType::X25519KeyAgreementKey2019 => {
        retrieve_public_key(&vault, location).await
      }
    }
  }

  async fn key_get(&self, did: &IotaDID, location: &KeyLocation) -> Result<PublicKey> {
    let vault: Vault<'_> = self.vault(did);

    match location.method() {
      MethodType::Ed25519VerificationKey2018 | MethodType::X25519KeyAgreementKey2019 => {
        retrieve_public_key(&vault, location).await
      }
    }
  }

  async fn key_del(&self, did: &IotaDID, location: &KeyLocation) -> Result<()> {
    let vault: Vault<'_> = self.vault(did);

    match location.method() {
      MethodType::Ed25519VerificationKey2018 | MethodType::X25519KeyAgreementKey2019 => {
        vault.delete(location_seed(location), false).await?;
        vault.delete(location_skey(location), false).await?;

        // TODO: Garbage Collection (?)
      }
    }

    Ok(())
  }

  async fn key_sign(&self, did: &IotaDID, location: &KeyLocation, data: Vec<u8>) -> Result<Signature> {
    let vault: Vault<'_> = self.vault(did);

    match location.method() {
      MethodType::Ed25519VerificationKey2018 => sign_ed25519(&vault, data, location).await,
      MethodType::X25519KeyAgreementKey2019 => Err(identity_did::Error::InvalidMethodType.into()),
    }
  }

  async fn key_exists(&self, did: &IotaDID, location: &KeyLocation) -> Result<bool> {
    let vault: Vault<'_> = self.vault(did);

    Ok(match location.method() {
      MethodType::Ed25519VerificationKey2018 | MethodType::X25519KeyAgreementKey2019 => {
        vault.exists(location_skey(location)).await
      }
    }?)
  }

  async fn chain_state(&self, did: &IotaDID) -> Result<Option<ChainState>> {
    // Load the chain-specific store
    let store: Store<'_> = self.store(&fmt_did(did));
    let data: Option<Vec<u8>> = store.get(location_chain_state()).await?;

    match data {
      None => return Ok(None),
      Some(data) => Ok(Some(ChainState::from_json_slice(&data)?)),
    }
  }

  async fn set_chain_state(&self, did: &IotaDID, chain_state: &ChainState) -> Result<()> {
    // Load the chain-specific store
    let store: Store<'_> = self.store(&fmt_did(did));

    let json: Vec<u8> = chain_state.to_json_vec()?;

    store.set(location_chain_state(), json, None).await?;

    Ok(())
  }

  async fn state(&self, did: &IotaDID) -> Result<Option<IdentityState>> {
    // Load the chain-specific store
    let store: Store<'_> = self.store(&fmt_did(did));

    // Read the state from the stronghold snapshot
    let data: Option<Vec<u8>> = store.get(location_state()).await?;

    match data {
      None => return Ok(None),
      Some(data) => Ok(Some(IdentityState::from_json_slice(&data)?)),
    }
  }

  async fn set_state(&self, did: &IotaDID, state: &IdentityState) -> Result<()> {
    // Load the chain-specific store
    let store: Store<'_> = self.store(&fmt_did(did));

    // Serialize the state
    let json: Vec<u8> = state.to_json_vec()?;

    // Write the state to the stronghold snapshot
    store.set(location_state(), json, None).await?;

    Ok(())
  }

  async fn purge(&self, _did: &IotaDID) -> Result<()> {
    // TODO: Will be re-implemented later with the key location refactor
    todo!("stronghold purge not implemented");
  }
}

impl Drop for Stronghold {
  fn drop(&mut self) {
    if self.dropsave {
      let _ = executor::block_on(self.flush_changes());
    }
  }
}

async fn generate_private_key(vault: &Vault<'_>, location: &KeyLocation) -> Result<PublicKey> {
  let key_type: iota_stronghold::procedures::KeyType = match location.method() {
    MethodType::Ed25519VerificationKey2018 => iota_stronghold::procedures::KeyType::Ed25519,
    MethodType::X25519KeyAgreementKey2019 => iota_stronghold::procedures::KeyType::X25519,
  };
  let procedure = GenerateKey {
    ty: key_type,
    output: location_skey(location),
    hint: default_hint(),
  };
  vault.execute(procedure).await?;

  // Retrieve the public key of the derived child key
  retrieve_public_key(vault, location).await
}

async fn retrieve_public_key(vault: &Vault<'_>, location: &KeyLocation) -> Result<PublicKey> {
  let procedure: iota_stronghold::procedures::PublicKey = iota_stronghold::procedures::PublicKey {
    ty: location_key_type(location),
    private_key: location_skey(location),
  };
  Ok(vault.execute(procedure).await.map(|public| public.to_vec().into())?)
}

async fn sign_ed25519(vault: &Vault<'_>, payload: Vec<u8>, location: &KeyLocation) -> Result<Signature> {
  let procedure: Ed25519Sign = Ed25519Sign {
    private_key: location_skey(location),
    msg: payload,
  };
  let signature: [u8; 64] = vault.execute(procedure).await?;
  Ok(Signature::new(signature.into()))
}

fn location_key_type(location: &KeyLocation) -> iota_stronghold::procedures::KeyType {
  match location.method() {
    MethodType::Ed25519VerificationKey2018 => iota_stronghold::procedures::KeyType::Ed25519,
    MethodType::X25519KeyAgreementKey2019 => iota_stronghold::procedures::KeyType::X25519,
  }
}

fn location_chain_state() -> Location {
  Location::generic("$chain_state", Vec::new())
}

fn location_state() -> Location {
  Location::generic("$state", Vec::new())
}

fn location_seed(location: &KeyLocation) -> Location {
  Location::generic(fmt_key("$seed", location), Vec::new())
}

fn location_skey(location: &KeyLocation) -> Location {
  Location::generic(fmt_key("$skey", location), Vec::new())
}

fn fmt_key(prefix: &str, location: &KeyLocation) -> Vec<u8> {
  format!("{}:{}:{}", prefix, location.generation(), location.fragment_name()).into_bytes()
}

fn fmt_did(did: &IotaDID) -> String {
  format!("$identity:{}", did.authority())
}
