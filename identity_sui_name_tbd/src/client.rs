// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::pin::Pin;
use std::str::FromStr;

use fastcrypto::hash::Blake2b256;
use fastcrypto::hash::HashFunction;
use fastcrypto::traits::ToFromBytes;
use futures::stream::FuturesUnordered;
use futures::Future;
use futures::TryStreamExt;
use identity_verification::jwk::Jwk;
use move_core_types::account_address::AccountAddress;
use move_core_types::language_storage::StructTag;
use secret_storage::key_signature_set::KeySignatureTypes;
use secret_storage::signer::Signer;
use shared_crypto::intent::Intent;
use shared_crypto::intent::IntentMessage;
use sui_sdk::rpc_types::OwnedObjectRef;
use sui_sdk::rpc_types::SuiTransactionBlockEffects;
use sui_sdk::rpc_types::SuiTransactionBlockResponseOptions;
use sui_sdk::types::base_types::ObjectID;
use sui_sdk::types::base_types::SuiAddress;
use sui_sdk::types::crypto::DefaultHash;
use sui_sdk::types::crypto::Signature;
use sui_sdk::types::crypto::SignatureScheme;
use sui_sdk::types::object::Owner;
use sui_sdk::types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use sui_sdk::types::quorum_driver_types::ExecuteTransactionRequestType;
use sui_sdk::types::transaction::Argument;
use sui_sdk::types::transaction::Command;
use sui_sdk::types::transaction::ProgrammableMoveCall;
use sui_sdk::types::transaction::ProgrammableTransaction;
use sui_sdk::types::transaction::Transaction;
use sui_sdk::types::transaction::TransactionData;
use sui_sdk::types::Identifier;
use sui_sdk::types::TypeTag;
use sui_sdk::types::SUI_FRAMEWORK_ADDRESS;
use sui_sdk::types::SUI_FRAMEWORK_PACKAGE_ID;
use sui_sdk::SuiClient;

use crate::migration::get_alias;
use crate::migration::get_identity_document;
use crate::migration::lookup;
use crate::Error;

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
  pub(crate) sui_client: Option<SuiClient>,
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

  /// Sets the `sui_client` value.
  #[must_use]
  pub fn sui_client(mut self, value: SuiClient) -> Self {
    self.sui_client = Some(value);
    self
  }

  /// Returns a new `IdentityClientBuilder` based on the `IdentityClientBuilder` configuration.
  pub fn build(self) -> Result<IdentityClient, Error> {
    IdentityClient::from_builder(self)
  }
}

struct SigningInfo {
  #[allow(dead_code)]
  sender_public_key: Vec<u8>,
  sender_address: SuiAddress,
}

pub struct IdentityClient {
  identity_iota_package_id: ObjectID,
  sui_client: SuiClient,
  signing_info: Option<SigningInfo>,
}

impl IdentityClient {
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

  pub fn sender_address(&self) -> Result<SuiAddress, Error> {
    self
      .signing_info
      .as_ref()
      .ok_or_else(|| Error::InvalidConfig("public key for sender not set".to_string()))
      .map(|v| v.sender_address)
  }

  /// Returns a new `CoreDocument` based on the [`DocumentBuilder`] configuration.
  pub fn from_builder(builder: IdentityClientBuilder) -> Result<Self, Error> {
    let signing_info = match builder.sender_public_key {
      Some(sender_public_key) => {
        let sender_address = convert_to_address(&sender_public_key)?;
        Some(SigningInfo {
          sender_public_key,
          sender_address,
        })
      }
      (None) => None,
      _ => {
        return Err(Error::InvalidConfig(
          r#"properties "public_jwk" must be set"#.to_string(),
        ));
      }
    };

    Ok(Self {
      identity_iota_package_id: builder
        .identity_iota_package_id
        .ok_or_else(|| Error::InvalidConfig("missing `identity_iota_package_id` argument".to_string()))?,
      sui_client: builder
        .sui_client
        .ok_or_else(|| Error::InvalidConfig("missing `sui_client` argument ".to_string()))?,
      signing_info,
    })
  }

  /// Publishes given IOTA document to new `document` onchain.
  pub async fn publish_did<S>(&self, iota_document: &[u8], gas_budget: u64, signer: &S) -> Result<ObjectID, Error>
  where
    S: Signer<KinesisKeySignature>,
  {
    let programmable_transaction =
      self.get_new_doc_programmable_transaction(iota_document, self.identity_iota_package_id)?;
    let tx_data = self.get_transaction_data(programmable_transaction, gas_budget).await?;
    let kinesis_signature = self.sign_transaction_data(&tx_data, signer).await?;

    // execute tx
    let response = self
      .sui_client
      .quorum_driver_api()
      .execute_transaction_block(
        Transaction::from_data(tx_data, vec![kinesis_signature]),
        SuiTransactionBlockResponseOptions::full_content(),
        Some(ExecuteTransactionRequestType::WaitForLocalExecution),
      )
      .await?;

    let created = match response.clone().effects {
      Some(SuiTransactionBlockEffects::V1(effects)) => effects.created,
      _ => {
        return Err(Error::TransactionUnexpectedResponse(format!(
          "could not find effects in transaction response: {response:?}"
        )));
      }
    };
    let new_documents: Vec<OwnedObjectRef> = created
      .into_iter()
      .filter(|elem| {
        matches!(
          elem.owner,
          Owner::Shared {
            initial_shared_version: _,
          }
        )
      })
      .collect();
    let new_document = match &new_documents[..] {
      [value] => value,
      _ => {
        return Err(Error::TransactionUnexpectedResponse(format!(
          "could not find new document in response: {response:?}"
        )));
      }
    };

    Ok(new_document.object_id())
  }

  async fn get_coin_for_transaction(&self) -> Result<sui_sdk::rpc_types::Coin, Error> {
    let coins = self
      .sui_client
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
      .sui_client
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
    S: Signer<KinesisKeySignature>,
  {
    let SigningInfo { sender_public_key, .. } = self
      .signing_info
      .as_ref()
      .ok_or_else(|| Error::InvalidConfig("not properly configured for signing".to_string()))?;

    let intent = Intent::sui_transaction();
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

  fn get_new_doc_programmable_transaction(
    &self,
    iota_document: &[u8],
    package_id: ObjectID,
  ) -> Result<sui_sdk::types::transaction::ProgrammableTransaction, Error> {
    let mut ptb = ProgrammableTransactionBuilder::new();

    let pure_vec: Vec<Argument> = iota_document
      .iter()
      .map(|elem| ptb.pure(*elem))
      .collect::<Result<Vec<_>, _>>()
      .map_err(|err| Error::InvalidArgument(format!("could not convert given document to move vector; {err}")))?;
    let iota_document_move_vec = ptb.command(Command::MakeMoveVec(Some(sui_sdk::types::TypeTag::U8), pure_vec));

    let sui_struct_tag: StructTag = StructTag {
      address: SUI_FRAMEWORK_ADDRESS,
      module: Identifier::from_str("sui")
        .map_err(|err| Error::ParsingFailed(format!("\"sui\" to identifier; {err}")))?,
      name: Identifier::from_str("SUI").map_err(|err| Error::ParsingFailed(format!("\"SUI\" to identifier; {err}")))?,
      type_params: vec![],
    };
    let balance = ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
      package: SUI_FRAMEWORK_PACKAGE_ID,
      module: Identifier::from_str("balance")
        .map_err(|err| Error::ParsingFailed(format!("\"balance\" to identifier; {err}")))?,
      function: Identifier::from_str("zero")
        .map_err(|err| Error::ParsingFailed(format!("\"zero\" to identifier; {err}")))?,
      type_arguments: vec![TypeTag::Struct(Box::new(sui_struct_tag))],
      arguments: vec![],
    })));

    let bag = ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
      package: SUI_FRAMEWORK_PACKAGE_ID,
      module: Identifier::from_str("bag")
        .map_err(|err| Error::ParsingFailed(format!("\"bag\" to identifier; {err}")))?,
      function: Identifier::from_str("new")
        .map_err(|err| Error::ParsingFailed(format!("\"new\" to identifier; {err}")))?,
      type_arguments: vec![],
      arguments: vec![],
    })));

    let new_document_result = ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
      package: package_id,
      module: Identifier::from_str("document")
        .map_err(|err| Error::ParsingFailed(format!("\"document\" to identifier; {err}")))?,
      function: Identifier::from_str("new")
        .map_err(|err| Error::ParsingFailed(format!("\"new\" to identifier; {err}")))?,
      type_arguments: vec![],
      arguments: vec![iota_document_move_vec, balance, bag],
    })));

    let new_document_result_index = if let Argument::Result(index) = new_document_result {
      index
    } else {
      return Err(Error::TransactionBuildingFailed(
        "could not get result index from document::new call".to_string(),
      ));
    };

    ptb.transfer_arg(
      self.sender_address()?,
      Argument::NestedResult(new_document_result_index, 1),
    );

    let document_struct_tag: StructTag = StructTag {
      address: AccountAddress::from_str(&package_id.to_string())
        .map_err(|err| Error::ParsingFailed(format!("package id\"{package_id}\" to account address; {err}")))?,
      module: Identifier::from_str("document")
        .map_err(|err| Error::ParsingFailed(format!("\"document\" to identifier; {err}")))?,
      name: Identifier::from_str("Document")
        .map_err(|err| Error::ParsingFailed(format!("\"Document\" to identifier; {err}")))?,
      type_params: vec![],
    };

    ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
      package: SUI_FRAMEWORK_PACKAGE_ID,
      module: Identifier::from_str("transfer")
        .map_err(|err| Error::ParsingFailed(format!("\"transfer\" to identifier; {err}")))?,
      function: Identifier::from_str("public_share_object")
        .map_err(|err| Error::ParsingFailed(format!("\"public_share_object\" to identifier; {err}")))?,
      type_arguments: vec![TypeTag::Struct(Box::new(document_struct_tag))],
      arguments: vec![Argument::NestedResult(new_document_result_index, 0)],
    })));

    Ok(ptb.finish())
  }
}

impl IdentityClient {
  pub async fn get_did_document(&self, object_id: ObjectID) -> Result<Vec<u8>, Error> {
    // spawn all checks
    let mut all_futures =
      FuturesUnordered::<Pin<Box<dyn Future<Output = Result<Option<Vec<u8>>, Error>> + Send>>>::new();
    all_futures.push(Box::pin(resolve_new(&self.sui_client, object_id)));
    all_futures.push(Box::pin(resolve_migrated(&self.sui_client, object_id)));
    all_futures.push(Box::pin(resolve_unmigrated(&self.sui_client, object_id)));

    // use first non-None value as result
    let mut state_metadata_outcome: Option<Vec<u8>> = None;
    while let Some(result) = all_futures.try_next().await? {
      if result.is_some() {
        state_metadata_outcome = result;
        all_futures.clear();
        break;
      }
    }

    // check if we found state metadata
    let state_metadata = if let Some(value) = state_metadata_outcome {
      value
    } else {
      return Err(Error::DIDResolutionErrorKinesis(format!(
        "could not find DID document for {object_id}"
      )));
    };

    // unpack and return document
    Ok(state_metadata)
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

pub fn convert_to_address(sender_public_key: &[u8]) -> Result<SuiAddress, Error> {
  let to_hash = [[SignatureScheme::ED25519.flag()].as_slice(), sender_public_key].concat();

  let mut hasher = Blake2b256::default();
  hasher.update(to_hash.as_slice());
  let digest = hasher.finalize().digest;

  SuiAddress::from_bytes(digest)
    .map_err(|err| Error::InvalidKey(format!("could not convert public key to sender address; {err}")))
}

async fn resolve_new(client: &SuiClient, object_id: ObjectID) -> Result<Option<Vec<u8>>, Error> {
  let document = get_identity_document(client, object_id).await.map_err(|err| {
    Error::DIDResolutionErrorKinesis(format!(
      "could not get identity document for object id {object_id}; {err}"
    ))
  })?;

  Ok(document.map(|document| document.doc))
}

async fn resolve_migrated(client: &SuiClient, object_id: ObjectID) -> Result<Option<Vec<u8>>, Error> {
  let document = lookup(client, object_id).await.map_err(|err| {
    Error::DIDResolutionErrorKinesis(format!(
      "failed to look up object_id {object_id} in migration registry; {err}"
    ))
  })?;

  Ok(document.map(|document| document.doc))
}

async fn resolve_unmigrated(client: &SuiClient, object_id: ObjectID) -> Result<Option<Vec<u8>>, Error> {
  let unmigrated_alias = get_alias(client, object_id)
    .await
    .map_err(|err| Error::DIDResolutionErrorKinesis(format!("could  no query for object id {object_id}; {err}")))?;
  unmigrated_alias
    .map(|v| {
      v.state_metadata.ok_or_else(|| {
        Error::DIDResolutionErrorKinesis(format!(
          "unmigrated alias for object id {object_id} does not contain DID document"
        ))
      })
    })
    .transpose()
}
