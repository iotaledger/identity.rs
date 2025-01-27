// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::rc::Rc;

use fastcrypto::ed25519::Ed25519PublicKey;
use fastcrypto::traits::ToFromBytes;

use identity_iota::iota::rebased::client::IdentityClient;
use identity_iota::iota_interaction::types::base_types::IotaAddress;
use identity_iota::iota_interaction::SignatureBcs;

use identity_iota::core::Base;
use identity_iota::core::BaseEncoding;

use iota_interaction_ts::bindings::WasmExecutionStatus;
use iota_interaction_ts::bindings::WasmOwnedObjectRef;
use iota_interaction_ts::iota_client_ts_sdk::IotaClientTsSdk;

use identity_iota::iota::rebased::Error;

use super::IdentityContainer;
use super::WasmIdentityBuilder;
use super::WasmKinesisIdentityClientReadOnly;

use crate::iota::IotaDocumentLock;
use crate::iota::WasmIotaDID;
use crate::iota::WasmIotaDocument;
use crate::storage::StorageSignerOwned;
use crate::storage::WasmStorageSigner;
use identity_iota::iota::IotaDocument;
use wasm_bindgen::prelude::*;

use super::types::WasmIotaAddress;
use super::types::WasmObjectID;

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
pub struct WasmKinesisIdentityClient(pub(crate) IdentityClient<StorageSignerOwned>);

// builder related functions
#[wasm_bindgen(js_class = KinesisIdentityClient)]
impl WasmKinesisIdentityClient {
  #[wasm_bindgen(js_name = create)]
  pub async fn new(client: WasmKinesisIdentityClientReadOnly, signer: WasmStorageSigner) -> Result<WasmKinesisIdentityClient, JsError> {
    let inner_client = IdentityClient::new(client.0, signer.0).await?;
    Ok(WasmKinesisIdentityClient(inner_client))
  }

  #[wasm_bindgen(js_name = senderPublicKey)]
  pub fn sender_public_key(&self) -> Vec<u8> {
    self.0.sender_public_key().to_vec()
  }

  #[wasm_bindgen(js_name = senderAddress)]
  pub fn sender_address(&self) -> WasmIotaAddress {
    self.0.sender_address().to_string()
  }

  #[wasm_bindgen(js_name = network)]
  pub fn network(&self) -> String {
    self.0.network().to_string()
  }
  
  #[wasm_bindgen(js_name = migrationRegistryId)]
  pub fn migration_registry_id(&self) -> String {
    self.0.migration_registry_id().to_string()
  }

  #[wasm_bindgen(js_name = createIdentity)]
  pub fn create_identity(&self, iota_document: &WasmIotaDocument) -> WasmIdentityBuilder {
    WasmIdentityBuilder::new(iota_document)
  }

  // #[wasm_bindgen(js_name = getIdentity)]
  // pub async fn get_identity(&self, object_id: WasmObjectID) -> Result<WasmIdentity, JsError> {
  //   let identity = self.0.get_identity(object_id.parse()?).await.map_err(|e| e.into())?;
  //   Ok(WasmIdentity(identity))
  // }

  #[wasm_bindgen(js_name = getIdentity)]
  pub async fn get_identity(&self, object_id: WasmObjectID) -> Result<IdentityContainer, JsError> {
    let inner_value = self.0.get_identity(object_id.parse()?).await.unwrap();
    Ok(IdentityContainer(inner_value))
  }

  #[wasm_bindgen(js_name = packageId)]
  pub fn package_id(&self) -> Result<String, JsError> {
    Ok(self.0.package_id().to_string())
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

  // not included in any e2e test anymore, so let's skip it for now

  // #[wasm_bindgen(js_name = publishDidDocument)]
  // pub async fn publish_did_document(
  //   &self,
  //   document: &WasmIotaDocument,
  // ) -> Result<(), JsError> {
  //   let doc: IotaDocument = document
  //     .0
  //     .try_read()
  //     .map_err(|err| JsError::new(&format!("failed to read DID document; {err:?}")))?
  //     .clone();
  //   let publish_tx = self
  //     .0
  //     .publish_did_document(doc);
  //     // .await
  //     // .map_err(<Error as std::convert::Into<JsError>>::into)?;

  //   // Ok(WasmIotaDocument(Rc::new(IotaDocumentLock::new(document))))
  //   Ok(())
  // }

  // #[wasm_bindgen(js_name = publishDidDocumentUpdate)]
  // pub async fn publish_did_document_update(
  //   &self,
  //   document: &WasmIotaDocument,
  //   gas_budget: u64,
  //   signer: &WasmStorageSigner,
  // ) -> Result<WasmIotaDocument, JsError> {
  //   let doc: IotaDocument = document
  //     .0
  //     .try_read()
  //     .map_err(|err| JsError::new(&format!("failed to read DID document; {err:?}")))?
  //     .clone();
  //   let document = self
  //     .0
  //     .publish_did_document_update(doc, gas_budget)
  //     .await
  //     .map_err(<Error as std::convert::Into<JsError>>::into)?;

  //   Ok(WasmIotaDocument(Rc::new(IotaDocumentLock::new(document))))
  // }

  // #[wasm_bindgen(js_name = deactivateDidOutput)]
  // pub async fn deactivate_did_output(
  //   &self,
  //   did: &WasmIotaDID,
  //   gas_budget: u64,
  //   signer: &WasmStorageSigner,
  // ) -> Result<(), JsError> {
  //   self
  //     .0
  //     .deactivate_did_output(&did.0, gas_budget)
  //     .await
  //     .map_err(<Error as std::convert::Into<JsError>>::into)?;

  //   Ok(())
  // }
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
