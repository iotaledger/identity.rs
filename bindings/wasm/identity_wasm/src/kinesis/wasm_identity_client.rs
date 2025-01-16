// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::rc::Rc;

use fastcrypto::ed25519::Ed25519PublicKey;
use fastcrypto::traits::ToFromBytes;

use identity_iota::iota_interaction::types::base_types::IotaAddress;
use identity_iota::iota_interaction::SignatureBcs;

use identity_iota::core::Base;
use identity_iota::core::BaseEncoding;

use iota_interaction_ts::bindings::WasmExecutionStatus;
use iota_interaction_ts::bindings::WasmOwnedObjectRef;
use iota_interaction_ts::iota_client_ts_sdk::IotaClientTsSdk;

use identity_iota::iota::rebased::Error;

use super::client_dummy::DummySigner;
use super::client_dummy::Identity;
use super::client_dummy::IdentityClient;

use crate::iota::IotaDocumentLock;
use crate::iota::WasmIotaDID;
use crate::iota::WasmIotaDocument;
use identity_iota::iota::IotaDocument;
use wasm_bindgen::prelude::*;

use super::types::WasmIotaAddress;
use super::types::WasmObjectID;
use super::wasm_identity_client_builder::WasmKinesisIdentityClientBuilder;
use super::WasmIdentityBuilder;

#[wasm_bindgen(getter_with_clone, inspectable, js_name = IotaTransactionBlockResponseEssence)]
pub struct WasmIotaTransactionBlockResponseEssence {
  #[wasm_bindgen(js_name = effectsExist)]
  pub effects_exist: bool,
  pub effects: String,
  #[wasm_bindgen(js_name = effectsExecutionStatus)]
  pub effects_execution_status: Option<WasmExecutionStatus>,
  #[wasm_bindgen(js_name = effectsCreated)]
  pub effects_created: Option<Vec<WasmOwnedObjectRef>>,
}

#[wasm_bindgen(js_name = KinesisIdentityClient)]
pub struct WasmKinesisIdentityClient(pub(crate) IdentityClient<IotaClientTsSdk>);

// builder related functions
#[wasm_bindgen(js_class = KinesisIdentityClient)]
impl WasmKinesisIdentityClient {
  #[wasm_bindgen]
  pub fn builder() -> WasmKinesisIdentityClientBuilder {
    // WasmKinesisIdentityClientBuilder::default()
    WasmKinesisIdentityClientBuilder(IdentityClient::<IotaClientTsSdk>::builder())
  }

  // mock functions for wasm integration

  #[wasm_bindgen(js_name = senderPublicKey)]
  pub fn sender_public_key(&self) -> Result<Vec<u8>, JsError> {
    self.0.sender_public_key().map(|v| v.to_vec()).map_err(|e| e.into())
  }

  #[wasm_bindgen(js_name = senderAddress)]
  pub fn sender_address(&self) -> Result<WasmIotaAddress, JsError> {
    self.0.sender_address().map(|a| a.to_string()).map_err(|e| e.into())
  }

  #[wasm_bindgen(js_name = networkName)]
  pub fn network_name(&self) -> String {
    self.0.network_name().to_string()
  }

  #[wasm_bindgen(js_name = createIdentity)]
  pub fn create_identity(&self, iota_document: &[u8]) -> WasmIdentityBuilder {
    WasmIdentityBuilder(self.0.create_identity(iota_document))
  }

  #[wasm_bindgen(js_name = getIdentity)]
  pub async fn get_identity(&self, object_id: WasmObjectID) -> Result<Identity, JsError> {
    self.0.get_identity(object_id.parse()?).await.map_err(|e| e.into())
  }

  #[wasm_bindgen(js_name = executeDummyTransaction)]
  pub async fn execute_dummy_transaction(
    &self,
    tx_data_bcs_str: String,
    signatures_str: Vec<String>,
  ) -> Result<WasmIotaTransactionBlockResponseEssence, JsError> {
    let dummy = 1;
    let tx_data_bcs = BaseEncoding::decode(tx_data_bcs_str.as_str(), Base::Base64Pad)?;
    let signatures = signatures_str
      .iter()
      .map(|s| BaseEncoding::decode(s, Base::Base64Pad))
      .collect::<std::result::Result<Vec<SignatureBcs>, _>>()?;

    let response = self
      .0
      .execute_dummy_transaction(tx_data_bcs, signatures)
      .await
      .map_err(<Error as Into<JsError>>::into)?;

    let effects_execution_status: Option<WasmExecutionStatus> = response
      .effects_execution_status()
      .map(|status| serde_wasm_bindgen::to_value(&status).unwrap().into());

    let effects_created: Option<Vec<WasmOwnedObjectRef>> = response.effects_created().map(|effects| {
      effects
        .into_iter()
        .map(|efct| serde_wasm_bindgen::to_value(&efct).unwrap().into())
        .collect()
    });

    Ok(WasmIotaTransactionBlockResponseEssence {
      effects_exist: response.effects_is_some(),
      effects: response.to_string(),
      effects_execution_status,
      effects_created,
    })
  }

  #[wasm_bindgen(js_name = resolveDid)]
  pub async fn resolve_did(&self, did: &WasmIotaDID) -> Result<WasmIotaDocument, JsError> {
    let document = self
      .0
      .resolve_did(&did.0)
      .await
      .map_err(<identity_iota::iota::rebased::Error as std::convert::Into<JsError>>::into)?;
    Ok(WasmIotaDocument(Rc::new(IotaDocumentLock::new(document))))
  }

  #[wasm_bindgen(js_name = publishDidDocument)]
  pub async fn publish_did_document(
    &self,
    document: &WasmIotaDocument,
    gas_budget: u64,
    signer: &DummySigner,
  ) -> Result<WasmIotaDocument, JsError> {
    let doc: IotaDocument = document
      .0
      .try_read()
      .map_err(|err| JsError::new(&format!("failed to read DID document; {err:?}")))?
      .clone();
    let document = self
      .0
      .publish_did_document(doc, gas_budget, signer)
      .await
      .map_err(<Error as std::convert::Into<JsError>>::into)?;

    Ok(WasmIotaDocument(Rc::new(IotaDocumentLock::new(document))))
  }

  #[wasm_bindgen(js_name = publishDidDocumentUpdate)]
  pub async fn publish_did_document_update(
    &self,
    document: &WasmIotaDocument,
    gas_budget: u64,
    signer: &DummySigner,
  ) -> Result<WasmIotaDocument, JsError> {
    let doc: IotaDocument = document
      .0
      .try_read()
      .map_err(|err| JsError::new(&format!("failed to read DID document; {err:?}")))?
      .clone();
    let document = self
      .0
      .publish_did_document_update(doc, gas_budget, signer)
      .await
      .map_err(<Error as std::convert::Into<JsError>>::into)?;

    Ok(WasmIotaDocument(Rc::new(IotaDocumentLock::new(document))))
  }

  #[wasm_bindgen(js_name = deactivateDidOutput)]
  pub async fn deactivate_did_output(
    &self,
    did: &WasmIotaDID,
    gas_budget: u64,
    signer: &DummySigner,
  ) -> Result<(), JsError> {
    self
      .0
      .deactivate_did_output(&did.0, gas_budget, signer)
      .await
      .map_err(<Error as std::convert::Into<JsError>>::into)?;

    Ok(())
  }

  // test function(s) for wasm calling test

  // make test call
  #[wasm_bindgen(js_name = getChainIdentifier)]
  pub async fn get_chain_identifier(&self) -> Result<String, JsError> {
    IdentityClient::get_chain_identifier(&self.0)
      .await
      .map_err(|err| JsError::new(&format!("could not get balance; {err}")))
  }
}

/// TODO: consider importing function from rebased later on, if possible
pub fn convert_to_address(sender_public_key: &[u8]) -> Result<IotaAddress, Error> {
  let public_key = Ed25519PublicKey::from_bytes(sender_public_key)
    .map_err(|err| Error::InvalidKey(format!("could not parse public key to Ed25519 public key; {err}")))?;

  Ok(IotaAddress::from(&public_key))
}

#[wasm_bindgen(js_name = convertToAddress)]
pub fn wasm_convert_to_address(sender_public_key: &[u8]) -> Result<String, JsError> {
  convert_to_address(sender_public_key)
    .map(|v| v.to_string())
    .map_err(|err| JsError::new(&format!("could not derive address from public key; {err}")))
}
