// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::ops::Deref;
use std::pin::Pin;
use std::str::FromStr;

use fastcrypto::ed25519::Ed25519PublicKey;
use fastcrypto::hash::HashFunction;
use fastcrypto::traits::ToFromBytes;
use futures::stream::FuturesUnordered;
use futures::Future;
use futures::TryStreamExt;
use identity_iota_core::block::output::AliasId;
use identity_iota_core::IotaDID;
use identity_iota_core::IotaDocument;
use identity_iota_core::NetworkName;
use identity_iota_core::StateMetadataDocument;
use identity_verification::jwk::Jwk;
use iota_sdk::rpc_types::IotaTransactionBlockResponse;
use iota_sdk::rpc_types::IotaTransactionBlockResponseOptions;
use iota_sdk::types::base_types::IotaAddress;
use iota_sdk::types::base_types::ObjectID;
use iota_sdk::types::crypto::DefaultHash;
use iota_sdk::types::crypto::Signature;
use iota_sdk::types::crypto::SignatureScheme;
use iota_sdk::types::quorum_driver_types::ExecuteTransactionRequestType;
use iota_sdk::types::transaction::ProgrammableTransaction;
use iota_sdk::types::transaction::Transaction;
use iota_sdk::types::transaction::TransactionData;
use iota_sdk::IotaClient;
use secret_storage::key_signature_set::KeySignatureTypes;
use secret_storage::signer::Signer;
use shared_crypto::intent::Intent;
use shared_crypto::intent::IntentMessage;

use crate::migration::get_alias;
use crate::migration::get_identity;
use crate::migration::lookup;
use crate::migration::Identity;
use crate::migration::IdentityBuilder;
use crate::Error;

const DEFAULT_NETWORK_NAME: &str = "iota";
const DEFAULT_IDENTITY_PACKAGE_ID: &str = "0x4342dbbd222a5b1a369afaa19defd5a121252bb89bccb611c72d9127a90cfaca";

pub struct KinesisKeySignature {
  pub public_key: Vec<u8>,
  pub signature: Vec<u8>,
}

impl KeySignatureTypes for KinesisKeySignature {
  type PublicKey = Vec<u8>;
  type Signature = Vec<u8>;
}

/// Mirrored types from identity_storage::KeyId
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct KeyId(String);

impl KeyId {
  /// Creates a new key identifier from a string.
  pub fn new(id: impl Into<String>) -> Self {
    Self(id.into())
  }

  /// Returns string representation of the key id.
  pub fn as_str(&self) -> &str {
    &self.0
  }
}

impl std::fmt::Display for KeyId {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(&self.0)
  }
}

impl From<KeyId> for String {
  fn from(value: KeyId) -> Self {
    value.0
  }
}

#[derive(Default)]
pub struct IdentityClientBuilder {
  pub(crate) identity_iota_package_id: Option<ObjectID>,
  pub(crate) sender_public_key: Option<Vec<u8>>,
  pub(crate) iota_client: Option<IotaClient>,
  pub(crate) network_name: Option<String>,
}

impl IdentityClientBuilder {
  /// Sets the `identity_iota_package_id` value.
  #[must_use]
  pub fn identity_iota_package_id(mut self, value: ObjectID) -> Self {
    self.identity_iota_package_id = Some(value);
    self
  }

  /// Sets the `sender_public_key` value.
  #[must_use]
  pub fn sender_public_key(mut self, value: &[u8]) -> Self {
    self.sender_public_key = Some(value.into());
    self
  }

  /// Sets the `iota_client` value.
  #[must_use]
  pub fn iota_client(mut self, value: IotaClient) -> Self {
    self.iota_client = Some(value);
    self
  }

  /// Sets the `network_name` value.
  #[must_use]
  pub fn network_name(mut self, value: &str) -> Self {
    self.network_name = Some(value.to_string());
    self
  }

  /// Returns a new `IdentityClientBuilder` based on the `IdentityClientBuilder` configuration.
  pub fn build(self) -> Result<IdentityClient, Error> {
    IdentityClient::from_builder(self)
  }
}

#[derive(Clone)]
struct SigningInfo {
  #[allow(dead_code)]
  sender_public_key: Vec<u8>,
  sender_address: IotaAddress,
}

#[derive(Clone)]
pub struct IdentityClient {
  identity_iota_package_id: ObjectID,
  iota_client: IotaClient,
  signing_info: Option<SigningInfo>,
  network_name: NetworkName,
}

impl Deref for IdentityClient {
  type Target = IotaClient;
  fn deref(&self) -> &Self::Target {
    &self.iota_client
  }
}

impl IdentityClient {
  pub(crate) fn package_id(&self) -> ObjectID {
    self.identity_iota_package_id
  }

  pub fn builder() -> IdentityClientBuilder {
    IdentityClientBuilder::default()
  }

  pub fn sender_public_key(&self) -> Result<&[u8], Error> {
    self
      .signing_info
      .as_ref()
      .ok_or_else(|| Error::InvalidConfig("public key for sender not set".to_string()))
      .map(|v| v.sender_public_key.as_ref())
  }

  pub fn sender_address(&self) -> Result<IotaAddress, Error> {
    self
      .signing_info
      .as_ref()
      .ok_or_else(|| Error::InvalidConfig("public key for sender not set".to_string()))
      .map(|v| v.sender_address)
  }

  pub fn network_name(&self) -> &NetworkName {
    &self.network_name
  }

  /// Returns a new `CoreDocument` based on the [`DocumentBuilder`] configuration.
  pub fn from_builder(builder: IdentityClientBuilder) -> Result<Self, Error> {
    let signing_info = builder
      .sender_public_key
      .map(|sender_public_key| {
        convert_to_address(&sender_public_key).map(|sender_address| SigningInfo {
          sender_public_key,
          sender_address,
        })
      })
      .transpose()?;
    let network_name = builder.network_name.unwrap_or_else(|| DEFAULT_NETWORK_NAME.to_string());
    let network_name = network_name
      .clone()
      .try_into()
      .map_err(|err| Error::InvalidConfig(format!(r#"could not convert "{network_name}" to a NetworkName; {err}"#)))?;

    Ok(Self {
      identity_iota_package_id: builder
        .identity_iota_package_id
        .map_or_else(|| ObjectID::from_str(DEFAULT_IDENTITY_PACKAGE_ID), Ok)
        .map_err(|err| Error::InvalidConfig(format!("could not parse identity package id; {err}")))?,
      iota_client: builder
        .iota_client
        .ok_or_else(|| Error::InvalidConfig("missing `iota_client` argument ".to_string()))?,
      signing_info,
      network_name,
    })
  }

  /// Creates a new onchain Identity.
  pub fn create_identity<'a>(&self, iota_document: &'a [u8]) -> IdentityBuilder<'a> {
    IdentityBuilder::new(iota_document, self.identity_iota_package_id)
  }

  pub(crate) async fn execute_transaction<S>(
    &self,
    tx: ProgrammableTransaction,
    gas_budget: u64,
    signer: &S,
  ) -> Result<IotaTransactionBlockResponse, Error>
  where
    S: Signer<KinesisKeySignature> + Send + Sync,
  {
    let tx_data = self.get_transaction_data(tx, gas_budget).await?;
    let kinesis_signature = self.sign_transaction_data(&tx_data, signer).await?;

    // execute tx
    self
      .quorum_driver_api()
      .execute_transaction_block(
        Transaction::from_data(tx_data, vec![kinesis_signature]),
        IotaTransactionBlockResponseOptions::full_content(),
        Some(ExecuteTransactionRequestType::WaitForLocalExecution),
      )
      .await
      .map_err(Error::TransactionExecutionFailed)
  }

  async fn get_coin_for_transaction(&self) -> Result<iota_sdk::rpc_types::Coin, Error> {
    let coins = self
      .iota_client
      .coin_read_api()
      .get_coins(self.sender_address()?, None, None, None)
      .await
      .map_err(|err| Error::GasIssue(format!("could not get coins; {err}")))?;

    coins
      .data
      .into_iter()
      .next()
      .ok_or_else(|| Error::GasIssue("could not find coins".to_string()))
  }

  async fn get_transaction_data(
    &self,
    programmable_transaction: ProgrammableTransaction,
    gas_budget: u64,
  ) -> Result<TransactionData, Error> {
    let gas_price = self
      .iota_client
      .read_api()
      .get_reference_gas_price()
      .await
      .map_err(|err| Error::GasIssue(format!("could not get gas price; {err}")))?;
    let coin = self.get_coin_for_transaction().await?;
    let tx_data = TransactionData::new_programmable(
      self.sender_address()?,
      vec![coin.object_ref()],
      programmable_transaction,
      gas_budget,
      gas_price,
    );

    Ok(tx_data)
  }

  async fn sign_transaction_data<S>(&self, tx_data: &TransactionData, signer: &S) -> Result<Signature, Error>
  where
    S: Signer<KinesisKeySignature> + Send + Sync,
  {
    let SigningInfo { sender_public_key, .. } = self
      .signing_info
      .as_ref()
      .ok_or_else(|| Error::InvalidConfig("not properly configured for signing".to_string()))?;

    let intent = Intent::iota_transaction();
    let intent_msg = IntentMessage::new(intent, tx_data);
    let mut hasher = DefaultHash::default();
    hasher.update(bcs::to_bytes(&intent_msg).map_err(|err| {
      Error::TransactionSigningFailed(format!("could not serialize transaction message to bcs; {err}"))
    })?);
    let digest = hasher.finalize().digest;

    let raw_signature = signer
      .sign(digest)
      .await
      .map_err(|err| Error::TransactionSigningFailed(format!("could not sign transaction message; {err}")))?;

    let binding = [
      [SignatureScheme::ED25519.flag()].as_slice(),
      &raw_signature,
      sender_public_key,
    ]
    .concat();
    let signature_bytes: &[u8] = binding.as_slice();

    Signature::from_bytes(signature_bytes)
      .map_err(|err| Error::TransactionSigningFailed(format!("could not parse signature to IOTA signature; {err}")))
  }
}

impl IdentityClient {
  pub async fn get_identity(&self, object_id: ObjectID) -> Result<Identity, Error> {
    // spawn all checks
    let mut all_futures =
      FuturesUnordered::<Pin<Box<dyn Future<Output = Result<Option<Identity>, Error>> + Send>>>::new();
    all_futures.push(Box::pin(resolve_new(&self.iota_client, object_id)));
    all_futures.push(Box::pin(resolve_migrated(&self.iota_client, object_id)));
    all_futures.push(Box::pin(resolve_unmigrated(&self.iota_client, object_id)));

    // use first non-None value as result
    let mut identity_outcome: Option<Identity> = None;
    while let Some(result) = all_futures.try_next().await? {
      if result.is_some() {
        identity_outcome = result;
        all_futures.clear();
        break;
      }
    }

    identity_outcome
      .ok_or_else(|| Error::DIDResolutionErrorKinesis(format!("could not find DID document for {object_id}")))
  }
}

// former extension trait, keep separate until properly re-integrated
impl IdentityClient {
  pub async fn resolve_did(&self, did: &IotaDID) -> Result<IotaDocument, Error> {
    // get alias id from did (starting with 0x)
    let object_id = ObjectID::from_str(&AliasId::from(did).to_string())
      .map_err(|err| Error::DIDResolutionErrorKinesis(format!("could not parse object id from did {did}; {err}")))?;

    let identity = get_identity(self, object_id).await?.ok_or_else(|| {
      Error::DIDResolutionErrorKinesis(format!("call succeeded but could not resolve {did} to object"))
    })?;
    let state_metadata = identity.did_doc.controlled_value();

    // unpack, replace placeholders and return document
    StateMetadataDocument::unpack(state_metadata)
      .and_then(|doc| doc.into_iota_document(did))
      .map_err(|err| {
        Error::DidDocParsingFailed(format!(
          "could not transform DID document to IotaDocument for DID {did}; {err}"
        ))
      })
  }

  pub async fn publish_did_document<S>(
    &self,
    document: IotaDocument,
    gas_budget: u64,
    signer: &S,
  ) -> Result<IotaDocument, Error>
  where
    S: Signer<KinesisKeySignature> + Send + Sync,
  {
    let packed = document
      .clone()
      .pack()
      .map_err(|err| Error::DidDocSerialization(format!("could not pack DID document: {err}")))?;

    let oci = self
      .create_identity(&packed)
      .gas_budget(gas_budget)
      .finish(self, signer)
      .await?;

    // replace placeholders in document
    let did: IotaDID = IotaDID::new(&oci.id.id.bytes, self.network_name());
    let metadata_document: StateMetadataDocument = document.into();
    let document_without_placeholders = metadata_document.into_iota_document(&did).map_err(|err| {
      Error::DidDocParsingFailed(format!(
        "could not replace placeholders in published DID document {did}; {err}"
      ))
    })?;

    Ok(document_without_placeholders)
  }
}

pub fn get_sender_public_key(sender_public_jwk: &Jwk) -> Result<Vec<u8>, Error> {
  let public_key_base_64 = &sender_public_jwk
    .try_okp_params()
    .map_err(|err| Error::InvalidKey(format!("key not of type `Okp`; {err}")))?
    .x;

  identity_jose::jwu::decode_b64(public_key_base_64)
    .map_err(|err| Error::InvalidKey(format!("could not decode base64 public key; {err}")))
}

pub fn convert_to_address(sender_public_key: &[u8]) -> Result<IotaAddress, Error> {
  let public_key = Ed25519PublicKey::from_bytes(sender_public_key)
    .map_err(|err| Error::InvalidKey(format!("could not parse public key to Ed25519 public key; {err}")))?;

  Ok(IotaAddress::from(&public_key))
}

async fn resolve_new(client: &IotaClient, object_id: ObjectID) -> Result<Option<Identity>, Error> {
  let onchain_identity = get_identity(client, object_id).await.map_err(|err| {
    Error::DIDResolutionErrorKinesis(format!(
      "could not get identity document for object id {object_id}; {err}"
    ))
  })?;
  Ok(onchain_identity.map(Identity::FullFledged))
}

async fn resolve_migrated(client: &IotaClient, object_id: ObjectID) -> Result<Option<Identity>, Error> {
  let onchain_identity = lookup(client, object_id).await.map_err(|err| {
    Error::DIDResolutionErrorKinesis(format!(
      "failed to look up object_id {object_id} in migration registry; {err}"
    ))
  })?;
  Ok(onchain_identity.map(Identity::FullFledged))
}

async fn resolve_unmigrated(client: &IotaClient, object_id: ObjectID) -> Result<Option<Identity>, Error> {
  let unmigrated_alias = get_alias(client, object_id)
    .await
    .map_err(|err| Error::DIDResolutionErrorKinesis(format!("could  no query for object id {object_id}; {err}")))?;
  Ok(unmigrated_alias.map(Identity::Legacy))
}
