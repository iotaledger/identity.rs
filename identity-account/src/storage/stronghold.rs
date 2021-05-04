// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::ops::RangeFrom;
use crypto::keys::slip10::Chain;
use futures::future;
use futures::stream;
use futures::stream::BoxStream;
use futures::StreamExt;
use futures::TryStreamExt;
use hashbrown::HashSet;
use identity_core::convert::FromJson;
use identity_core::convert::ToJson;
use identity_core::crypto::PublicKey;
use identity_did::verification::MethodType;
use iota_stronghold::Location;
use iota_stronghold::SLIP10DeriveInput;
use std::path::Path;
use std::sync::Arc;

use crate::error::Error;
use crate::error::Result;
use crate::events::Commit;
use crate::events::Event;
use crate::events::EventData;
use crate::identity::IdentityId;
use crate::identity::IdentityIndex;
use crate::identity::IdentitySnapshot;
use crate::storage::Storage;
use crate::stronghold::default_hint;
use crate::stronghold::Snapshot;
use crate::stronghold::Store;
use crate::stronghold::Vault;
use crate::types::Generation;
use crate::types::KeyLocation;
use crate::types::Signature;
use crate::utils::derive_encryption_key;
use crate::utils::EncryptionKey;

// name of the metadata store
const META: &str = "$meta";

// event concurrency limit
const ECL: usize = 8;

// =============================================================================
// =============================================================================

#[derive(Debug)]
pub struct Stronghold {
  snapshot: Arc<Snapshot>,
}

impl Stronghold {
  pub async fn new<'a, T, U>(snapshot: &T, password: U) -> Result<Self>
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
    })
  }

  fn store(&self, name: &str) -> Store<'_> {
    self.snapshot.store(name, &[])
  }

  fn vault(&self, id: IdentityId) -> Vault<'_> {
    self.snapshot.vault(&fmt_id(id), &[])
  }
}

#[async_trait::async_trait]
impl Storage for Stronghold {
  async fn set_password(&self, password: EncryptionKey) -> Result<()> {
    self.snapshot.set_password(password)
  }

  async fn flush_changes(&self) -> Result<()> {
    self.snapshot.save().await
  }

  async fn key_new(&self, id: IdentityId, location: &KeyLocation) -> Result<PublicKey> {
    let vault: Vault<'_> = self.vault(id);

    let public: PublicKey = match location.method() {
      MethodType::Ed25519VerificationKey2018 => generate_ed25519(&vault, location).await?,
      MethodType::MerkleKeyCollection2021 => todo!("[Stronghold::key_new] Handle MerkleKeyCollection2021"),
    };

    Ok(public)
  }

  async fn key_get(&self, id: IdentityId, location: &KeyLocation) -> Result<PublicKey> {
    let vault: Vault<'_> = self.vault(id);

    match location.method() {
      MethodType::Ed25519VerificationKey2018 => retrieve_ed25519(&vault, location).await,
      MethodType::MerkleKeyCollection2021 => todo!("[Stronghold::key_get] Handle MerkleKeyCollection2021"),
    }
  }

  async fn key_del(&self, id: IdentityId, location: &KeyLocation) -> Result<()> {
    let vault: Vault<'_> = self.vault(id);

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

  async fn key_sign(&self, id: IdentityId, location: &KeyLocation, data: Vec<u8>) -> Result<Signature> {
    let vault: Vault<'_> = self.vault(id);

    match location.method() {
      MethodType::Ed25519VerificationKey2018 => sign_ed25519(&vault, data, location).await,
      MethodType::MerkleKeyCollection2021 => todo!("[Stronghold::key_sign] Handle MerkleKeyCollection2021"),
    }
  }

  async fn key_exists(&self, id: IdentityId, location: &KeyLocation) -> Result<bool> {
    let vault: Vault<'_> = self.vault(id);

    match location.method() {
      MethodType::Ed25519VerificationKey2018 => vault.exists(location_skey(location)).await,
      MethodType::MerkleKeyCollection2021 => todo!("[Stronghold::key_exists] Handle MerkleKeyCollection2021"),
    }
  }

  async fn index(&self) -> Result<IdentityIndex> {
    // Load the metadata actor
    let store: Store<'_> = self.store(META);

    // Read the index from the snapshot
    let data: Vec<u8> = store.get(location_index()).await?;

    // No index data (new snapshot)
    if data.is_empty() {
      return Ok(IdentityIndex::new());
    }

    // Deserialize and return
    Ok(IdentityIndex::from_json_slice(&data)?)
  }

  async fn set_index(&self, index: &IdentityIndex) -> Result<()> {
    // Load the metadata actor
    let store: Store<'_> = self.store(META);

    // Serialize the index
    let json: Vec<u8> = index.to_json_vec()?;

    // Write the index to the snapshot
    store.set(location_index(), json, None).await?;

    Ok(())
  }

  async fn snapshot(&self, id: IdentityId) -> Result<Option<IdentitySnapshot>> {
    // Load the chain-specific store
    let store: Store<'_> = self.store(&fmt_id(id));

    // Read the event snapshot from the stronghold snapshot
    let data: Vec<u8> = store.get(location_snapshot()).await?;

    // No snapshot data found
    if data.is_empty() {
      return Ok(None);
    }

    // Deserialize and return
    Ok(Some(IdentitySnapshot::from_json_slice(&data)?))
  }

  async fn set_snapshot(&self, id: IdentityId, snapshot: &IdentitySnapshot) -> Result<()> {
    // Load the chain-specific store
    let store: Store<'_> = self.store(&fmt_id(id));

    // Serialize the state snapshot
    let json: Vec<u8> = snapshot.to_json_vec()?;

    // Write the state snapshot to the stronghold snapshot
    store.set(location_snapshot(), json, None).await?;

    Ok(())
  }

  async fn append(&self, id: IdentityId, commits: &[Commit]) -> Result<()> {
    fn encode(commit: &Commit) -> Result<(Generation, Vec<u8>)> {
      Ok((commit.sequence(), commit.event().to_json_vec()?))
    }

    let store: Store<'_> = self.store(&fmt_id(id));

    let future: _ = stream::iter(commits.iter().map(encode))
      .into_stream()
      .and_then(|(index, json)| store.set(location_event(index), json, None))
      .try_for_each_concurrent(ECL, |()| future::ready(Ok(())));

    // Write all events to the snapshot
    future.await?;

    Ok(())
  }

  async fn stream(&self, id: IdentityId, index: Generation) -> Result<BoxStream<'_, Result<Commit>>> {
    let name: String = fmt_id(id);
    let range: RangeFrom<u32> = (index.to_u32() + 1)..;

    let stream: BoxStream<'_, Result<Commit>> = stream::iter(range)
      .map(Generation::from)
      .map(Ok)
      // ================================
      // Load the event from the snapshot
      // ================================
      .and_then(move |index| {
        let name: String = name.clone();
        let snap: Arc<Snapshot> = Arc::clone(&self.snapshot);

        async move {
          let location: Location = location_event(index);
          let store: Store<'_> = snap.store(&name, &[]);
          let event: Vec<u8> = store.get(location).await?;

          Ok((index, event))
        }
      })
      // ================================
      // Parse the event
      // ================================
      .try_filter_map(move |(index, json)| async move {
        if json.is_empty() {
          Err(Error::EventNotFound)
        } else {
          let event: Event = Event::from_json_slice(&json)?;
          let commit: Commit = Commit::new(id, index, event);

          Ok(Some(commit))
        }
      })
      // ================================
      // Downcast to "traditional" stream
      // ================================
      .into_stream()
      // ================================
      // Bail on any invalid event
      // ================================
      .take_while(|event| future::ready(event.is_ok()))
      // ================================
      // Create a boxed stream
      // ================================
      .boxed();

    Ok(stream)
  }

  async fn purge(&self, id: IdentityId) -> Result<()> {
    type PurgeSet = (Generation, HashSet<KeyLocation>);

    async fn fold(mut output: PurgeSet, commit: Commit) -> Result<PurgeSet> {
      if let EventData::MethodCreated(_, method) = commit.event().data() {
        output.1.insert(method.location().clone());
      }

      let gen_a: u32 = commit.sequence().to_u32();
      let gen_b: u32 = output.0.to_u32();

      Ok((Generation::from_u32(gen_a.max(gen_b)), output.1))
    }

    // Load the chain-specific store/vault
    let store: Store<'_> = self.store(&fmt_id(id));
    let vault: Vault<'_> = self.vault(id);

    // Scan the event stream and collect a set of all key locations
    let output: (Generation, HashSet<KeyLocation>) = self
      .stream(id, Generation::new())
      .await?
      .try_fold((Generation::new(), HashSet::new()), fold)
      .await?;

    // Remove the state snapshot
    store.del(location_snapshot()).await?;

    // Remove all events
    for index in 0..output.0.to_u32() {
      store.del(location_event(Generation::from_u32(index))).await?;
    }

    // Remove all keys
    for location in output.1 {
      match location.method() {
        MethodType::Ed25519VerificationKey2018 => {
          vault.delete(location_seed(&location), false).await?;
          vault.delete(location_skey(&location), false).await?;
        }
        MethodType::MerkleKeyCollection2021 => {
          todo!("[Stronghold::purge] Handle MerkleKeyCollection2021")
        }
      }
    }

    Ok(())
  }
}

async fn generate_ed25519(vault: &Vault<'_>, location: &KeyLocation) -> Result<PublicKey> {
  // Generate a SLIP10 seed as the private key
  vault
    .slip10_generate(location_seed(location), default_hint(), None)
    .await?;

  let chain: Chain = Chain::from_u32_hardened(vec![0, 0, 0]);
  let seed: SLIP10DeriveInput = SLIP10DeriveInput::Seed(location_seed(location));

  // Use the SLIP10 seed to derive a child key
  vault
    .slip10_derive(chain, seed, location_skey(location), default_hint())
    .await?;

  // Retrieve the public key of the derived child key
  retrieve_ed25519(vault, location).await
}

async fn retrieve_ed25519(vault: &Vault<'_>, location: &KeyLocation) -> Result<PublicKey> {
  vault
    .ed25519_public_key(location_skey(location))
    .await
    .map(|public| public.to_vec().into())
}

async fn sign_ed25519(vault: &Vault<'_>, payload: Vec<u8>, location: &KeyLocation) -> Result<Signature> {
  let public_key: PublicKey = retrieve_ed25519(&vault, location).await?;
  let signature: [u8; 64] = vault.ed25519_sign(payload, location_skey(location)).await?;

  Ok(Signature::new(public_key, signature.into()))
}

fn location_index() -> Location {
  Location::generic("$index", Vec::new())
}

fn location_snapshot() -> Location {
  Location::generic("$snapshot", Vec::new())
}

fn location_event(index: Generation) -> Location {
  Location::generic(format!("$event:{}", index), Vec::new())
}

fn location_seed(location: &KeyLocation) -> Location {
  Location::generic(fmt_key("$seed", location), Vec::new())
}

fn location_skey(location: &KeyLocation) -> Location {
  Location::generic(fmt_key("$skey", location), Vec::new())
}

fn fmt_key(prefix: &str, location: &KeyLocation) -> Vec<u8> {
  format!(
    "{}:{}:{}:{}",
    prefix,
    location.auth_generation(),
    location.diff_generation(),
    location.fragment(),
  )
  .into_bytes()
}

fn fmt_id(id: IdentityId) -> String {
  format!("$identity:{}", id)
}
