// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use anyhow::Context;
use fastcrypto::ed25519::Ed25519PublicKey;
use fastcrypto::traits::ToFromBytes;
use identity_iota::iota::IotaDocument;
use identity_iota::iota::NetworkName;
use identity_iota::verification::jws::JwsAlgorithm;
use identity_iota::verification::MethodScope;
use identity_jose::jwk::Jwk;
use identity_storage::key_id_storage::KeyIdMemstore;
use identity_storage::key_storage::JwkMemStore;
use identity_storage::KeyIdStorage;
use identity_storage::Storage;
use identity_storage::{JwkDocumentExt, KeyType};
use identity_storage::{JwkStorage, KeyId, StorageSigner};
use identity_stronghold::StrongholdStorage;
use identity_sui_name_tbd::client::{IdentityClient, IdentityClientReadOnly};
use identity_sui_name_tbd::transaction::Transaction;
use iota_sdk::client::secret::stronghold::StrongholdSecretManager;
use iota_sdk::client::stronghold::StrongholdAdapter;
use iota_sdk::client::Password;
use iota_sdk_move::types::base_types::{IotaAddress, ObjectID};
use iota_sdk_move::IotaClientBuilder;
use rand::distributions::Alphanumeric;
use rand::distributions::DistString;
use rand::thread_rng;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::str::FromStr;
use tokio::net::TcpListener;
use tokio::process::Command;
use tokio::task::JoinHandle;
use tonic::transport::Uri;

pub const TEST_GAS_BUDGET: u64 = 50_000_000;

pub type MemStorage = Storage<JwkMemStore, KeyIdMemstore>;

pub const API_ENDPOINT: &str = "http://127.0.0.1:9000";
pub const FAUCET_ENDPOINT: &str = "http://localhost:9123/gas";

pub struct TestServer {
  client: IdentityClientReadOnly,
  addr: SocketAddr,
  _handle: JoinHandle<Result<(), tonic::transport::Error>>,
}

impl TestServer {
  pub async fn new() -> Self {
    let stronghold = StrongholdSecretManager::builder()
      .password(random_password(18))
      .build(random_stronghold_path())
      .map(StrongholdStorage::new)
      .expect("Failed to create temporary stronghold");

    Self::new_with_stronghold(stronghold).await
  }

  pub async fn new_with_stronghold(stronghold: StrongholdStorage) -> Self {
    let _ = tracing::subscriber::set_global_default(tracing_subscriber::fmt().compact().finish());

    let listener = TcpListener::bind("127.0.0.1:0")
      .await
      .expect("Failed to bind to random OS's port");
    let addr = listener.local_addr().unwrap();

    let iota_client = IotaClientBuilder::default()
      .build(API_ENDPOINT)
      .await
      .expect("Failed to connect to API's endpoint");

    let identity_pkg_id =
      ObjectID::from_str("0xc030a6ab95219bc1a669b222abb3f43692ec9c06e166ec4590630287364e017d").unwrap();

    let identity_client = IdentityClientReadOnly::new(iota_client, identity_pkg_id)
      .await
      .expect("Failed to build Identity client");

    let server = identity_grpc::server::GRpcServer::new(identity_client.clone(), stronghold)
      .into_router()
      .serve_with_incoming(tokio_stream::wrappers::TcpListenerStream::new(listener));
    TestServer {
      _handle: tokio::spawn(server),
      addr,
      client: identity_client,
    }
  }

  pub fn endpoint(&self) -> Uri {
    format!("https://{}", self.addr)
      .parse()
      .expect("Failed to parse server's URI")
  }

  pub fn client(&self) -> &IdentityClientReadOnly {
    &self.client
  }
}

pub async fn create_did<K, I>(
  client: &IdentityClientReadOnly,
  storage: &Storage<K, I>,
) -> anyhow::Result<(IotaAddress, IotaDocument, KeyId, String)>
where
  K: JwkStorage,
  I: KeyIdStorage,
{
  let (address, key_id, pub_key_jwk) = get_address(storage).await.context("failed to get address with funds")?;

  // Fund the account
  request_faucet_funds(address, FAUCET_ENDPOINT).await?;

  let signer = StorageSigner::new(storage, key_id.clone(), pub_key_jwk);

  let identity_client = IdentityClient::new(client.clone(), signer).await?;

  let network_name = client.network();
  let (document, fragment): (IotaDocument, String) = create_did_document(network_name, storage).await?;

  let document = identity_client
    .publish_did_document(document)
    .execute(&identity_client)
    .await?;

  Ok((address, document, key_id, fragment))
}

/// Creates an example DID document with the given `network_name`.
///
/// Its functionality is equivalent to the "create DID" example
/// and exists for convenient calling from the other examples.
pub async fn create_did_document<K, I>(
  network_name: &NetworkName,
  storage: &Storage<K, I>,
) -> anyhow::Result<(IotaDocument, String)>
where
  I: KeyIdStorage,
  K: JwkStorage,
{
  let mut document: IotaDocument = IotaDocument::new(network_name);

  let fragment: String = document
    .generate_method(
      storage,
      JwkMemStore::ED25519_KEY_TYPE,
      JwsAlgorithm::EdDSA,
      None,
      MethodScope::VerificationMethod,
    )
    .await?;

  Ok((document, fragment))
}

/// Generates a new Ed25519 key pair
pub async fn get_address<I, K>(storage: &Storage<K, I>) -> anyhow::Result<(IotaAddress, KeyId, Jwk)>
where
  K: JwkStorage,
  I: KeyIdStorage,
{
  let generated_key = storage
    .key_storage()
    .generate(KeyType::new("Ed25519"), JwsAlgorithm::EdDSA)
    .await?;

  let key_id = generated_key.key_id;

  let pub_key_jwt = generated_key.jwk.to_public().expect("should not fail");
  let pub_key_bytes = pub_key_jwt
    .try_okp_params()
    .map(|key| identity_jose::jwu::decode_b64(key.x.clone()).expect("should be decodeable"))?;

  let address = Ed25519PublicKey::from_bytes(&pub_key_bytes)?;

  Ok((IotaAddress::from(&address), key_id, pub_key_jwt))
}

/// Requests funds from the faucet for the given `address`.
async fn request_faucet_funds(address: IotaAddress, faucet_endpoint: &str) -> anyhow::Result<()> {
  let output = Command::new("iota")
    .arg("client")
    .arg("faucet")
    .arg("--address")
    .arg(address.to_string())
    .arg("--url")
    .arg(faucet_endpoint)
    .arg("--json")
    .output()
    .await
    .context("Failed to execute command")?;

  // Check if the output is success
  if !output.status.success() {
    anyhow::bail!(
      "Failed to request funds from faucet: {}",
      std::str::from_utf8(&output.stderr).unwrap()
    );
  }

  Ok(())
}

pub struct Entity<K, I> {
  storage: Storage<K, I>,
  did: Option<(IotaAddress, IotaDocument, KeyId, String)>,
}

pub fn random_password(len: usize) -> Password {
  let mut rng = thread_rng();
  Alphanumeric.sample_string(&mut rng, len).into()
}

pub fn random_stronghold_path() -> PathBuf {
  let mut file = std::env::temp_dir();
  file.push("test_strongholds");
  file.push(rand::distributions::Alphanumeric.sample_string(&mut rand::thread_rng(), 32));
  file.set_extension("stronghold");
  file.to_owned()
}

impl Default for Entity<JwkMemStore, KeyIdMemstore> {
  fn default() -> Self {
    let storage = MemStorage::new(JwkMemStore::new(), KeyIdMemstore::new());

    Self { storage, did: None }
  }
}

impl Entity<JwkMemStore, KeyIdMemstore> {
  pub fn new() -> Self {
    Self::default()
  }
}

impl Entity<StrongholdStorage, StrongholdStorage> {
  pub fn new_with_stronghold(s: StrongholdStorage) -> Self {
    let storage = Storage::new(s.clone(), s);

    Self { storage, did: None }
  }
}

impl<K: JwkStorage, I: KeyIdStorage> Entity<K, I> {
  pub async fn create_did(&mut self, client: &IdentityClientReadOnly) -> anyhow::Result<()> {
    let Entity { storage, did, .. } = self;

    *did = Some(create_did(client, storage).await?);

    Ok(())
  }

  pub fn storage(&self) -> &Storage<K, I> {
    &self.storage
  }

  pub fn document(&self) -> Option<&IotaDocument> {
    self.did.as_ref().map(|(_, doc, _, _)| doc)
  }

  pub fn fragment(&self) -> Option<&str> {
    self.did.as_ref().map(|(_, _, _, frag)| frag.as_ref())
  }
}

impl Entity<StrongholdStorage, StrongholdStorage> {
  pub async fn update_document<F>(&mut self, client: &IdentityClientReadOnly, f: F) -> anyhow::Result<()>
  where
    F: FnOnce(IotaDocument) -> Option<IotaDocument>,
  {
    let (address, doc, key_id, fragment) = self.did.take().context("Missing doc")?;
    let mut new_doc = f(doc.clone());
    if let Some(doc) = new_doc.take() {
      let Entity { storage, .. } = self;

      let public_key = storage.key_id_storage().get_public_key(&key_id).await?;
      let signer = StorageSigner::new(storage, key_id.clone(), public_key);

      let identity_client = IdentityClient::new(client.clone(), signer).await?;

      new_doc = Some(
        identity_client
          .publish_did_document_update(doc, TEST_GAS_BUDGET)
          .await?,
      );
    }

    self.did = Some((address, new_doc.unwrap_or(doc), key_id, fragment));

    Ok(())
  }
}

pub fn make_stronghold() -> StrongholdAdapter {
  StrongholdAdapter::builder()
    .password(random_password(18))
    .build(random_stronghold_path())
    .expect("Failed to create temporary stronghold")
}
