// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::ops::Deref;
use std::rc::Rc;

use identity_iota::iota::rebased::client::IdentityClient;
use identity_iota::iota::rebased::client::PublishDidTx;
use identity_iota::iota::rebased::transaction::TransactionInternal;
use identity_iota::iota::rebased::transaction::TransactionOutputInternal;

use iota_interaction_ts::bindings::WasmExecutionStatus;
use iota_interaction_ts::bindings::WasmOwnedObjectRef;
use iota_interaction_ts::WasmPublicKey;

use identity_iota::iota::rebased::Error;
use iota_interaction_ts::NativeTransactionBlockResponse;

use super::IdentityContainer;
use super::WasmIdentityBuilder;
use super::WasmIdentityClientReadOnly;
use super::WasmIotaAddress;
use super::WasmObjectID;

use crate::error::wasm_error;
use crate::iota::IotaDocumentLock;
use crate::iota::WasmIotaDID;
use crate::iota::WasmIotaDocument;
use crate::storage::WasmTransactionSigner;
use identity_iota::iota::IotaDocument;
use wasm_bindgen::prelude::*;

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

#[wasm_bindgen(js_name = IdentityClient)]
pub struct WasmIdentityClient(pub(crate) IdentityClient<WasmTransactionSigner>);

impl Deref for WasmIdentityClient {
  type Target = IdentityClient<WasmTransactionSigner>;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

#[wasm_bindgen(js_class = IdentityClient)]
impl WasmIdentityClient {
  #[wasm_bindgen(js_name = create)]
  pub async fn new(
    client: WasmIdentityClientReadOnly,
    signer: WasmTransactionSigner,
  ) -> Result<WasmIdentityClient, JsError> {
    let inner_client = IdentityClient::new(client.0, signer).await?;
    Ok(WasmIdentityClient(inner_client))
  }

  #[wasm_bindgen(js_name = senderPublicKey)]
  pub fn sender_public_key(&self) -> Result<WasmPublicKey, JsValue> {
    self.0.sender_public_key().try_into()
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
  pub fn create_identity(&self, iota_document: &WasmIotaDocument) -> Result<WasmIdentityBuilder, JsError> {
    WasmIdentityBuilder::new(iota_document)
      .map_err(|err| JsError::new(&format!("failed to initialize new identity builder; {err:?}")))
  }

  #[wasm_bindgen(js_name = getIdentity)]
  pub async fn get_identity(&self, object_id: WasmObjectID) -> Result<IdentityContainer, JsError> {
    let inner_value = self.0.get_identity(object_id.parse()?).await.unwrap();
    Ok(IdentityContainer(inner_value))
  }

  #[wasm_bindgen(js_name = packageId)]
  pub fn package_id(&self) -> String {
    self.0.package_id().to_string()
  }

  #[wasm_bindgen(js_name = resolveDid)]
  pub async fn resolve_did(&self, did: &WasmIotaDID) -> Result<WasmIotaDocument, JsError> {
    let document = self.0.resolve_did(&did.0).await.map_err(JsError::from)?;
    Ok(WasmIotaDocument(Rc::new(IotaDocumentLock::new(document))))
  }

  #[wasm_bindgen(js_name = publishDidDocument)]
  pub fn publish_did_document(&self, document: &WasmIotaDocument) -> Result<WasmPublishDidTx, JsError> {
    let doc: IotaDocument = document
      .0
      .try_read()
      .map_err(|err| JsError::new(&format!("failed to read DID document; {err:?}")))?
      .clone();

    Ok(WasmPublishDidTx(self.0.publish_did_document(doc)))
  }

  #[wasm_bindgen(js_name = publishDidDocumentUpdate)]
  pub async fn publish_did_document_update(
    &self,
    document: &WasmIotaDocument,
    gas_budget: u64,
  ) -> Result<WasmIotaDocument, JsError> {
    let doc: IotaDocument = document
      .0
      .try_read()
      .map_err(|err| JsError::new(&format!("failed to read DID document; {err:?}")))?
      .clone();
    let document = self
      .0
      .publish_did_document_update(doc, gas_budget)
      .await
      .map_err(<Error as std::convert::Into<JsError>>::into)?;

    Ok(WasmIotaDocument(Rc::new(IotaDocumentLock::new(document))))
  }

  #[wasm_bindgen(js_name = deactivateDidOutput)]
  pub async fn deactivate_did_output(&self, did: &WasmIotaDID, gas_budget: u64) -> Result<(), JsError> {
    self
      .0
      .deactivate_did_output(&did.0, gas_budget)
      .await
      .map_err(<Error as std::convert::Into<JsError>>::into)?;

    Ok(())
  }
}

// TODO: rethink how to organize the following types and impls
#[wasm_bindgen(js_name = PublishDidTx)]
pub struct WasmPublishDidTx(pub(crate) PublishDidTx);

#[wasm_bindgen(js_class = PublishDidTx)]
impl WasmPublishDidTx {
  #[wasm_bindgen(js_name = execute)]
  pub async fn execute(self, client: &WasmIdentityClient) -> Result<WasmTransactionOutputPublishDid, JsValue> {
    let output = self.0.execute(&client.0).await.map_err(wasm_error)?;
    Ok(WasmTransactionOutputPublishDid(output))
  }
}

#[wasm_bindgen(js_name = TransactionOutputInternalIotaDocument)]
pub struct WasmTransactionOutputPublishDid(pub(crate) TransactionOutputInternal<IotaDocument>);

#[wasm_bindgen(js_class = TransactionOutputInternalIotaDocument)]
impl WasmTransactionOutputPublishDid {
  #[wasm_bindgen(getter)]
  pub fn output(&self) -> WasmIotaDocument {
    self.0.output.clone().into()
  }

  #[wasm_bindgen(getter)]
  pub fn response(&self) -> NativeTransactionBlockResponse {
    self.0.response.clone_native_response()
  }
}
