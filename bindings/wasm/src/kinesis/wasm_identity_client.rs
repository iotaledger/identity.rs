// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::rc::Rc;

use identity_iota::iota::iota_sdk_abstraction::DummySigner;
use identity_iota::iota::iota_sdk_abstraction::Identity;
use identity_iota::iota::iota_sdk_abstraction::IdentityClient;
use identity_iota::iota::iota_sdk_abstraction::types::base_types::{IotaAddress, ObjectID};
use identity_iota::iota::IotaDocument;
use wasm_bindgen::prelude::*;

use crate::iota::IotaDocumentLock;
use crate::iota::WasmIotaDID;
use crate::iota::WasmIotaDocument;

use super::wasm_identity_client_builder::WasmKinesisIdentityClientBuilder;
use super::WasmIdentityBuilder;
use super::ts_client_sdk::IotaClientTsSdk;
use super::types::{WasmIotaAddress, WasmObjectID};

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
    self.0
        .sender_address()
        .map(|a| a.to_string())
        .map_err(|e| e.into())
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

  #[wasm_bindgen(js_name = resolveDid)]
  pub async fn resolve_did(&self, did: &WasmIotaDID) -> Result<WasmIotaDocument, JsError> {
    let document = self
      .0
      .resolve_did(&did.0)
      .await
      .map_err(<identity_iota::iota::iota_sdk_abstraction::Error as std::convert::Into<JsError>>::into)?;
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
      .map_err(<identity_iota::iota::iota_sdk_abstraction::Error as std::convert::Into<JsError>>::into)?;

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
      .map_err(<identity_iota::iota::iota_sdk_abstraction::Error as std::convert::Into<JsError>>::into)?;

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
      .map_err(<identity_iota::iota::iota_sdk_abstraction::Error as std::convert::Into<JsError>>::into)?;

    Ok(())
  }

  // test function(s) for wasm calling test

  // make test call
  #[wasm_bindgen(js_name = getBalance)]
  pub async fn get_balance(&self) -> Result<String, JsError> {
    IdentityClient::get_chain_identifier(&self.0)
      .await
      .map_err(|err| JsError::new(&format!("could not get balance; {err}")))
  }
}
