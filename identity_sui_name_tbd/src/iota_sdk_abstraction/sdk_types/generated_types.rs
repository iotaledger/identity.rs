// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use fastcrypto::encoding::Base64;
use serde::Deserialize;
use serde::Serialize;

use crate::iota_sdk_abstraction::SignatureBcs;
use crate::iota_sdk_abstraction::TransactionDataBcs;

use super::iota_json_rpc_types::iota_transaction::IotaTransactionBlockResponseOptions;
use super::iota_json_rpc_types::IotaObjectDataOptions;
use super::iota_types::base_types::SequenceNumber;
use super::iota_types::dynamic_field::DynamicFieldName;
use super::iota_types::quorum_driver_types::ExecuteTransactionRequestType;

// The types defined in this file:
// * do not exist in the iota rust sdk
// * have an equivalent type in the iota typescript sdk
// * are needed for wasm-bindings
// * have been generated by @iota/sdk/typescript/scripts/generate.ts
//
// As there is no equivalent rust type in the iota rust sdk, we need to
// define equivalent rust types here.

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteTransactionBlockParams {
  /// BCS serialized transaction data bytes without its type tag, as base-64 encoded string.
  transaction_block: Base64,
  /// A list of signatures (`flag || signature || pubkey` bytes, as base-64 encoded string). Signature is committed to
  /// the intent message of the transaction data, as base-64 encoded string.
  signature: Vec<Base64>,
  /// options for specifying the content to be returned
  options: Option<IotaTransactionBlockResponseOptions>,
  /// The request type, derived from `IotaTransactionBlockResponseOptions` if None
  request_type: Option<ExecuteTransactionRequestType>,
}

impl ExecuteTransactionBlockParams {
  pub fn new(
    tx_bytes: &TransactionDataBcs,
    signatures: &Vec<SignatureBcs>,
    options: Option<IotaTransactionBlockResponseOptions>,
    request_type: Option<ExecuteTransactionRequestType>,
  ) -> Self {
    ExecuteTransactionBlockParams {
      transaction_block: Base64::from_bytes(&tx_bytes),
      signature: signatures.into_iter().map(|sig| Base64::from_bytes(&sig)).collect(),
      options,
      request_type,
    }
  }
}

/// Return the dynamic field object information for a specified object
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDynamicFieldObjectParams {
  /// The ID of the queried parent object
  parent_id: String,
  /// The Name of the dynamic field
  name: DynamicFieldName,
}

impl GetDynamicFieldObjectParams {
  pub fn new(parent_id: String, name: DynamicFieldName) -> Self {
    GetDynamicFieldObjectParams { parent_id, name }
  }
}

/// Return the object information for a specified object
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetObjectParams {
  /// the ID of the queried object
  id: String,
  /// options for specifying the content to be returned
  options: Option<IotaObjectDataOptions>,
}

impl GetObjectParams {
  pub fn new(id: String, options: Option<IotaObjectDataOptions>) -> Self {
    GetObjectParams { id, options }
  }
}

/// Return the list of objects owned by an address. Note that if the address owns more than
/// `QUERY_MAX_RESULT_LIMIT` objects, the pagination is not accurate, because previous page may have
/// been updated when the next page is fetched. Please use iotax_queryObjects if this is a concern.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOwnedObjectsParams {
  /// the owner's Iota address
  owner: String,
  /// An optional paging cursor. If provided, the query will start from the next item after the specified
  /// cursor. Default to start from the first item if not specified.
  cursor: Option<String>,
  /// Max number of items returned per page, default to [QUERY_MAX_RESULT_LIMIT] if not specified.
  limit: Option<usize>,
}

impl GetOwnedObjectsParams {
  pub fn new(owner: String, cursor: Option<String>, limit: Option<usize>) -> Self {
    GetOwnedObjectsParams { owner, cursor, limit }
  }
}

/// Note there is no software-level guarantee/SLA that objects with past versions can be retrieved by
/// this API, even if the object and version exists/existed. The result may vary across nodes depending
/// on their pruning policies. Return the object information for a specified version
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TryGetPastObjectParams {
  /// the ID of the queried object
  id: String,
  /// the version of the queried object. If None, default to the latest known version
  version: SequenceNumber,
  //// options for specifying the content to be returned
  options: Option<IotaObjectDataOptions>,
}

impl TryGetPastObjectParams {
  pub fn new(id: String, version: SequenceNumber, options: Option<IotaObjectDataOptions>) -> Self {
    TryGetPastObjectParams { id, version, options }
  }
}
