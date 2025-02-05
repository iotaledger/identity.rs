// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota::iota::rebased::migration::CreateIdentityTx;
use identity_iota::iota::rebased::migration::IdentityBuilder;
use identity_iota::iota::rebased::migration::OnChainIdentity;
use identity_iota::iota::rebased::transaction::TransactionInternal;
use identity_iota::iota::rebased::transaction::TransactionOutputInternal;
use identity_iota::iota::IotaDocument;
use iota_interaction_ts::AdapterNativeResponse;
use tsify::Tsify;
use wasm_bindgen::prelude::*;

use iota_interaction_ts::bindings::WasmIotaObjectData;

use iota_interaction_ts::iota_client_ts_sdk::IotaClientTsSdk;

use crate::error::wasm_error;
use crate::error::Result;
use crate::iota::WasmIotaDocument;

use super::client_dummy::DummySigner;
use super::client_dummy::ProposalAction;
use super::client_dummy::ProposalBuilder;
use super::types::into_sdk_type;
use super::types::WasmIotaAddress;
use super::types::WasmObjectID;
use super::WasmKinesisIdentityClient;
use super::WasmProposal;

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
pub struct WasmOnChainIdentity(pub(crate) OnChainIdentity);

#[wasm_bindgen(js_class = OnChainIdentity)]
impl WasmOnChainIdentity {
  #[wasm_bindgen(js_name = id)]
  pub fn id(&self) -> String {
    self.0.id().to_string()
  }

  #[wasm_bindgen(js_name = isShared)]
  pub fn is_shared(&self) -> bool {
    self.0.is_shared()
  }

  #[wasm_bindgen(skip_typescript)] // ts type in custom section below
  pub fn proposals(&self) -> Result<JsValue> {
    serde_wasm_bindgen::to_value(&self.0.proposals()).map_err(wasm_error)
  }

  #[wasm_bindgen(js_name = updateDidDocument)]
  pub fn update_did_document(self, updated_doc: WasmIotaDocument) -> Result<WasmProposalBuilder> {
    unimplemented!("WasmOnChainIdentity::update_did_document");
    // let doc: IotaDocument = updated_doc
    //   .0
    //   .try_read()
    //   .map_err(|err| JsError::new(&format!("failed to read DID document; {err:?}")))?
    //   .clone();
    // Ok(WasmProposalBuilder(self.0.update_did_document::<IotaClientTsSdk>(doc)))
  }

  #[wasm_bindgen(js_name = deactivateDid)]
  pub fn deactivate_did(self) -> WasmProposalBuilder {
    unimplemented!("WasmOnChainIdentity::deactivate_did");
    // WasmProposalBuilder(self.0.deactivate_did::<IotaClientTsSdk>())
  }

  #[wasm_bindgen(js_name = getHistory, skip_typescript)] // ts type in custom section below
  pub async fn get_history(
    &self,
    client: WasmKinesisIdentityClient,
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

#[wasm_bindgen(js_name = ProposalBuilder)]
pub struct WasmProposalBuilder(pub(crate) ProposalBuilder);

#[wasm_bindgen(js_class = ProposalBuilder)]
impl WasmProposalBuilder {
  // #[wasm_bindgen(constructor)]
  // pub fn new(identity: WasmOnChainIdentity, action: WasmProposalAction) -> Self {
  //   Self(ProposalBuilder::new(identity.0, action.into()))
  // }

  #[wasm_bindgen(js_name = expirationEpoch)]
  pub fn expiration_epoch(self, exp: u64) -> Self {
    Self(self.0.expiration_epoch(exp))
  }

  pub fn key(self, key: String) -> Self {
    Self(self.0.key(key))
  }

  #[wasm_bindgen(js_name = gasBudget)]
  pub fn gas_budget(self, amount: u64) -> Self {
    Self(self.0.gas_budget(amount))
  }

  pub async fn finish(self, client: &WasmKinesisIdentityClient, signer: &DummySigner) -> Result<Option<WasmProposal>> {
    unimplemented!("WasmProposalBuilder::finish");
    // self
    //   .0
    //   .finish(&client.0, signer)
    //   .await
    //   .map(|option| option.map(WasmProposal))
    //   .map_err(wasm_error)
  }
}

#[wasm_bindgen(js_name = IdentityBuilder)]
pub struct WasmIdentityBuilder(pub(crate) IdentityBuilder);

#[wasm_bindgen(js_class = IdentityBuilder)]
impl WasmIdentityBuilder {
  #[wasm_bindgen(constructor)]
  pub fn new(did_doc: &WasmIotaDocument) -> Result<WasmIdentityBuilder> {
    let document: IotaDocument = did_doc.0.try_read().unwrap().clone();
    Ok(
      WasmIdentityBuilder(
        IdentityBuilder::new(document)
      )
    )
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

  #[wasm_bindgen(js_name = finish)]
  pub fn finish(self) -> WasmCreateIdentityTx {
    WasmCreateIdentityTx(self.0.finish())
  }
}

#[wasm_bindgen(js_name = CreateIdentityTx)]
pub struct WasmCreateIdentityTx(pub(crate) CreateIdentityTx);

#[wasm_bindgen(js_class = CreateIdentityTx)]
impl WasmCreateIdentityTx {
  #[wasm_bindgen(js_name = execute)]
  pub async fn execute(self, client: &WasmKinesisIdentityClient) -> Result<WasmTransactionOutputInternalOnChainIdentity> {
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
    WasmOnChainIdentity(self.0.output.clone())
  }

  #[wasm_bindgen(getter)]
  pub fn response(&self) -> AdapterNativeResponse {
    self.0.response.clone_native_response()
  }
}