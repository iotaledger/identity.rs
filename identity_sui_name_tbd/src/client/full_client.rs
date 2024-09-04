// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::ops::Deref;

use async_trait::async_trait;
use fastcrypto::ed25519::Ed25519PublicKey;
use fastcrypto::traits::ToFromBytes;
use identity_iota_core::IotaDID;
use identity_iota_core::IotaDocument;
use identity_iota_core::StateMetadataDocument;
use identity_verification::jwk::Jwk;
use iota_sdk::rpc_types::IotaExecutionStatus;
use iota_sdk::rpc_types::IotaObjectData;
use iota_sdk::rpc_types::IotaObjectDataFilter;
use iota_sdk::rpc_types::IotaObjectResponseQuery;
use iota_sdk::rpc_types::IotaTransactionBlockEffects;
use iota_sdk::rpc_types::IotaTransactionBlockEffectsAPI;
use iota_sdk::rpc_types::IotaTransactionBlockEffectsV1;
use iota_sdk::rpc_types::IotaTransactionBlockResponse;
use iota_sdk::rpc_types::IotaTransactionBlockResponseOptions;
use iota_sdk::types::base_types::IotaAddress;
use iota_sdk::types::base_types::ObjectRef;
use iota_sdk::types::crypto::DefaultHash;
use iota_sdk::types::crypto::Signature;
use iota_sdk::types::crypto::SignatureScheme;
use iota_sdk::types::quorum_driver_types::ExecuteTransactionRequestType;
use iota_sdk::types::transaction::ProgrammableTransaction;
use iota_sdk::types::transaction::Transaction;
use iota_sdk::types::transaction::TransactionData;
use move_core_types::language_storage::StructTag;
use secret_storage::SignatureScheme as SignatureSchemeT;
use secret_storage::Signer;
use serde::de::DeserializeOwned;
use serde::Serialize;
use shared_crypto::intent::Intent;
use shared_crypto::intent::IntentMessage;

use crate::assets::AuthenticatedAssetBuilder;
use crate::migration::Identity;
use crate::migration::IdentityBuilder;
use crate::transaction::Transaction as TransactionT;
use crate::utils::MoveType;
use crate::Error;

use super::get_object_id_from_did;
use super::IdentityClientReadOnly;

pub struct IotaKeySignature {
  pub public_key: Vec<u8>,
  pub signature: Vec<u8>,
}

impl SignatureSchemeT for IotaKeySignature {
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

#[derive(Clone)]
pub struct IdentityClient<S> {
  read_client: IdentityClientReadOnly,
  address: IotaAddress,
  public_key: Vec<u8>,
  signer: S,
}

impl<S> Deref for IdentityClient<S> {
  type Target = IdentityClientReadOnly;
  fn deref(&self) -> &Self::Target {
    &self.read_client
  }
}

impl<S> IdentityClient<S>
where
  S: Signer<IotaKeySignature>,
{
  pub async fn new(client: IdentityClientReadOnly, signer: S) -> Result<Self, Error> {
    let public_key = signer
      .public_key()
      .await
      .map_err(|e| Error::InvalidKey(e.to_string()))?;
    let address = convert_to_address(&public_key)?;

    Ok(Self {
      public_key,
      address,
      read_client: client,
      signer,
    })
  }

  async fn sign_transaction_data(&self, tx_data: &TransactionData) -> Result<Signature, Error> {
    use fastcrypto::hash::HashFunction;
    let sender_public_key = self.sender_public_key();

    let intent = Intent::iota_transaction();
    let intent_msg = IntentMessage::new(intent, tx_data);
    let mut hasher = DefaultHash::default();
    let bcs_bytes = bcs::to_bytes(&intent_msg).map_err(|err| {
      Error::TransactionSigningFailed(format!("could not serialize transaction message to bcs; {err}"))
    })?;
    hasher.update(bcs_bytes);
    let digest = hasher.finalize().digest;

    let raw_signature = self
      .signer
      .sign(&digest)
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

  pub(crate) async fn execute_transaction(
    &self,
    tx: ProgrammableTransaction,
    gas_budget: Option<u64>,
  ) -> Result<IotaTransactionBlockResponse, Error> {
    let gas_budget = match gas_budget {
      Some(gas) => gas,
      None => self.default_gas_budget(&tx).await?,
    };
    let tx_data = self.get_transaction_data(tx, gas_budget).await?;
    let signature = self.sign_transaction_data(&tx_data).await?;

    // execute tx
    let response = self
      .quorum_driver_api()
      .execute_transaction_block(
        Transaction::from_data(tx_data, vec![signature]),
        IotaTransactionBlockResponseOptions::full_content(),
        Some(ExecuteTransactionRequestType::WaitForLocalExecution),
      )
      .await
      .map_err(Error::TransactionExecutionFailed)?;

    if let Some(IotaTransactionBlockEffects::V1(IotaTransactionBlockEffectsV1 {
      status: IotaExecutionStatus::Failure { error },
      ..
    })) = &response.effects
    {
      Err(Error::TransactionUnexpectedResponse(error.to_string()))
    } else {
      Ok(response)
    }
  }
}

impl<S> IdentityClient<S> {
  pub fn sender_public_key(&self) -> &[u8] {
    &self.public_key
  }

  pub fn sender_address(&self) -> IotaAddress {
    self.address
  }

  pub fn signer(&self) -> &S {
    &self.signer
  }

  /// Creates a new onchain Identity.
  pub fn create_identity<'a>(&self, iota_document: &'a [u8]) -> IdentityBuilder<'a> {
    IdentityBuilder::new(iota_document)
  }

  pub fn create_authenticated_asset<T>(&self, content: T) -> AuthenticatedAssetBuilder<T>
  where
    T: MoveType + Serialize + DeserializeOwned,
  {
    AuthenticatedAssetBuilder::new(content)
  }

  pub(crate) async fn default_gas_budget(&self, tx: &ProgrammableTransaction) -> Result<u64, Error> {
    let gas_price = self
      .read_api()
      .get_reference_gas_price()
      .await
      .map_err(|e| Error::RpcError(e.to_string()))?;
    let gas_coin = self.get_coin_for_transaction().await?;
    let tx_data = TransactionData::new_programmable(
      self.sender_address(),
      vec![gas_coin.object_ref()],
      tx.clone(),
      50_000_000_000,
      gas_price,
    );
    let dry_run_gas_result = self.read_api().dry_run_transaction_block(tx_data).await?.effects;
    if dry_run_gas_result.status().is_err() {
      let IotaExecutionStatus::Failure { error } = dry_run_gas_result.into_status() else {
        unreachable!();
      };
      return Err(Error::TransactionUnexpectedResponse(error));
    }
    let gas_summary = dry_run_gas_result.gas_cost_summary();
    let overhead = gas_price * 1000;
    let net_used = gas_summary.net_gas_usage();
    let computation = gas_summary.computation_cost;

    let budget = overhead + (net_used.max(0) as u64).max(computation);
    Ok(budget)
  }

  async fn get_coin_for_transaction(&self) -> Result<iota_sdk::rpc_types::Coin, Error> {
    let coins = self
      .coin_read_api()
      .get_coins(self.sender_address(), None, None, None)
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
      .read_api()
      .get_reference_gas_price()
      .await
      .map_err(|err| Error::GasIssue(format!("could not get gas price; {err}")))?;
    let coin = self.get_coin_for_transaction().await?;
    let tx_data = TransactionData::new_programmable(
      self.sender_address(),
      vec![coin.object_ref()],
      programmable_transaction,
      gas_budget,
      gas_price,
    );

    Ok(tx_data)
  }

  pub async fn find_owned_ref<P>(&self, tag: StructTag, predicate: P) -> Result<Option<ObjectRef>, Error>
  where
    P: Fn(&IotaObjectData) -> bool,
  {
    let filter = IotaObjectResponseQuery::new_with_filter(IotaObjectDataFilter::StructType(tag));

    let mut cursor = None;
    loop {
      let mut page = self
        .read_api()
        .get_owned_objects(self.sender_address(), Some(filter.clone()), cursor, None)
        .await?;
      let obj_ref = std::mem::take(&mut page.data)
        .into_iter()
        .filter_map(|res| res.data)
        .find(|obj| predicate(obj))
        .map(|obj_data| obj_data.object_ref());
      cursor = page.next_cursor;

      if obj_ref.is_some() {
        return Ok(obj_ref);
      }
      if !page.has_next_page {
        break;
      }
    }

    Ok(None)
  }
}

impl<S> IdentityClient<S>
where
  S: Signer<IotaKeySignature> + Sync,
{
  pub fn publish_did_document(&self, document: IotaDocument) -> PublishDidTx {
    PublishDidTx(document)
  }

  // TODO: define what happens for (legacy|migrated|new) documents
  pub async fn publish_did_document_update(
    &self,
    document: IotaDocument,
    gas_budget: u64,
  ) -> Result<IotaDocument, Error> {
    let mut oci =
      if let Identity::FullFledged(value) = self.get_identity(get_object_id_from_did(document.id())?).await? {
        value
      } else {
        return Err(Error::Identity("only new identities can be updated".to_string()));
      };

    oci
      .update_did_document(document.clone())
      .finish()
      .execute_with_gas(gas_budget, self)
      .await?;

    Ok(document)
  }

  pub async fn deactivate_did_output(&self, did: &IotaDID, gas_budget: u64) -> Result<(), Error> {
    let mut oci = if let Identity::FullFledged(value) = self.get_identity(get_object_id_from_did(did)?).await? {
      value
    } else {
      return Err(Error::Identity("only new identities can be deactivated".to_string()));
    };

    oci.deactivate_did().finish().execute_with_gas(gas_budget, self).await?;

    Ok(())
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

#[derive(Debug)]
pub struct PublishDidTx(IotaDocument);

#[async_trait]
impl TransactionT for PublishDidTx {
  type Output = IotaDocument;
  async fn execute_with_opt_gas<S>(
    self,
    gas_budget: Option<u64>,
    client: &IdentityClient<S>,
  ) -> Result<Self::Output, Error>
  where
    S: Signer<IotaKeySignature> + Sync,
  {
    let packed = self
      .0
      .clone()
      .pack()
      .map_err(|err| Error::DidDocSerialization(format!("could not pack DID document: {err}")))?;

    let oci = client
      .create_identity(&packed)
      .finish()
      .execute_with_opt_gas(gas_budget, client)
      .await?;

    // replace placeholders in document
    let did: IotaDID = IotaDID::new(&oci.id(), client.network());
    let metadata_document: StateMetadataDocument = self.0.into();
    let document_without_placeholders = metadata_document.into_iota_document(&did).map_err(|err| {
      Error::DidDocParsingFailed(format!(
        "could not replace placeholders in published DID document {did}; {err}"
      ))
    })?;

    Ok(document_without_placeholders)
  }
}
