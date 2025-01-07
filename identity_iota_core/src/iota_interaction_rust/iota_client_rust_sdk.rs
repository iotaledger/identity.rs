// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;
use std::boxed::Box;
use std::marker::Send;
use std::option::Option;
use std::result::Result;

use fastcrypto::hash::Blake2b256;
use fastcrypto::traits::ToFromBytes;
use secret_storage::Signer;

use identity_iota_interaction::apis::{CoinReadApi, EventApi, QuorumDriverApi, ReadApi};
use identity_iota_interaction::error::IotaRpcResult;
use identity_iota_interaction::rpc_types::{
  Coin, CoinPage, EventFilter, EventPage, IotaExecutionStatus, IotaObjectData, IotaObjectDataOptions,
  IotaObjectResponse, IotaObjectResponseQuery, IotaPastObjectResponse, IotaTransactionBlockEffects,
  IotaTransactionBlockEffectsAPI, IotaTransactionBlockEffectsV1, IotaTransactionBlockResponse,
  IotaTransactionBlockResponseOptions, ObjectChange, ObjectsPage, OwnedObjectRef,
};
use identity_iota_interaction::shared_crypto::intent::{Intent, IntentMessage};
use identity_iota_interaction::types::{
  base_types::{IotaAddress, ObjectID, SequenceNumber},
  crypto::{Signature, SignatureScheme},
  digests::TransactionDigest,
  dynamic_field::DynamicFieldName,
  event::EventID,
  quorum_driver_types::ExecuteTransactionRequestType,
  transaction::{ProgrammableTransaction, Transaction, TransactionData},
};
use identity_iota_interaction::{IotaKeySignature};
use identity_iota_interaction::{
  CoinReadTrait, EventTrait, IotaClientTrait, IotaTransactionBlockResponseT, ProgrammableTransactionBcs,
  QuorumDriverTrait, ReadTrait,
};
use identity_iota_interaction::{IotaClient, SignatureBcs, TransactionDataBcs};
use crate::rebased::Error;

/// The minimum balance required to execute a transaction.
pub(crate) const MINIMUM_BALANCE: u64 = 1_000_000_000;

#[allow(unreachable_pub, dead_code)]
pub trait IotaTransactionBlockResponseAdaptedT: IotaTransactionBlockResponseT<Error=Error, NativeResponse=IotaTransactionBlockResponse> {}
impl<T> IotaTransactionBlockResponseAdaptedT for T where T: IotaTransactionBlockResponseT<Error=Error, NativeResponse=IotaTransactionBlockResponse> {}
#[allow(unreachable_pub, dead_code)]
pub type IotaTransactionBlockResponseAdaptedTraitObj = Box<dyn IotaTransactionBlockResponseT<Error=Error, NativeResponse=IotaTransactionBlockResponse>>;

#[allow(unreachable_pub, dead_code)]
pub trait QuorumDriverApiAdaptedT: QuorumDriverTrait<Error=Error, NativeResponse=IotaTransactionBlockResponse> {}
impl<T> QuorumDriverApiAdaptedT for T where T: QuorumDriverTrait<Error=Error, NativeResponse=IotaTransactionBlockResponse> {}
#[allow(unreachable_pub, dead_code)]
pub type QuorumDriverApiAdaptedTraitObj = Box<dyn QuorumDriverTrait<Error=Error, NativeResponse=IotaTransactionBlockResponse>>;

#[allow(unreachable_pub, dead_code)]
pub trait ReadApiAdaptedT: ReadTrait<Error=Error, NativeResponse=IotaTransactionBlockResponse> {}
impl<T> ReadApiAdaptedT for T where T: ReadTrait<Error=Error, NativeResponse=IotaTransactionBlockResponse> {}
#[allow(unreachable_pub, dead_code)]
pub type ReadApiAdaptedTraitObj = Box<dyn ReadTrait<Error=Error, NativeResponse=IotaTransactionBlockResponse>>;

#[allow(unreachable_pub, dead_code)]
pub trait CoinReadApiAdaptedT: CoinReadTrait<Error=Error> {}
impl<T> CoinReadApiAdaptedT for T where T: CoinReadTrait<Error=Error> {}
#[allow(unreachable_pub, dead_code)]
pub type CoinReadApiAdaptedTraitObj = Box<dyn CoinReadTrait<Error=Error>>;

#[allow(unreachable_pub, dead_code)]
pub trait EventApiAdaptedT: EventTrait<Error=Error> {}
impl<T> EventApiAdaptedT for T where T: EventTrait<Error=Error> {}
#[allow(unreachable_pub, dead_code)]
pub type EventApiAdaptedTraitObj = Box<dyn EventTrait<Error=Error>>;

#[allow(unreachable_pub, dead_code)]
pub trait IotaClientAdaptedT: IotaClientTrait<Error=Error, NativeResponse=IotaTransactionBlockResponse> {}
impl<T> IotaClientAdaptedT for T where T: IotaClientTrait<Error=Error, NativeResponse=IotaTransactionBlockResponse> {}
#[allow(unreachable_pub, dead_code)]
pub type IotaClientAdaptedTraitObj = Box<dyn IotaClientTrait<Error=Error, NativeResponse=IotaTransactionBlockResponse>>;

pub struct IotaTransactionBlockResponseProvider {
  response: IotaTransactionBlockResponse,
}

impl IotaTransactionBlockResponseProvider {
  pub(crate) fn new(response: IotaTransactionBlockResponse) -> Self {
    IotaTransactionBlockResponseProvider { response }
  }
}

impl IotaTransactionBlockResponseT for IotaTransactionBlockResponseProvider {
  type Error = Error;
  type NativeResponse = IotaTransactionBlockResponse;

  fn effects_is_none(&self) -> bool {
    self.response.effects.is_none()
  }

  fn effects_is_some(&self) -> bool {
    self.response.effects.is_some()
  }

  fn to_string(&self) -> String {
    format!("{:?}", self.response)
  }

  fn effects_execution_status(&self) -> Option<IotaExecutionStatus> {
    self.response.effects.as_ref().map(|effects| effects.status().clone())
  }

  fn effects_created(&self) -> Option<Vec<OwnedObjectRef>> {
    self.response.effects.as_ref().map(|effects| effects.created().to_vec())
  }

  fn as_native_response(&self) -> &Self::NativeResponse {
    &self.response
  }

  fn as_mut_native_response(&mut self) -> &mut Self::NativeResponse {
    &mut self.response
  }

  fn clone_native_response(&self) -> Self::NativeResponse {
    self.response.clone()
  }
}

pub(crate) struct QuorumDriverAdapter<'a> {
  api: &'a QuorumDriverApi,
}

#[async_trait::async_trait()]
impl<'a> QuorumDriverTrait for QuorumDriverAdapter<'a> {
  type Error = Error;
  type NativeResponse = IotaTransactionBlockResponse;

  async fn execute_transaction_block(
    &self,
    tx_data_bcs: &TransactionDataBcs,
    signatures: &[SignatureBcs],
    options: Option<IotaTransactionBlockResponseOptions>,
    request_type: Option<ExecuteTransactionRequestType>,
  ) -> IotaRpcResult<IotaTransactionBlockResponseAdaptedTraitObj> {
    let tx_data = bcs::from_bytes::<TransactionData>(tx_data_bcs.as_slice())?;
    let signatures_vec = signatures
      .iter()
      .map(|signature_bcs| bcs::from_bytes::<Signature>(signature_bcs.as_slice()))
      .collect::<Result<Vec<Signature>, _>>()?;
    let tx = Transaction::from_data(tx_data, signatures_vec);
    let response = self
      .api
      .execute_transaction_block(tx, options.unwrap_or_default(), request_type)
      .await?;
    Ok(Box::new(IotaTransactionBlockResponseProvider::new(response)))
  }
}

pub(crate) struct ReadAdapter<'a> {
  api: &'a ReadApi,
}

#[async_trait::async_trait()]
impl<'a> ReadTrait for ReadAdapter<'a> {
  type Error = Error;
  type NativeResponse = IotaTransactionBlockResponse;

  async fn get_chain_identifier(&self) -> Result<String, Self::Error> {
    self
      .api
      .get_chain_identifier()
      .await
      .map_err(|e| Error::Network("SDK get_chain_identifier() call failed".to_string(), e))
  }

  async fn get_dynamic_field_object(
    &self,
    parent_object_id: ObjectID,
    name: DynamicFieldName,
  ) -> IotaRpcResult<IotaObjectResponse> {
    self.api.get_dynamic_field_object(parent_object_id, name).await
  }

  async fn get_object_with_options(
    &self,
    object_id: ObjectID,
    options: IotaObjectDataOptions,
  ) -> IotaRpcResult<IotaObjectResponse> {
    self.api.get_object_with_options(object_id, options).await
  }

  async fn get_owned_objects(
    &self,
    address: IotaAddress,
    query: Option<IotaObjectResponseQuery>,
    cursor: Option<ObjectID>,
    limit: Option<usize>,
  ) -> IotaRpcResult<ObjectsPage> {
    self.api.get_owned_objects(address, query, cursor, limit).await
  }

  async fn get_reference_gas_price(&self) -> IotaRpcResult<u64> {
    self.api.get_reference_gas_price().await
  }

  async fn get_transaction_with_options(
    &self,
    digest: TransactionDigest,
    options: IotaTransactionBlockResponseOptions,
  ) -> IotaRpcResult<IotaTransactionBlockResponseAdaptedTraitObj> {
    let response = self.api.get_transaction_with_options(digest, options).await?;
    Ok(Box::new(IotaTransactionBlockResponseProvider::new(response)))
  }

  async fn try_get_parsed_past_object(
    &self,
    object_id: ObjectID,
    version: SequenceNumber,
    options: IotaObjectDataOptions,
  ) -> IotaRpcResult<IotaPastObjectResponse> {
    self.api.try_get_parsed_past_object(object_id, version, options).await
  }
}

pub(crate) struct CoinReadAdapter<'a> {
  api: &'a CoinReadApi,
}

#[async_trait::async_trait()]
impl<'a> CoinReadTrait for CoinReadAdapter<'a> {
  type Error = Error;

  async fn get_coins(
    &self,
    owner: IotaAddress,
    coin_type: Option<String>,
    cursor: Option<ObjectID>,
    limit: Option<usize>,
  ) -> IotaRpcResult<CoinPage> {
    self.api.get_coins(owner, coin_type, cursor, limit).await
  }
}

pub(crate) struct EventAdapter<'a> {
  api: &'a EventApi,
}

#[async_trait::async_trait()]
impl<'a> EventTrait for EventAdapter<'a> {
  type Error = Error;

  async fn query_events(
    &self,
    query: EventFilter,
    cursor: Option<EventID>,
    limit: Option<usize>,
    descending_order: bool,
  ) -> IotaRpcResult<EventPage> {
    self.api.query_events(query, cursor, limit, descending_order).await
  }
}

#[derive(Clone)]
pub struct IotaClientRustSdk {
  iota_client: IotaClient,
}

#[async_trait]
impl IotaClientTrait for IotaClientRustSdk {
  type Error = Error;
  type NativeResponse = IotaTransactionBlockResponse;

  fn quorum_driver_api(&self) -> Box<dyn QuorumDriverTrait<Error=Error, NativeResponse=IotaTransactionBlockResponse> + Send + '_> {
    Box::new(QuorumDriverAdapter {
      api: self.iota_client.quorum_driver_api(),
    })
  }

  fn read_api(&self) -> Box<dyn ReadTrait<Error=Error, NativeResponse=IotaTransactionBlockResponse> + Send + '_> {
    Box::new(ReadAdapter {
      api: self.iota_client.read_api(),
    })
  }

  fn coin_read_api(&self) -> Box<dyn CoinReadTrait<Error = Self::Error> + Send + '_> {
    Box::new(CoinReadAdapter {
      api: self.iota_client.coin_read_api(),
    })
  }

  fn event_api(&self) -> Box<dyn EventTrait<Error = Self::Error> + Send + '_> {
    Box::new(EventAdapter {
      api: self.iota_client.event_api(),
    })
  }

  async fn execute_transaction<S: Signer<IotaKeySignature> + Sync>(
    &self,
    sender_address: IotaAddress,
    sender_public_key: &[u8],
    tx_bcs: ProgrammableTransactionBcs,
    gas_budget: Option<u64>,
    signer: &S,
  ) -> Result<IotaTransactionBlockResponseAdaptedTraitObj, Self::Error> {
    let tx = bcs::from_bytes::<ProgrammableTransaction>(tx_bcs.as_slice())?;
    let response = self
      .sdk_execute_transaction(sender_address, sender_public_key, tx, gas_budget, signer)
      .await?;
    Ok(Box::new(IotaTransactionBlockResponseProvider::new(response)))
  }

  async fn default_gas_budget(
    &self,
    sender_address: IotaAddress,
    tx_bcs: &ProgrammableTransactionBcs,
  ) -> Result<u64, Error> {
    let tx = bcs::from_bytes::<ProgrammableTransaction>(tx_bcs.as_slice())?;
    self.sdk_default_gas_budget(sender_address, &tx).await
  }

  async fn get_previous_version(&self, iod: IotaObjectData) -> Result<Option<IotaObjectData>, Error> {
    // try to get digest of previous tx
    // if we requested the prev tx and it isn't returned, this should be the oldest state
    let prev_tx_digest = if let Some(value) = iod.previous_transaction {
      value
    } else {
      return Ok(None);
    };

    // resolve previous tx
    let prev_tx_response = self
      .iota_client
      .read_api()
      .get_transaction_with_options(
        prev_tx_digest,
        IotaTransactionBlockResponseOptions::new().with_object_changes(),
      )
      .await
      .map_err(|err| {
        Error::InvalidIdentityHistory(format!("could not get previous transaction {prev_tx_digest}; {err}"))
      })?;

    // check for updated/created changes
    let (created, other_changes): (Vec<ObjectChange>, _) = prev_tx_response
      .clone()
      .object_changes
      .ok_or_else(|| {
        Error::InvalidIdentityHistory(format!(
          "could not find object changes for object {} in transaction {prev_tx_digest}",
          iod.object_id
        ))
      })?
      .into_iter()
      .filter(|elem| iod.object_id.eq(&elem.object_id()))
      .partition(|elem| matches!(elem, ObjectChange::Created { .. }));

    // previous tx contain create tx, so there is no previous version
    if created.len() == 1 {
      return Ok(None);
    }

    let mut previous_versions: Vec<SequenceNumber> = other_changes
      .iter()
      .filter_map(|elem| match elem {
        ObjectChange::Mutated { previous_version, .. } => Some(*previous_version),
        _ => None,
      })
      .collect();

    previous_versions.sort();

    let earliest_previous = if let Some(value) = previous_versions.first() {
      value
    } else {
      return Ok(None); // no mutations in prev tx, so no more versions can be found
    };

    let past_obj_response = self.get_past_object(iod.object_id, *earliest_previous).await?;
    match past_obj_response {
      IotaPastObjectResponse::VersionFound(value) => Ok(Some(value)),
      _ => Err(Error::InvalidIdentityHistory(format!(
        "could not find previous version, past object response: {past_obj_response:?}"
      ))),
    }
  }

  async fn get_past_object(
    &self,
    object_id: ObjectID,
    version: SequenceNumber,
  ) -> Result<IotaPastObjectResponse, Error> {
    self
      .iota_client
      .read_api()
      .try_get_parsed_past_object(object_id, version, IotaObjectDataOptions::full_content())
      .await
      .map_err(|err| {
        Error::InvalidIdentityHistory(format!("could not look up object {object_id} version {version}; {err}"))
      })
  }
}

impl IotaClientRustSdk {
  pub fn new(iota_client: IotaClient) -> Result<Self, Error> {
    Ok(Self { iota_client })
  }

  async fn sdk_execute_transaction<S: Signer<IotaKeySignature>>(
    &self,
    sender_address: IotaAddress,
    sender_public_key: &[u8],
    tx: ProgrammableTransaction,
    gas_budget: Option<u64>,
    signer: &S,
  ) -> Result<IotaTransactionBlockResponse, Error> {
    let gas_budget = match gas_budget {
      Some(gas) => gas,
      None => self.sdk_default_gas_budget(sender_address, &tx).await?,
    };
    let tx_data = self.get_transaction_data(tx, gas_budget, sender_address).await?;
    let signature = Self::sign_transaction_data(signer, &tx_data, sender_public_key).await?;

    // execute tx
    let response = self
      .iota_client
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

  async fn sdk_default_gas_budget(
    &self,
    sender_address: IotaAddress,
    tx: &ProgrammableTransaction,
  ) -> Result<u64, Error> {
    let gas_price = self
      .iota_client
      .read_api()
      .get_reference_gas_price()
      .await
      .map_err(|e| Error::RpcError(e.to_string()))?;
    let gas_coin = self.get_coin_for_transaction(sender_address).await?;
    let tx_data = TransactionData::new_programmable(
      sender_address,
      vec![gas_coin.object_ref()],
      tx.clone(),
      50_000_000,
      gas_price,
    );
    let dry_run_gas_result = self
      .iota_client
      .read_api()
      .dry_run_transaction_block(tx_data)
      .await?
      .effects;
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

  async fn get_transaction_data(
    &self,
    programmable_transaction: ProgrammableTransaction,
    gas_budget: u64,
    sender_address: IotaAddress,
  ) -> Result<TransactionData, Error> {
    let gas_price = self
      .iota_client
      .read_api()
      .get_reference_gas_price()
      .await
      .map_err(|err| Error::GasIssue(format!("could not get gas price; {err}")))?;
    let coin = self.get_coin_for_transaction(sender_address).await?;
    let tx_data = TransactionData::new_programmable(
      sender_address,
      vec![coin.object_ref()],
      programmable_transaction,
      gas_budget,
      gas_price,
    );

    Ok(tx_data)
  }

  async fn sign_transaction_data<S: Signer<IotaKeySignature>>(
    signer: &S,
    tx_data: &TransactionData,
    sender_public_key: &[u8],
  ) -> Result<Signature, Error> {
    use fastcrypto::hash::HashFunction;

    let intent = Intent::iota_transaction();
    let intent_msg = IntentMessage::new(intent, tx_data);
    let mut hasher = Blake2b256::default();
    let bcs_bytes = bcs::to_bytes(&intent_msg).map_err(|err| {
      Error::TransactionSigningFailed(format!("could not serialize transaction message to bcs; {err}"))
    })?;
    hasher.update(bcs_bytes);
    let digest = hasher.finalize().digest;

    let raw_signature = signer
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

  async fn get_coin_for_transaction(&self, sender_address: IotaAddress) -> Result<Coin, Error> {
    const LIMIT: usize = 10;
    let mut cursor = None;

    loop {
      let coins = self
        .iota_client
        .coin_read_api()
        .get_coins(sender_address, None, cursor, Some(LIMIT))
        .await?;

      let Some(coin) = coins.data.into_iter().max_by_key(|coin| coin.balance) else {
        return Err(Error::GasIssue(format!(
          "no coins found for address {}",
          sender_address
        )));
      };

      if coin.balance >= MINIMUM_BALANCE {
        return Ok(coin);
      }

      if !coins.has_next_page {
        break;
      }

      cursor = coins.next_cursor;
    }

    Err(Error::GasIssue(format!(
      "no coin found with minimum required balance of {} for address {}",
      MINIMUM_BALANCE,
      sender_address
    )))
  }
}
