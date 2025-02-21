// Copyright 2020-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;

use identity_iota::iota::rebased::migration::Proposal;
use identity_iota::iota::rebased::proposals::ConfigChange;
use identity_iota::iota::rebased::proposals::ProposalResult;
use identity_iota::iota::rebased::proposals::ProposalT;
use identity_iota::iota::rebased::transaction::TransactionInternal;
use identity_iota::iota::rebased::transaction::TransactionOutputInternal;
use iota_interaction_ts::AdapterNativeResponse;
use tokio::sync::RwLock;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::prelude::JsCast;
use wasm_bindgen::JsValue;

use super::MapStringNumber;
use super::StringSet;
use crate::error::Result;
use crate::error::WasmResult;
use crate::rebased::WasmIdentityClient;
use crate::rebased::WasmOnChainIdentity;

#[wasm_bindgen(js_name = ConfigChange, inspectable, getter_with_clone)]
pub struct WasmConfigChange {
  pub threshold: Option<u64>,
  #[wasm_bindgen(js_name = controllersToAdd)]
  pub controllers_to_add: Option<MapStringNumber>,
  #[wasm_bindgen(js_name = controllersToRemove)]
  pub controllers_to_remove: Option<StringSet>,
  #[wasm_bindgen(js_name = controllersToUpdate)]
  pub controllers_to_update: Option<MapStringNumber>,
}

impl TryFrom<ConfigChange> for WasmConfigChange {
  type Error = JsValue;
  fn try_from(value: ConfigChange) -> std::result::Result<Self, Self::Error> {
    let threshold = value.threshold();
    let controllers_to_add = if value.controllers_to_add().is_empty() {
      None
    } else {
      Some(value.controllers_to_add().try_into()?)
    };
    let controllers_to_remove = if value.controllers_to_remove().is_empty() {
      None
    } else {
      Some(value.controllers_to_remove().try_into()?)
    };
    let controllers_to_update = if value.controllers_to_update().is_empty() {
      None
    } else {
      Some(value.controllers_to_update().try_into()?)
    };

    Ok(Self {
      threshold,
      controllers_to_add,
      controllers_to_remove,
      controllers_to_update,
    })
  }
}

#[wasm_bindgen(js_name = ConfigChangeProposal)]
#[derive(Clone)]
pub struct WasmConfigChangeProposal(pub(crate) Rc<RwLock<Proposal<ConfigChange>>>);

#[wasm_bindgen(js_class = ConfigChangeProposal)]
impl WasmConfigChangeProposal {
  fn new(proposal: Proposal<ConfigChange>) -> Self {
    Self(Rc::new(RwLock::new(proposal)))
  }

  #[wasm_bindgen(getter)]
  pub fn id(&self) -> Result<String> {
    self
      .0
      .try_read()
      .wasm_result()
      .map(|proposal| proposal.id().to_string())
  }

  #[wasm_bindgen(getter)]
  pub fn action(&self) -> Result<WasmConfigChange> {
    self
      .0
      .try_read()
      .wasm_result()
      .and_then(|proposal| proposal.action().clone().try_into())
  }

  #[wasm_bindgen(getter)]
  pub fn expiration_epoch(&self) -> Result<Option<u64>> {
    self
      .0
      .try_read()
      .wasm_result()
      .map(|proposal| proposal.expiration_epoch())
  }

  #[wasm_bindgen(getter)]
  pub fn votes(&self) -> Result<u64> {
    self.0.try_read().wasm_result().map(|proposal| proposal.votes())
  }

  #[wasm_bindgen(getter)]
  pub fn voters(&self) -> Result<StringSet> {
    let js_set = self
      .0
      .try_read()
      .wasm_result()?
      .voters()
      .iter()
      .map(ToString::to_string)
      .map(js_sys::JsString::from)
      .fold(js_sys::Set::default(), |set, value| {
        set.add(&value);
        set
      })
      .unchecked_into();

    Ok(js_set)
  }

  #[wasm_bindgen]
  pub fn approve(&self, identity: &WasmOnChainIdentity) -> WasmApproveConfigChangeProposalTx {
    WasmApproveConfigChangeProposalTx::new(self, identity)
  }

  #[wasm_bindgen(js_name = intoTx)]
  pub fn into_tx(self, identity: &WasmOnChainIdentity) -> WasmExecuteConfigChangeProposalTx {
    WasmExecuteConfigChangeProposalTx::new(self, identity)
  }
}

#[wasm_bindgen(js_name = ApproveConfigChangeProposalTx)]
pub struct WasmApproveConfigChangeProposalTx {
  proposal: WasmConfigChangeProposal,
  identity: WasmOnChainIdentity,
  gas_budget: Option<u64>,
}

#[wasm_bindgen(js_class = ApproveConfigChangeProposalTx)]
impl WasmApproveConfigChangeProposalTx {
  fn new(proposal: &WasmConfigChangeProposal, identity: &WasmOnChainIdentity) -> Self {
    Self {
      proposal: proposal.clone(),
      identity: identity.clone(),
      gas_budget: None,
    }
  }

  #[wasm_bindgen(js_name = withGasBudget)]
  pub fn with_gas_budget(mut self, budget: u64) -> Self {
    self.gas_budget = Some(budget);
    self
  }

  #[wasm_bindgen(setter, js_name = gasBudget)]
  pub fn set_gas_budget(&mut self, budget: u64) {
    self.gas_budget = Some(budget);
  }

  #[wasm_bindgen]
  pub async fn execute(self, client: &WasmIdentityClient) -> Result<AdapterNativeResponse> {
    let identity_ref = self.identity.0.read().await;
    self
      .proposal
      .0
      .write()
      .await
      .approve(&identity_ref)
      .execute_with_opt_gas_internal(self.gas_budget, &client.0)
      .await
      .wasm_result()
      .map(|tx_output| tx_output.response.clone_native_response())
  }
}

#[wasm_bindgen(js_name = ExecuteConfigChangeProposalTx)]
pub struct WasmExecuteConfigChangeProposalTx {
  proposal: WasmConfigChangeProposal,
  identity: WasmOnChainIdentity,
  gas_budget: Option<u64>,
}

#[wasm_bindgen(js_class = ExecuteConfigChangeProposalTx)]
impl WasmExecuteConfigChangeProposalTx {
  fn new(proposal: WasmConfigChangeProposal, identity: &WasmOnChainIdentity) -> Self {
    Self {
      proposal,
      identity: identity.clone(),
      gas_budget: None,
    }
  }

  #[wasm_bindgen(js_name = withGasBudget)]
  pub fn with_gas_budget(mut self, budget: u64) -> Self {
    self.gas_budget = Some(budget);
    self
  }

  #[wasm_bindgen(setter, js_name = gasBudget)]
  pub fn set_gas_budget(&mut self, budget: u64) {
    self.gas_budget = Some(budget);
  }

  #[wasm_bindgen]
  pub async fn execute(self, client: &WasmIdentityClient) -> Result<AdapterNativeResponse> {
    let mut identity_ref = self.identity.0.write().await;
    let proposal = Rc::into_inner(self.proposal.0)
      .ok_or_else(|| js_sys::Error::new("cannot consume proposal; try to drop all other references to it"))?
      .into_inner();

    proposal
      .into_tx(&mut identity_ref, client)
      .await
      .wasm_result()?
      .execute_with_opt_gas_internal(self.gas_budget, client)
      .await
      .wasm_result()
      .map(|tx_output| tx_output.response.clone_native_response())
  }
}

#[wasm_bindgen(js_name = CreateConfigChangeProposalTxOutput, inspectable, getter_with_clone)]
pub struct WasmCreateConfigChangeProposalTxOutput {
  pub output: Option<WasmConfigChangeProposal>,
  pub response: AdapterNativeResponse,
}

impl From<TransactionOutputInternal<ProposalResult<Proposal<ConfigChange>>>>
  for WasmCreateConfigChangeProposalTxOutput
{
  fn from(tx_output: TransactionOutputInternal<ProposalResult<Proposal<ConfigChange>>>) -> Self {
    let output = match tx_output.output {
      ProposalResult::Pending(proposal) => Some(WasmConfigChangeProposal::new(proposal)),
      ProposalResult::Executed(_) => None,
    };
    let response = tx_output.response.clone_native_response();
    Self { output, response }
  }
}

#[wasm_bindgen(js_name = CreateConfigChangeProposalTx)]
pub struct WasmCreateConfigChangeProposalTx {
  identity: WasmOnChainIdentity,
  threshold: Option<u64>,
  controllers_to_add: Option<MapStringNumber>,
  controllers_to_remove: Option<StringSet>,
  controllers_to_update: Option<MapStringNumber>,
  expiration_epoch: Option<u64>,
  gas_budget: Option<u64>,
}

#[wasm_bindgen(js_class = CreateConfigChangeProposalTx)]
impl WasmCreateConfigChangeProposalTx {
  pub(crate) fn new(identity: &WasmOnChainIdentity, config: WasmConfigChange, expiration_epoch: Option<u64>) -> Self {
    let WasmConfigChange {
      controllers_to_add,
      controllers_to_remove,
      controllers_to_update,
      threshold,
    } = config;
    Self {
      identity: identity.clone(),
      threshold,
      controllers_to_add,
      controllers_to_remove,
      controllers_to_update,
      expiration_epoch,
      gas_budget: None,
    }
  }

  #[wasm_bindgen(js_name = withGasBudget)]
  pub fn with_gas_budget(mut self, budget: u64) -> Self {
    self.gas_budget = Some(budget);
    self
  }

  #[wasm_bindgen(setter, js_name = gasBudget)]
  pub fn set_gas_budget(&mut self, budget: u64) {
    self.gas_budget = Some(budget);
  }

  #[wasm_bindgen]
  pub async fn execute(self, client: &WasmIdentityClient) -> Result<WasmCreateConfigChangeProposalTxOutput> {
    let mut identity_ref = self.identity.0.write().await;
    let controllers_to_add = self
      .controllers_to_add
      .map(HashMap::try_from)
      .transpose()?
      .unwrap_or_default();
    let controllers_to_remove = self
      .controllers_to_remove
      .map(HashSet::try_from)
      .transpose()?
      .unwrap_or_default();
    let controllers_to_update = self
      .controllers_to_update
      .map(HashMap::try_from)
      .transpose()?
      .unwrap_or_default();
    let builder = identity_ref
      .update_config()
      .add_multiple_controllers(controllers_to_add)
      .remove_multiple_controllers(controllers_to_remove)
      .update_multiple_controllers(controllers_to_update);
    let builder = if let Some(exp) = self.expiration_epoch {
      builder.expiration_epoch(exp)
    } else {
      builder
    };
    let builder = if let Some(tr) = self.threshold {
      builder.threshold(tr)
    } else {
      builder
    };

    let tx_output = builder
      .finish(&client)
      .await
      .wasm_result()?
      .execute_with_opt_gas_internal(self.gas_budget, &client)
      .await
      .wasm_result()?;

    Ok(tx_output.into())
  }
}
