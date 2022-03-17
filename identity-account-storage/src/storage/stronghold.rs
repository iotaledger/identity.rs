// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::path::Path;
use std::sync::Arc;

use async_trait::async_trait;
use futures::executor;
use identity_core::convert::FromJson;
use identity_core::convert::ToJson;
use identity_core::crypto::KeyType;
use identity_core::crypto::PrivateKey;
use identity_core::crypto::PublicKey;
use identity_did::did::DID;
use identity_did::verification::MethodType;
use identity_iota_core::did::IotaDID;
use identity_iota_core::document::IotaVerificationMethod;
use iota_stronghold::procedures;
use iota_stronghold::Location;

use crate::error::Result;
use crate::identity::ChainState;
use crate::identity::IdentityState;
use crate::storage::Storage;
use crate::stronghold::default_hint;
use crate::stronghold::IotaStrongholdResult;
use crate::stronghold::Snapshot;
use crate::stronghold::Store;
use crate::stronghold::Vault;
use crate::types::KeyLocation;
use crate::types::KeyLocation2;
use crate::types::Signature;
use crate::utils::derive_encryption_key;
use crate::utils::EncryptionKey;

#[derive(Debug)]
pub struct Stronghold {
  snapshot: Arc<Snapshot>,
  dropsave: bool,
}

impl Stronghold {
  pub async fn new<'a, T, U>(snapshot: &T, password: U, dropsave: Option<bool>) -> Result<Self>
  where
    T: AsRef<Path> + ?Sized,
    U: Into<Option<&'a str>>,
  {
    let snapshot: Snapshot = Snapshot::new(snapshot);

    if let Some(password) = password.into() {
      snapshot.load(derive_encryption_key(password)).await?;
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
  async fn set_password(&self, password: EncryptionKey) -> Result<()> {
    self.snapshot.set_password(password).await?;
    Ok(())
  }

  async fn flush_changes(&self) -> Result<()> {
    self.snapshot.save().await?;
    Ok(())
  }

  async fn key_new(&self, did: &IotaDID, fragment: &str, method_type: MethodType) -> Result<KeyLocation2> {
    let vault: Vault<'_> = self.vault(did);

    let location: KeyLocation2 = match method_type {
      MethodType::Ed25519VerificationKey2018 => generate_ed25519(&vault, method_type, fragment, did).await?,
      MethodType::MerkleKeyCollection2021 => todo!("[Stronghold::key_new] Handle MerkleKeyCollection2021"),
    };

    Ok(location)
  }

  // TODO: Should this also not return the pub key since the caller could have just calculated it themselves,
  // and to be consistent with key_new?
  async fn key_insert(&self, did: &IotaDID, location: &KeyLocation2, private_key: PrivateKey) -> Result<()> {
    let vault = self.vault(did);

    let stronghold_location: Location = location.to_location(did);

    vault
      .insert(stronghold_location.clone(), private_key.as_ref(), default_hint(), &[])
      .await?;

    Ok(())
  }

  async fn key_get(&self, did: &IotaDID, location: &KeyLocation2) -> Result<PublicKey> {
    let vault: Vault<'_> = self.vault(did);

    match location.method() {
      MethodType::Ed25519VerificationKey2018 => retrieve_ed25519(&vault, location.to_location(did)).await,
      MethodType::MerkleKeyCollection2021 => todo!("[Stronghold::key_get] Handle MerkleKeyCollection2021"),
    }
  }

  async fn key_del(&self, did: &IotaDID, location: &KeyLocation) -> Result<()> {
    let vault: Vault<'_> = self.vault(did);

    match location.method() {
      MethodType::Ed25519VerificationKey2018 => {
        vault.delete(location_seed(location), false).await?;
        vault.delete(location_skey(location), false).await?;

        // TODO: Garbage Collection (?)
      }
      MethodType::MerkleKeyCollection2021 => todo!("[Stronghold::key_del] Handle MerkleKeyCollection2021"),
    }

    Ok(())
  }

  async fn key_sign(&self, did: &IotaDID, location: &KeyLocation, data: Vec<u8>) -> Result<Signature> {
    let vault: Vault<'_> = self.vault(did);

    match location.method() {
      MethodType::Ed25519VerificationKey2018 => sign_ed25519(&vault, data, location).await,
      MethodType::MerkleKeyCollection2021 => todo!("[Stronghold::key_sign] Handle MerkleKeyCollection2021"),
    }
  }

  async fn key_exists(&self, did: &IotaDID, location: &KeyLocation) -> Result<bool> {
    let vault: Vault<'_> = self.vault(did);

    Ok(match location.method() {
      MethodType::Ed25519VerificationKey2018 => vault.exists(location_skey(location)).await,
      MethodType::MerkleKeyCollection2021 => todo!("[Stronghold::key_exists] Handle MerkleKeyCollection2021"),
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

async fn generate_ed25519(
  vault: &Vault<'_>,
  method_type: MethodType,
  fragment: &str,
  did: &IotaDID,
) -> Result<KeyLocation2> {
  let random: Vec<u8> = random()?.to_vec();
  let did_string: String = did.to_string();
  let seed_location: Location = Location::generic(format!("tmp_seed:{}", did_string), random.clone());
  let key_location: Location = Location::generic(format!("tmp_key:{}", did_string), random);

  // Generate a SLIP10 seed as the private key
  let procedure: procedures::Slip10Generate = procedures::Slip10Generate {
    output: seed_location.clone(),
    hint: default_hint(),
    size_bytes: None,
  };
  vault.execute(procedure).await?;

  let chain: procedures::Chain = procedures::Chain::from_u32_hardened(vec![0, 0, 0]);
  let seed: procedures::Slip10DeriveInput = procedures::Slip10DeriveInput::Seed(seed_location);

  // Use the SLIP10 seed to derive a child key
  let procedure: procedures::Slip10Derive = procedures::Slip10Derive {
    chain,
    input: seed,
    output: key_location.clone(),
    hint: default_hint(),
  };
  vault.execute(procedure).await?;

  // Retrieve the public key of the derived child key
  let public_key: PublicKey = retrieve_ed25519(vault, key_location.clone()).await?;

  let method = IotaVerificationMethod::new(did.clone(), KeyType::Ed25519, &public_key, fragment)?;
  let new_location = KeyLocation2::new(method_type, fragment.to_owned(), method.key_data());

  move_key(vault, key_location, new_location.to_location(did)).await?;

  Ok(new_location)
}

async fn retrieve_ed25519(vault: &Vault<'_>, location: Location) -> Result<PublicKey> {
  let procedure: procedures::PublicKey = procedures::PublicKey {
    ty: procedures::KeyType::Ed25519,
    private_key: location,
  };
  Ok(vault.execute(procedure).await.map(|public| public.to_vec().into())?)
}

async fn sign_ed25519(vault: &Vault<'_>, payload: Vec<u8>, location: &KeyLocation) -> Result<Signature> {
  // TODO: Fix location.
  let public_key: PublicKey = retrieve_ed25519(vault, Location::generic(vec![], vec![])).await?;
  let procedure: procedures::Ed25519Sign = procedures::Ed25519Sign {
    private_key: location_skey(location),
    msg: payload,
  };
  let signature: [u8; 64] = vault.execute(procedure).await?;

  Ok(Signature::new(public_key, signature.into()))
}

// Moves a key from one location to another, thus deleting the old one.
async fn move_key(vault: &Vault<'_>, from: Location, to: Location) -> Result<()> {
  let copy_record = procedures::CopyRecord {
    source: from.clone(),
    target: to,
    hint: default_hint(),
  };

  vault.execute(copy_record).await?;

  let revoke_data = procedures::RevokeData {
    location: from,
    should_gc: true,
  };

  vault.execute(revoke_data).await?;

  Ok(())
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

fn random() -> IotaStrongholdResult<[u8; 64]> {
  let mut bytes: [u8; 64] = [0; 64];
  crypto::utils::rand::fill(&mut bytes)
    .map_err(|err| crate::stronghold::StrongholdError::IoError(std::io::Error::new(std::io::ErrorKind::Other, err)))?;
  Ok(bytes)
}

fn fmt_key(prefix: &str, location: &KeyLocation) -> Vec<u8> {
  format!("{}:{}:{}", prefix, location.generation(), location.fragment_name()).into_bytes()
}

fn fmt_did(did: &IotaDID) -> String {
  format!("$identity:{}", did.authority())
}

impl KeyLocation2 {
  fn to_location(&self, did: &IotaDID) -> Location {
    let bytes: Vec<u8> = format!("{}:{}", self.fragment, self.key_hash).into_bytes();
    Location::generic(did.to_string(), bytes)
  }
}
