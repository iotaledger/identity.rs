// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use core::iter::Map;
use core::pin::Pin;
use futures::stream;
use futures::stream::Iter;
use futures::stream::LocalBoxStream;
use futures::task::Context;
use futures::task::Poll;
use futures::Stream;
use futures::StreamExt;
use hashbrown::HashMap;
use identity_core::common::Value;
use identity_core::convert::FromJson;
use identity_core::convert::SerdeInto;
use identity_core::convert::ToJson;
use identity_core::crypto::Ed25519;
use identity_core::crypto::KeyPair;
use identity_core::crypto::KeyType;
use identity_core::crypto::PublicKey;
use identity_core::crypto::SecretKey;
use identity_core::crypto::Sign;
use identity_core::json;
use identity_core::utils::decode_b58;
use identity_core::utils::encode_b58;
use identity_did::verification::MethodType;
use std::fs;
use std::path::Path;
use std::sync::RwLockReadGuard;
use std::sync::RwLockWriteGuard;
use std::vec::IntoIter;

use crate::chain::ChainIndex;
use crate::chain::ChainKey;
use crate::error::Error;
use crate::error::Result;
use crate::events::Commit;
use crate::events::Snapshot;
use crate::storage::Storage;
use crate::types::ChainId;
use crate::types::Signature;
use crate::utils::EncryptionKey;
use crate::utils::Shared;

type MemChange = fn(Commit) -> Result<Commit>;
type MemStream = Iter<Map<IntoIter<Commit>, MemChange>>;

type MemVault = HashMap<ChainKey, KeyPair>;

pub struct MemStore {
  events: Shared<HashMap<ChainId, Vec<Commit>>>,
  chains: Shared<ChainIndex>,
  states: Shared<HashMap<ChainId, Snapshot>>,
  vaults: Shared<HashMap<ChainId, MemVault>>,
}

impl MemStore {
  pub fn new() -> Self {
    Self {
      events: Shared::new(HashMap::new()),
      chains: Shared::new(ChainIndex::new()),
      states: Shared::new(HashMap::new()),
      vaults: Shared::new(HashMap::new()),
    }
  }

  pub fn import_or_default<P: AsRef<Path> + ?Sized>(path: &P) -> Self {
    match Self::import(path) {
      Ok(this) => this,
      Err(_) => Self::new(),
    }
  }

  pub fn import<P: AsRef<Path> + ?Sized>(path: &P) -> Result<Self> {
    #[derive(Deserialize)]
    struct Key {
      #[serde(rename = "type")]
      type_: KeyType,
      public: String,
      secret: String,
    }

    fn parse_chain_id(chain: &str) -> Result<ChainId> {
      chain
        .get(2..)
        .and_then(|data| u32::from_str_radix(data, 16).ok())
        .ok_or(Error::InvalidChainId)
        .map(Into::into)
    }

    let data: Vec<u8> = fs::read(path)?;
    let json: Value = Value::from_json_slice(&data)?;

    let chains: ChainIndex = json["chains"].serde_into()?;
    let json_events: HashMap<String, Vec<Commit>> = json["events"].serde_into()?;
    let json_vaults: HashMap<String, HashMap<String, Key>> = json["vaults"].serde_into()?;

    let mut events: HashMap<ChainId, Vec<Commit>> = HashMap::new();
    let mut vaults: HashMap<ChainId, MemVault> = HashMap::new();

    for (chain, queue) in json_events {
      events.insert(parse_chain_id(&chain)?, queue);
    }

    for (chain, json_vault) in json_vaults {
      let chain: ChainId = parse_chain_id(&chain)?;
      let vault: &mut MemVault = vaults.entry(chain).or_default();

      for (key, keydata) in json_vault {
        let data: Vec<u8> = decode_b58(&key)?;
        let name: ChainKey = ChainKey::decode(&data).ok_or(Error::InvalidChainKey)?;

        let public: PublicKey = decode_b58(&keydata.public).map(Into::into)?;
        let secret: SecretKey = decode_b58(&keydata.secret).map(Into::into)?;
        let keypair: KeyPair = (keydata.type_, public, secret).into();

        vault.insert(name, keypair);
      }
    }

    Ok(Self {
      chains: Shared::new(chains),
      events: Shared::new(events),
      vaults: Shared::new(vaults),
      states: Shared::new(HashMap::new()),
    })
  }

  pub fn export<P: AsRef<Path> + ?Sized>(&self, path: &P) -> Result<()> {
    if let Some(parent) = path.as_ref().parent() {
      let _ = fs::create_dir_all(parent);
    }

    let mut json: Value = json!({
      "chains": &*self.chains.read()?,
      "vaults": {},
      "events": {},
    });

    for (chain, events) in self.events.read()?.iter() {
      json["events"][chain.to_string()] = events.serde_into()?;
    }

    for (chain, vault) in self.vaults.read()?.iter() {
      let data: &mut Value = &mut json["vaults"][chain.to_string()];

      for (key, keypair) in vault {
        data[encode_b58(&key.encode())] = json!({
          "type": keypair.type_(),
          "public": encode_b58(keypair.public()),
          "secret": encode_b58(keypair.secret()),
        });
      }
    }

    fs::write(path, json.to_json_pretty()?)?;

    Ok(())
  }
}

#[async_trait::async_trait(?Send)]
impl Storage for MemStore {
  async fn set_password(&self, _password: EncryptionKey) -> Result<()> {
    Ok(())
  }

  async fn flush_changes(&self) -> Result<()> {
    Ok(())
  }

  // TODO: Should ensure keys are unique across types
  async fn key_new(&self, chain: ChainId, location: &ChainKey) -> Result<PublicKey> {
    let mut vaults: RwLockWriteGuard<_> = self.vaults.write()?;
    let vault: &mut MemVault = vaults.entry(chain).or_default();

    match location.type_() {
      MethodType::Ed25519VerificationKey2018 => {
        let keypair: KeyPair = KeyPair::new_ed25519()?;
        let public: PublicKey = keypair.public().clone();

        vault.insert(location.clone(), keypair);

        Ok(public)
      }
      MethodType::MerkleKeyCollection2021 => {
        todo!("[MemStore::key_new] Handle MerkleKeyCollection2021")
      }
    }
  }

  async fn key_get(&self, chain: ChainId, location: &ChainKey) -> Result<PublicKey> {
    let vaults: RwLockReadGuard<_> = self.vaults.read()?;
    let vault: &MemVault = vaults.get(&chain).ok_or(Error::KeyVaultNotFound)?;
    let keypair: &KeyPair = vault.get(location).ok_or(Error::KeyPairNotFound)?;

    Ok(keypair.public().clone())
  }

  async fn key_del(&self, chain: ChainId, location: &ChainKey) -> Result<()> {
    let mut vaults: RwLockWriteGuard<_> = self.vaults.write()?;
    let vault: &mut MemVault = vaults.get_mut(&chain).ok_or(Error::KeyVaultNotFound)?;

    vault.remove(location);

    Ok(())
  }

  async fn key_sign(&self, chain: ChainId, location: &ChainKey, data: Vec<u8>) -> Result<Signature> {
    let vaults: RwLockReadGuard<_> = self.vaults.read()?;
    let vault: &MemVault = vaults.get(&chain).ok_or(Error::KeyVaultNotFound)?;
    let keypair: &KeyPair = vault.get(location).ok_or(Error::KeyPairNotFound)?;

    match location.type_() {
      MethodType::Ed25519VerificationKey2018 => {
        assert_eq!(keypair.type_(), KeyType::Ed25519);

        let public: PublicKey = keypair.public().clone();
        let signature: [u8; 64] = Ed25519::sign(&data, keypair.secret())?;
        let signature: Signature = Signature::new(public, signature.to_vec());

        Ok(signature)
      }
      MethodType::MerkleKeyCollection2021 => {
        todo!("[MemStore::key_sign] Handle MerkleKeyCollection2021")
      }
    }
  }

  async fn get_chain_index(&self) -> Result<ChainIndex> {
    self.chains.read().map(|index| index.clone())
  }

  async fn set_chain_index(&self, index: &ChainIndex) -> Result<()> {
    *self.chains.write()? = index.clone();

    Ok(())
  }

  async fn get_snapshot(&self, chain: ChainId) -> Result<Option<Snapshot>> {
    self.states.read().map(|states| states.get(&chain).cloned())
  }

  async fn set_snapshot(&self, chain: ChainId, snapshot: &Snapshot) -> Result<()> {
    self.states.write()?.insert(chain, snapshot.clone());

    Ok(())
  }

  async fn append(&self, chain: ChainId, commits: &[Commit]) -> Result<()> {
    let mut state: RwLockWriteGuard<_> = self.events.write()?;
    let queue: &mut Vec<Commit> = state.entry(chain).or_default();

    for commit in commits {
      queue.push(commit.clone());
    }

    Ok(())
  }

  async fn stream(&self, chain: ChainId) -> Result<LocalBoxStream<'_, Result<Commit>>> {
    let state: RwLockReadGuard<_> = self.events.read()?;
    let queue: Vec<Commit> = state.get(&chain).cloned().unwrap_or_default();

    Ok(EventStream::new(chain, queue).boxed_local())
  }
}

impl Debug for MemStore {
  #[cfg(feature = "mem-store-debug")]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.debug_struct("MemStore")
      .field("events", &self.events)
      .field("chains", &self.chains)
      .field("states", &self.states)
      .field("vaults", &self.vaults)
      .finish()
  }

  #[cfg(not(feature = "mem-store-debug"))]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.write_str("MemStore")
  }
}

// =============================================================================
// Event Stream
// =============================================================================

#[derive(Debug)]
pub struct EventStream {
  chain: ChainId,
  state: MemStream,
}

impl EventStream {
  pub fn new(chain: ChainId, commits: Vec<Commit>) -> Self {
    Self {
      chain,
      state: stream::iter(commits.into_iter().map(Result::Ok as MemChange)),
    }
  }

  pub fn chain(&self) -> ChainId {
    self.chain
  }
}

impl Stream for EventStream {
  type Item = Result<Commit>;

  fn poll_next(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Option<Self::Item>> {
    Pin::new(&mut self.state).poll_next(context)
  }
}
