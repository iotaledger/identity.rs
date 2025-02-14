// Copyright 2020-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::rc::Rc;

use identity_iota::iota::rebased::migration::CreateIdentityTx;
use identity_iota::iota::rebased::migration::IdentityBuilder;
use identity_iota::iota::rebased::migration::OnChainIdentity;
use identity_iota::iota::rebased::proposals::ProposalResult;
use identity_iota::iota::rebased::transaction::TransactionInternal;
use identity_iota::iota::rebased::transaction::TransactionOutputInternal;
use identity_iota::iota::IotaDocument;
use iota_interaction_ts::AdapterNativeResponse;
use tokio::sync::RwLock;
use tsify::Tsify;
use wasm_bindgen::prelude::*;

use iota_interaction_ts::bindings::WasmIotaObjectData;

use crate::error::wasm_error;
use crate::error::Result;
use crate::error::WasmResult;
use crate::iota::WasmIotaDocument;

use super::client_dummy::ProposalAction;
// use super::client_dummy::DummySigner;
// use super::client_dummy::ProposalAction;
// use super::client_dummy::ProposalBuilder;
use super::proposals::WasmCreateDeactivateDidProposalTx;
use super::types::WasmIotaAddress;
use super::WasmIdentityClient;

/// Helper type for `WasmIdentityBuilder::controllers`.
/// Has getters to support `Clone` for serialization
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
  pub fn did_document(&self) -> WasmIotaDocument {
    let inner_doc: IotaDocument = self.0.as_ref().clone().into();
    WasmIotaDocument::from(inner_doc)
  }

  #[wasm_bindgen(js_name = isShared)]
  pub fn is_shared(&self) -> Result<bool> {
    Ok(self.0.try_read().wasm_result()?.is_shared())
  }

  #[wasm_bindgen(skip_typescript)] // ts type in custom section below
  pub fn proposals(&self) -> Result<JsValue> {
    let lock = self.0.try_read().wasm_result()?;
    let proposals = lock.proposals();
    serde_wasm_bindgen::to_value(proposals).map_err(wasm_error)
  }

  #[wasm_bindgen(js_name = updateDidDocument)]
  pub async fn update_did_document(
    &self,
    updated_doc: WasmIotaDocument,
    identity_client: &WasmIdentityClient,
    expiration_epoch: Option<u64>,
    gas_budget: Option<u64>,
  ) -> Result<WasmTransactionOutputInternalOptionalProposalId> {
    let mut identity_lock = self.0.write().await;
    let builder = identity_lock.update_did_document(updated_doc.0.read().await.clone());
    let builder = if let Some(exp) = expiration_epoch {
      builder.expiration_epoch(exp)
    } else {
      builder
    };

    let tx_output = builder
      .finish(&identity_client.0)
      .await
      .wasm_result()?
      .execute_with_opt_gas_internal(gas_budget, &identity_client.0)
      .await
      .wasm_result()?;

    let maybe_proposal = match tx_output.output {
      ProposalResult::Pending(proposal) => Some(proposal.id().to_string()),
      ProposalResult::Executed(_) => None,
    };

    Ok(WasmTransactionOutputInternalOptionalProposalId {
      output: maybe_proposal,
      response: tx_output.response.clone_native_response(),
    })
  }

  #[wasm_bindgen(js_name = deactivateDid)]
  pub async fn deactivate_did(
    &self,
    identity_client: &WasmIdentityClient,
    expiration_epoch: Option<u64>,
  ) -> WasmCreateDeactivateDidProposalTx {
    WasmCreateDeactivateDidProposalTx::new(self, expiration_epoch)
  }

  #[wasm_bindgen(js_name = getHistory, skip_typescript)] // ts type in custom section below
  pub async fn get_history(
    &self,
    client: WasmIdentityClient,
    last_version: Option<WasmIotaObjectData>,
    page_size: Option<usize>,
  ) -> Result<JsValue> {
    unimplemented!("WasmOnChainIdentity::get_history");
    // let rs_history = self
    //   .0
    //   .get_history(
    //     &client.0,
    //     last_version.map(|lv| into_sdk_type(lv).unwrap()).as_ref(),
    //     page_size,
    //   )
    //   .await
    //   .map_err(wasm_error)?;
    // serde_wasm_bindgen::to_value(&rs_history).map_err(wasm_error)
  }
}

// Manually add the method to the interface.
#[wasm_bindgen(typescript_custom_section)]
const WASM_ON_CHAIN_IDENTITY_TYPES: &str = r###"
	export interface OnChainIdentity {
		proposals(): Map<String, Proposal>;
		getHistory(): Map<String, Proposal>;
	}
"###;

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(rename = "ProposalAction")]
pub enum WasmProposalAction {
  UpdateDocument(IotaDocument),
  Deactivate,
}

impl From<WasmProposalAction> for ProposalAction {
  fn from(val: WasmProposalAction) -> Self {
    match val {
      WasmProposalAction::UpdateDocument(doc) => ProposalAction::UpdateDocument(doc),
      WasmProposalAction::Deactivate => ProposalAction::Deactivate,
    }
  }
}

// TODO: remove the following comment and commented out code if we don't run into a rename issue
// -> in case `serde(rename` runs into issues with properties with renamed types still having the
// original type, see [here](https://github.com/madonoharu/tsify/issues/43) for an example
// #[declare]
// pub type ProposalAction = WasmProposalAction;

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

  #[wasm_bindgen]
  pub fn finish(self) -> WasmCreateIdentityTx {
    WasmCreateIdentityTx(self.0.finish())
  }
}

#[wasm_bindgen(js_name = CreateIdentityTx)]
pub struct WasmCreateIdentityTx(pub(crate) CreateIdentityTx);

#[wasm_bindgen(js_class = CreateIdentityTx)]
impl WasmCreateIdentityTx {
  #[wasm_bindgen]
  pub async fn execute(self, client: &WasmIdentityClient) -> Result<WasmTransactionOutputInternalOnChainIdentity> {
    let output = self.0.execute(&client.0).await.map_err(wasm_error)?;
    Ok(WasmTransactionOutputInternalOnChainIdentity(output))
  }
}

#[wasm_bindgen(js_name = TransactionOutputInternalOnChainIdentity)]
pub struct WasmTransactionOutputInternalOnChainIdentity(pub(crate) TransactionOutputInternal<OnChainIdentity>);

#[wasm_bindgen(js_class = TransactionOutputInternalOnChainIdentity)]
impl WasmTransactionOutputInternalOnChainIdentity {
  #[wasm_bindgen(getter)]
  pub fn output(&self) -> WasmOnChainIdentity {
    WasmOnChainIdentity(Rc::new(RwLock::new(self.0.output.clone())))
  }

  #[wasm_bindgen(getter)]
  pub fn response(&self) -> AdapterNativeResponse {
    self.0.response.clone_native_response()
  }
}

#[wasm_bindgen(js_name = TransactionOutputInternalOptionalProposalId, inspectable, getter_with_clone)]
pub struct WasmTransactionOutputInternalOptionalDeactivateDidProposal {
  pub output: Option<String>,
  pub response: AdapterNativeResponse,
}
