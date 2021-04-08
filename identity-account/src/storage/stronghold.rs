// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::ops::RangeFrom;
use crypto::keys::slip10::Chain;
use futures::future;
use futures::stream;
use futures::stream::Iter;
use futures::stream::LocalBoxStream;
use futures::stream::Map;
use futures::StreamExt;
use futures::TryStreamExt;
use identity_core::convert::FromJson;
use identity_core::convert::ToJson;
use identity_core::crypto::PublicKey;
use identity_did::verification::MethodType;
use iota_stronghold::Location;
use iota_stronghold::SLIP10DeriveInput;
use std::path::Path;
use std::sync::Arc;

use crate::chain::ChainIndex;
use crate::chain::ChainKey;
use crate::error::Error;
use crate::error::Result;
use crate::events::Commit;
use crate::events::Snapshot as EventSnapshot;
use crate::storage::Storage;
use crate::stronghold::default_hint;
use crate::stronghold::Snapshot;
use crate::stronghold::Store;
use crate::stronghold::Vault;
use crate::types::ChainId;
use crate::types::Index;
use crate::types::Signature;
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
  pub async fn new<T, U>(snapshot: &T, password: U) -> Result<Self>
  where
    T: AsRef<Path> + ?Sized,
    U: Into<Option<EncryptionKey>>,
  {
    let snapshot: Snapshot = Snapshot::new(snapshot);

    if let Some(password) = password.into() {
      snapshot.load(password).await?;
    }

    Ok(Self {
      snapshot: Arc::new(snapshot),
    })
  }

  fn store(&self, name: &str) -> Store<'_> {
    self.snapshot.store(name, &[])
  }

  fn vault(&self, chain: ChainId) -> Vault<'_> {
    self.snapshot.vault(&fmt_chain(chain), &[])
  }
}

#[async_trait::async_trait(?Send)]
impl Storage for Stronghold {
  async fn set_password(&self, password: EncryptionKey) -> Result<()> {
    self.snapshot.set_password(password)
  }

  async fn flush_changes(&self) -> Result<()> {
    self.snapshot.save().await
  }

  async fn key_new(&self, chain: ChainId, location: &ChainKey) -> Result<PublicKey> {
    // Load the chain-specific vault
    let vault: Vault<'_> = self.vault(chain);

    let public: PublicKey = match location.type_() {
      MethodType::Ed25519VerificationKey2018 => generate_ed25519(&vault, location).await?,
      MethodType::MerkleKeyCollection2021 => todo!("[Stronghold::key_new] Handle MerkleKeyCollection2021"),
    };

    Ok(public)
  }

  async fn key_get(&self, chain: ChainId, location: &ChainKey) -> Result<PublicKey> {
    // Load the chain-specific vault
    let vault: Vault<'_> = self.vault(chain);

    match location.type_() {
      MethodType::Ed25519VerificationKey2018 => retrieve_ed25519(&vault, location).await,
      MethodType::MerkleKeyCollection2021 => todo!("[Stronghold::key_get] Handle MerkleKeyCollection2021"),
    }
  }

  async fn key_del(&self, chain: ChainId, location: &ChainKey) -> Result<()> {
    // Load the chain-specific vault
    let vault: Vault<'_> = self.vault(chain);

    match location.type_() {
      MethodType::Ed25519VerificationKey2018 => {
        vault.delete(location_seed(location), false).await?;
        vault.delete(location_skey(location), false).await?;
        // TODO: Garbage Collection (?)
      }
      MethodType::MerkleKeyCollection2021 => todo!("[Stronghold::key_del] Handle MerkleKeyCollection2021"),
    }

    Ok(())
  }

  async fn key_sign(&self, chain: ChainId, location: &ChainKey, data: Vec<u8>) -> Result<Signature> {
    let vault: Vault<'_> = self.vault(chain);

    match location.type_() {
      MethodType::Ed25519VerificationKey2018 => sign_ed25519(&vault, data, location).await,
      MethodType::MerkleKeyCollection2021 => todo!("[Stronghold::key_sign] Handle MerkleKeyCollection2021"),
    }
  }

  async fn get_chain_index(&self) -> Result<ChainIndex> {
    // Load the metadata actor
    let store: Store<'_> = self.store(META);

    // Read the index from the snapshot
    let data: Vec<u8> = store.get(location_index()).await?;

    // No index data (new snapshot)
    if data.is_empty() {
      return Ok(ChainIndex::new());
    }

    // Deserialize and return
    Ok(ChainIndex::from_json_slice(&data)?)
  }

  async fn set_chain_index(&self, index: &ChainIndex) -> Result<()> {
    // Load the metadata actor
    let store: Store<'_> = self.store(META);

    // Serialize the index
    let json: Vec<u8> = index.to_json_vec()?;

    // Write the index to the snapshot
    store.set(location_index(), json, None).await?;

    Ok(())
  }

  async fn get_snapshot(&self, chain: ChainId) -> Result<Option<EventSnapshot>> {
    // Load the chain-specific store
    let store: Store<'_> = self.store(&fmt_chain(chain));

    // Read the event snapshot from the stronghold snapshot
    let data: Vec<u8> = store.get(location_snapshot()).await?;

    // No snapshot data found
    if data.is_empty() {
      return Ok(None);
    }

    // Deserialize and return
    Ok(Some(EventSnapshot::from_json_slice(&data)?))
  }

  async fn set_snapshot(&self, chain: ChainId, snapshot: &EventSnapshot) -> Result<()> {
    // Load the chain-specific store
    let store: Store<'_> = self.store(&fmt_chain(chain));

    // Serialize the state snapshot
    let json: Vec<u8> = snapshot.to_json_vec()?;

    // Write the state snapshot to the stronghold snapshot
    store.set(location_snapshot(), json, None).await?;

    Ok(())
  }

  async fn append(&self, chain: ChainId, commits: &[Commit]) -> Result<()> {
    fn encode(commit: &Commit) -> Result<(Index, Vec<u8>)> {
      Ok((commit.index(), commit.to_json_vec()?))
    }

    let store: Store<'_> = self.store(&fmt_chain(chain));

    let future: _ = stream::iter(commits.into_iter().map(encode))
      .into_stream()
      .and_then(|(index, json)| store.set(location_event(index), json, None))
      .try_for_each_concurrent(ECL, |()| future::ready(Ok(())));

    // Write all events to the snapshot
    future.await?;

    Ok(())
  }

  async fn stream(&self, chain: ChainId, version: Index) -> Result<LocalBoxStream<'_, Result<Commit>>> {
    type IterA = Iter<RangeFrom<u32>>;
    type IterB = Map<IterA, fn(u32) -> Index>;
    type IterC = Map<IterB, fn(Index) -> Result<Index>>;

    let name: String = fmt_chain(chain);

    let iter: IterA = stream::iter((version.to_u32()..).into_iter());
    let iter: IterB = iter.map(Index::from);
    let iter: IterC = iter.map(Ok);

    let stream: LocalBoxStream<'_, Result<Commit>> = iter
      // ================================
      // Load the event from the snapshot
      // ================================
      .and_then(move |index| {
        let name: String = name.clone();
        let snap: Arc<Snapshot> = Arc::clone(&self.snapshot);

        async move { snap.store(&name, &[]).get(location_event(index)).await }
      })
      // ================================
      // Parse the event
      // ================================
      .try_filter_map(|data| async move {
        if data.is_empty() {
          Err(Error::ChainEventNotFound)
        } else {
          Commit::from_json_slice(&data).map(Some).map_err(Into::into)
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
      // Create a boxed version
      // ================================
      .boxed_local();

    Ok(stream)
  }
}

async fn generate_ed25519(vault: &Vault<'_>, location: &ChainKey) -> Result<PublicKey> {
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

async fn retrieve_ed25519(vault: &Vault<'_>, location: &ChainKey) -> Result<PublicKey> {
  vault
    .ed25519_public_key(location_skey(location))
    .await
    .map(|public| public.to_vec().into())
}

async fn sign_ed25519(vault: &Vault<'_>, payload: Vec<u8>, location: &ChainKey) -> Result<Signature> {
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

fn location_event(index: Index) -> Location {
  Location::generic(format!("$event:{}", index), Vec::new())
}

fn location_seed(location: &ChainKey) -> Location {
  Location::generic("$vault:seed", fmt_key(location))
}

fn location_skey(location: &ChainKey) -> Location {
  Location::generic("$vault:skey", fmt_key(location))
}

fn fmt_key(location: &ChainKey) -> Vec<u8> {
  format!(
    "{}-{}-{}",
    location.auth_index(),
    location.diff_index(),
    location.fragment(),
  )
  .into_bytes()
}

fn fmt_chain(chain: ChainId) -> String {
  format!("$chain:{}", chain)
}
