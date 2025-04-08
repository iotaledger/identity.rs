// Copyright 2020-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::rc::Rc;

use identity_iota::iota::rebased::migration::ControllerToken;
use identity_iota::iota::rebased::migration::CreateIdentity;
use identity_iota::iota::rebased::migration::IdentityBuilder;
use identity_iota::iota::rebased::migration::OnChainIdentity;
use identity_iota::iota::rebased::transaction_builder::Transaction;
use identity_iota::iota::IotaDocument;
use iota_interaction_ts::bindings::WasmIotaTransactionBlockEffects;
use iota_interaction_ts::error::WasmResult as _;
use tokio::sync::RwLock;
use wasm_bindgen::prelude::*;

use crate::error::wasm_error;
use crate::error::Result;
use crate::error::WasmResult;
use crate::iota::WasmIotaDocument;
use crate::rebased::proposals::WasmCreateConfigChangeProposal;
use crate::rebased::proposals::WasmCreateUpdateDidProposal;

use super::proposals::StringCouple;
use super::proposals::WasmConfigChange;
use super::proposals::WasmCreateSendProposal;
use super::WasmIdentityClient;
// use super::proposals::StringCouple;
// use super::proposals::WasmConfigChange;
// use super::proposals::WasmCreateConfigChangeProposalTx;
// use super::proposals::WasmCreateSendProposalTx;
// use super::proposals::WasmCreateUpdateDidProposalTx;
use super::WasmIdentityClientReadOnly;
use super::WasmIotaAddress;
use super::WasmTransactionBuilder;

// Helper type for `WasmIdentityBuilder::controllers`.
// Has getters to support `Clone` for serialization
#[derive(Debug)]
#[wasm_bindgen(getter_with_clone)]
pub struct ControllerAndVotingPower(pub WasmIotaAddress, pub u64);

#[wasm_bindgen(js_class = ControllerAndVotingPower)]
impl ControllerAndVotingPower {
  #[wasm_bindgen(constructor)]
  pub fn new(address: WasmIotaAddress, voting_power: u64) -> Self {
    Self(address, voting_power)
  }
}

#[derive(Clone)]
#[wasm_bindgen(js_name = ControllerToken)]
pub struct WasmControllerToken(pub(crate) ControllerToken);

#[wasm_bindgen(js_class = ControllerToken)]
impl WasmControllerToken {
  #[wasm_bindgen]
  pub fn id(&self) -> String {
    self.0.id().to_string()
  }

  #[wasm_bindgen(js_name = controllerOf)]
  pub fn controller_of(&self) -> String {
    self.0.controller_of().to_string()
  }
}

#[wasm_bindgen(js_name = OnChainIdentity)]
#[derive(Clone)]
pub struct WasmOnChainIdentity(pub(crate) Rc<RwLock<OnChainIdentity>>);

#[wasm_bindgen(js_class = OnChainIdentity)]
impl WasmOnChainIdentity {
  pub(crate) fn new(identity: OnChainIdentity) -> Self {
    Self(Rc::new(RwLock::new(identity)))
  }

  #[wasm_bindgen]
  pub fn id(&self) -> Result<String> {
    Ok(self.0.try_read().wasm_result()?.id().to_string())
  }

  #[wasm_bindgen(js_name = didDocument)]
  pub fn did_document(&self) -> Result<WasmIotaDocument> {
    let inner_doc = self.0.try_read().wasm_result()?.did_document().clone();
    Ok(WasmIotaDocument::from(inner_doc))
  }

  /// Returns whether the {@link IotaDocument} contained in this {@link OnChainIdentity} has been deleted.
  /// Once a DID Document is deleted, it cannot be reactivated.
  ///
  /// When calling {@link OnChainIdentity.did_document} on an Identity whose DID Document
  /// had been deleted, an *empty* and *deactivated* {@link IotaDocument} will be returned.
  #[wasm_bindgen(js_name = hasDeletedDid)]
  pub fn has_deleted_did(&self) -> Result<bool> {
    self
      .0
      .try_read()
      .wasm_result()
      .map(|identity| identity.has_deleted_did())
  }

  #[wasm_bindgen(js_name = isShared)]
  pub fn is_shared(&self) -> Result<bool> {
    Ok(self.0.try_read().wasm_result()?.is_shared())
  }

  #[wasm_bindgen(js_name = getControllerToken)]
  pub async fn get_controller_token(&self, client: &WasmIdentityClient) -> Result<Option<WasmControllerToken>> {
    let maybe_controller_token = self
      .0
      .read()
      .await
      .get_controller_token(&client.0)
      .await
      .wasm_result()?
      .map(WasmControllerToken);
    Ok(maybe_controller_token)
  }

  #[wasm_bindgen(skip_typescript)] // ts type in custom section below
  pub fn proposals(&self) -> Result<JsValue> {
    let lock = self.0.try_read().wasm_result()?;
    let proposals = lock.proposals();
    serde_wasm_bindgen::to_value(proposals).map_err(wasm_error)
  }

  #[wasm_bindgen(
    js_name = updateDidDocument,
    unchecked_return_type = "TransactionBuilder<CreateProposal<UpdateDid>>",
  )]
  pub fn update_did_document(
    &self,
    updated_doc: &WasmIotaDocument,
    controller_token: &WasmControllerToken,
    expiration_epoch: Option<u64>,
  ) -> WasmTransactionBuilder {
    let create_proposal_tx =
      WasmCreateUpdateDidProposal::new(self, updated_doc.clone(), controller_token.clone(), expiration_epoch);
    WasmTransactionBuilder::new(JsValue::from(create_proposal_tx).unchecked_into())
  }

  #[wasm_bindgen(
    js_name = deactivateDid,
    unchecked_return_type = "TransactionBuilder<CreateProposal<UpdateDid>>",
  )]
  pub fn deactivate_did(
    &self,
    controller_token: &WasmControllerToken,
    expiration_epoch: Option<u64>,
  ) -> WasmTransactionBuilder {
    let create_proposal_tx = WasmCreateUpdateDidProposal::deactivate(self, controller_token.clone(), expiration_epoch);
    WasmTransactionBuilder::new(JsValue::from(create_proposal_tx).unchecked_into())
  }

  #[wasm_bindgen(
    js_name = deleteDid,
    unchecked_return_type = "TransactionBuilder<CreateProposal<UpdateDid>>",
  )]
  pub fn delete_did(
    &self,
    controller_token: &WasmControllerToken,
    expiration_epoch: Option<u64>
  ) -> WasmTransactionBuilder {
    let tx = WasmCreateUpdateDidProposal::delete(self, controller_token.clone(), expiration_epoch);
    WasmTransactionBuilder::new(JsValue::from(create_proposal_tx).unchecked_into())
  }

  #[wasm_bindgen(
    js_name = updateConfig,
    unchecked_return_type = "TransactionBuilder<CreateProposal<ConfigChange>>",
  )]
  pub fn update_config(
    &self,
    controller_token: &WasmControllerToken,
    config: WasmConfigChange,
    expiration_epoch: Option<u64>,
  ) -> WasmTransactionBuilder {
    let tx = JsValue::from(WasmCreateConfigChangeProposal::new(
      self,
      controller_token,
      config,
      expiration_epoch,
    ));
    WasmTransactionBuilder::new(tx.unchecked_into())
  }

  #[wasm_bindgen(
    js_name = sendAssets,
    unchecked_return_type = "TransactionBuilder<CreateProposal<SendAction>>",
  )]
  pub fn send_assets(
    &self,
    controller_token: &WasmControllerToken,
    transfer_map: Vec<StringCouple>,
    expiration_epoch: Option<u64>,
  ) -> Result<WasmTransactionBuilder> {
    let tx = WasmCreateSendProposal::new(self, controller_token, transfer_map, expiration_epoch).wasm_result()?;
    Ok(WasmTransactionBuilder::new(JsValue::from(tx).unchecked_into()))
  }
}
//   #[allow(unused)] // API will be updated in the future
//   #[wasm_bindgen(js_name = getHistory, skip_typescript)] // ts type in custom section below
//   pub async fn get_history(
//     &self,
//     _client: WasmIdentityClient,
//     _last_version: Option<WasmIotaObjectData>,
//     _page_size: Option<usize>,
//   ) -> Result<JsValue> {
//     unimplemented!("WasmOnChainIdentity::get_history");
//     // let rs_history = self
//     //   .0
//     //   .get_history(
//     //     &client.0,
//     //     last_version.map(|lv| into_sdk_type(lv).unwrap()).as_ref(),
//     //     page_size,
//     //   )
//     //   .await
//     //   .map_err(wasm_error)?;
//     // serde_wasm_bindgen::to_value(&rs_history).map_err(wasm_error)
//   }
// }

// // Manually add the method to the interface.
// #[wasm_bindgen(typescript_custom_section)]
// const WASM_ON_CHAIN_IDENTITY_TYPES: &str = r###"
// 	export interface OnChainIdentity {
// 		proposals(): Map<String, Proposal>;
// 		getHistory(): Map<String, Proposal>;
// 	}
// "###;

#[wasm_bindgen(js_name = IdentityBuilder)]
pub struct WasmIdentityBuilder(pub(crate) IdentityBuilder);

#[wasm_bindgen(js_class = IdentityBuilder)]
impl WasmIdentityBuilder {
  #[wasm_bindgen(constructor)]
  pub fn new(did_doc: &WasmIotaDocument) -> Result<WasmIdentityBuilder> {
    let document: IotaDocument = did_doc.0.try_read().unwrap().clone();
    Ok(WasmIdentityBuilder(IdentityBuilder::new(document)))
  }

  pub fn controller(self, address: WasmIotaAddress, voting_power: u64) -> Self {
    Self(
      self.0.controller(
        address
          .parse()
          .expect("Parameter address could not be parsed into valid IotaAddress"),
        voting_power,
      ),
    )
  }

  pub fn threshold(self, threshold: u64) -> Self {
    Self(self.0.threshold(threshold))
  }

  pub fn controllers(self, controllers: Vec<ControllerAndVotingPower>) -> Self {
    Self(
      self.0.controllers(
        controllers
          .into_iter()
          .map(|v| {
            (
              v.0
                .parse()
                .expect("controller can not be parsed into valid IotaAddress"),
              v.1,
            )
          })
          .collect::<Vec<_>>(),
      ),
    )
  }

  #[wasm_bindgen(unchecked_return_type = "TransactionBuilder<CreateIdentity>")]
  pub fn finish(self) -> WasmTransactionBuilder {
    WasmTransactionBuilder::new(JsValue::from(WasmCreateIdentity::new(self)).unchecked_into())
  }
}

#[wasm_bindgen(js_name = CreateIdentity)]
pub struct WasmCreateIdentity(pub(crate) CreateIdentity);

#[wasm_bindgen(js_class = CreateIdentity)]
impl WasmCreateIdentity {
  #[wasm_bindgen(constructor)]
  pub fn new(builder: WasmIdentityBuilder) -> Self {
    Self(CreateIdentity::new(builder.0))
  }

  #[wasm_bindgen(js_name = buildProgrammableTransaction)]
  pub async fn build_programmable_transaction(&self, client: &WasmIdentityClientReadOnly) -> Result<Vec<u8>> {
    let pt = self.0.build_programmable_transaction(&client.0).await.wasm_result()?;
    bcs::to_bytes(&pt).wasm_result()
  }

  #[wasm_bindgen]
  pub async fn apply(
    self,
    effects: &WasmIotaTransactionBlockEffects,
    client: &WasmIdentityClientReadOnly,
  ) -> Result<WasmOnChainIdentity> {
    self
      .0
      .apply(&effects.clone().into(), &client.0)
      .await
      .wasm_result()
      .map(WasmOnChainIdentity::new)
  }
}
