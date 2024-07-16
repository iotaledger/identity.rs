// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::ops::Deref;
use std::pin::Pin;

use fastcrypto::ed25519::Ed25519PublicKey;
use fastcrypto::hash::HashFunction;
use fastcrypto::traits::ToFromBytes;
use futures::stream::FuturesUnordered;
use futures::Future;
use futures::TryStreamExt;
use identity_storage::JwkStorage;
use identity_storage::KeyId;
use identity_storage::KeyIdStorage;
use identity_storage::Storage;
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
use shared_crypto::intent::Intent;
use shared_crypto::intent::IntentMessage;

use crate::migration::get_alias;
use crate::migration::get_identity;
use crate::migration::lookup;
use crate::migration::Identity;
use crate::migration::IdentityBuilder;
use crate::Error;

pub struct IdentityClientBuilder<K, I> {
  pub(crate) identity_iota_package_id: Option<ObjectID>,
  pub(crate) public_jwk: Option<Jwk>,
  pub(crate) sender_key_id: Option<KeyId>,
  pub(crate) storage: Option<Storage<K, I>>,
  pub(crate) iota_client: Option<IotaClient>,
}

impl<K, I> IdentityClientBuilder<K, I>
where
  K: JwkStorage,
  I: KeyIdStorage,
{
  /// Sets the `identity_iota_package_id` value.
  #[must_use]
  pub fn identity_iota_package_id(mut self, value: ObjectID) -> Self {
    self.identity_iota_package_id = Some(value);
    self
  }

  /// Sets the `sender_key_id` value.
  #[must_use]
  pub fn sender_key_id(mut self, value: KeyId) -> Self {
    self.sender_key_id = Some(value);
    self
  }

  /// Sets the `sender_public_key` value.
  #[must_use]
  pub fn sender_public_jwk(mut self, value: Jwk) -> Self {
    self.public_jwk = Some(value);
    self
  }

  /// Sets the `storage` value.
  #[must_use]
  pub fn storage(mut self, value: Storage<K, I>) -> Self {
    self.storage = Some(value);
    self
  }

  /// Sets the `iota_client` value.
  #[must_use]
  pub fn iota_client(mut self, value: IotaClient) -> Self {
    self.iota_client = Some(value);
    self
  }

  /// Returns a new `IdentityClientBuilder` based on the `IdentityClientBuilder` configuration.
  pub fn build(self) -> Result<IdentityClient<K, I>, Error> {
    IdentityClient::from_builder(self)
  }
}

impl<K, I> Default for IdentityClientBuilder<K, I> {
  fn default() -> Self {
    Self {
      identity_iota_package_id: None,
      public_jwk: None,
      sender_key_id: None,
      storage: None,
      iota_client: None,
    }
  }
}

struct SigningInfo<K, I> {
  sender_key_id: KeyId,
  sender_public_jwk: Jwk,
  sender_public_key: Vec<u8>,
  sender_address: IotaAddress,
  storage: Storage<K, I>,
}

pub struct IdentityClient<K, I> {
  identity_iota_package_id: ObjectID,
  iota_client: IotaClient,
  signing_info: Option<SigningInfo<K, I>>,
}

impl<K, I> Deref for IdentityClient<K, I> {
  type Target = IotaClient;
  fn deref(&self) -> &Self::Target {
    &self.iota_client
  }
}

impl<K, I> IdentityClient<K, I>
where
  K: JwkStorage,
  I: KeyIdStorage,
{
  pub(crate) fn package_id(&self) -> ObjectID {
    self.identity_iota_package_id
  }

  pub fn builder() -> IdentityClientBuilder<K, I> {
    IdentityClientBuilder::<K, I>::default()
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

  /// Returns a new `CoreDocument` based on the [`DocumentBuilder`] configuration.
  pub fn from_builder(builder: IdentityClientBuilder<K, I>) -> Result<Self, Error> {
    let signing_info = match (builder.public_jwk, builder.sender_key_id, builder.storage) {
      (Some(public_jwk), Some(sender_key_id), Some(storage)) => {
        let sender_public_key = get_sender_public_key(&public_jwk)?;
        let sender_address = convert_to_address(&sender_public_key)?;
        Some(SigningInfo {
          sender_key_id,
          sender_public_jwk: public_jwk,
          sender_public_key,
          sender_address,
          storage,
        })
      }
      (None, None, None) => None,
      _ => {
        return Err(Error::InvalidConfig(
          r#"properties "public_jwk", "sender_key_id", and "storage" must be set together"#.to_string(),
        ));
      }
    };

    Ok(Self {
      identity_iota_package_id: builder
        .identity_iota_package_id
        .ok_or_else(|| Error::InvalidConfig("missing `identity_iota_package_id` argument".to_string()))?,
      iota_client: builder
        .iota_client
        .ok_or_else(|| Error::InvalidConfig("missing `iota_client` argument ".to_string()))?,
      signing_info,
    })
  }

  /// Creates a new onchain Identity.
  pub fn create_identity<'a>(&self, iota_document: &'a [u8]) -> IdentityBuilder<'a> {
    IdentityBuilder::new(iota_document, self.identity_iota_package_id)
  }

  pub(crate) async fn execute_transaction(
    &self,
    tx: ProgrammableTransaction,
    gas_budget: u64,
  ) -> Result<IotaTransactionBlockResponse, Error> {
    let tx_data = self.get_transaction_data(tx, gas_budget).await?;
    let kinesis_signature = self.sign_transaction_data(&tx_data).await?;

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

  async fn sign_transaction_data(&self, tx_data: &TransactionData) -> Result<Signature, Error> {
    let SigningInfo {
      storage,
      sender_key_id,
      sender_public_key,
      sender_public_jwk,
      ..
    } = self
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

    let raw_signature = storage
      .key_storage()
      .sign(sender_key_id, &digest, sender_public_jwk)
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

impl<K, I> IdentityClient<K, I> {
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

fn get_sender_public_key(sender_public_jwk: &Jwk) -> Result<Vec<u8>, Error> {
  let public_key_base_64 = &sender_public_jwk
    .try_okp_params()
    .map_err(|err| Error::InvalidKey(format!("key not of type `Okp`; {err}")))?
    .x;

  identity_jose::jwu::decode_b64(public_key_base_64)
    .map_err(|err| Error::InvalidKey(format!("could not decode base64 public key; {err}")))
}

fn convert_to_address(sender_public_key: &[u8]) -> Result<IotaAddress, Error> {
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
