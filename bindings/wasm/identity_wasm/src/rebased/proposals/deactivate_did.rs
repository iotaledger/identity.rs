// Copyright 2020-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::rc::Rc;

use identity_iota::iota::rebased::migration::Proposal;
use identity_iota::iota::rebased::proposals::DeactivateDid;
use identity_iota::iota::rebased::proposals::ProposalResult;
use identity_iota::iota::rebased::proposals::ProposalT;
use identity_iota::iota::rebased::transaction::TransactionInternal;
use identity_iota::iota::rebased::transaction::TransactionOutputInternal;
use iota_interaction_ts::NativeTransactionBlockResponse;
use tokio::sync::RwLock;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::prelude::JsCast;

use super::StringSet;
use crate::error::Result;
use crate::error::WasmResult;
use crate::rebased::WasmIdentityClient;
use crate::rebased::WasmOnChainIdentity;

#[wasm_bindgen(js_name = DeactivateDid)]
#[derive(Clone, Copy, Default)]
pub struct WasmDeactivateDid;

#[wasm_bindgen(js_name = DeactivateDidProposal)]
#[derive(Clone)]
pub struct WasmProposalDeactivateDid(pub(crate) Rc<RwLock<Proposal<DeactivateDid>>>);

#[wasm_bindgen(js_class = DeactivateDidProposal)]
impl WasmProposalDeactivateDid {
  fn new(proposal: Proposal<DeactivateDid>) -> Self {
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
  pub fn action(&self) -> WasmDeactivateDid {
    WasmDeactivateDid
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
  pub fn approve(&self, identity: &WasmOnChainIdentity) -> WasmApproveDeativateDidProposalTx {
    WasmApproveDeativateDidProposalTx::new(self, identity)
  }

  #[wasm_bindgen(js_name = intoTx)]
  pub fn into_tx(self, identity: &WasmOnChainIdentity) -> WasmExecuteDeactivateDidProposalTx {
    WasmExecuteDeactivateDidProposalTx::new(self, identity)
  }
}

#[wasm_bindgen(js_name = ApproveDeativateDidProposalTx)]
pub struct WasmApproveDeativateDidProposalTx {
  proposal: WasmProposalDeactivateDid,
  identity: WasmOnChainIdentity,
  gas_budget: Option<u64>,
}

#[wasm_bindgen(js_class = ApproveDeativateDidProposalTx)]
impl WasmApproveDeativateDidProposalTx {
  fn new(proposal: &WasmProposalDeactivateDid, identity: &WasmOnChainIdentity) -> Self {
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
  pub async fn execute(self, client: &WasmIdentityClient) -> Result<NativeTransactionBlockResponse> {
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

#[wasm_bindgen(js_name = ExecuteDeactivateDidProposalTx)]
pub struct WasmExecuteDeactivateDidProposalTx {
  proposal: WasmProposalDeactivateDid,
  identity: WasmOnChainIdentity,
  gas_budget: Option<u64>,
}

#[wasm_bindgen(js_class = ExecuteDeactivateDidProposalTx)]
impl WasmExecuteDeactivateDidProposalTx {
  fn new(proposal: WasmProposalDeactivateDid, identity: &WasmOnChainIdentity) -> Self {
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
  pub async fn execute(self, client: &WasmIdentityClient) -> Result<NativeTransactionBlockResponse> {
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

#[wasm_bindgen(js_name = CreateDeactivateDidProposalTxOutput, inspectable, getter_with_clone)]
pub struct WasmCreateDeactivateDidProposalTxOutput {
  pub output: Option<WasmProposalDeactivateDid>,
  pub response: NativeTransactionBlockResponse,
}

impl From<TransactionOutputInternal<ProposalResult<Proposal<DeactivateDid>>>>
  for WasmCreateDeactivateDidProposalTxOutput
{
  fn from(tx_output: TransactionOutputInternal<ProposalResult<Proposal<DeactivateDid>>>) -> Self {
    let output = match tx_output.output {
      ProposalResult::Pending(proposal) => Some(WasmProposalDeactivateDid::new(proposal)),
      ProposalResult::Executed(_) => None,
    };
    let response = tx_output.response.clone_native_response();
    Self { output, response }
  }
}

#[wasm_bindgen(js_name = CreateDeactivateDidProposalTx)]
pub struct WasmCreateDeactivateDidProposalTx {
  identity: WasmOnChainIdentity,
  expiration_epoch: Option<u64>,
  gas_budget: Option<u64>,
}

#[wasm_bindgen(js_class = CreateDeactivateDidProposalTx)]
impl WasmCreateDeactivateDidProposalTx {
  pub(crate) fn new(identity: &WasmOnChainIdentity, expiration_epoch: Option<u64>) -> Self {
    Self {
      identity: identity.clone(),
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
  pub async fn execute(self, client: &WasmIdentityClient) -> Result<WasmCreateDeactivateDidProposalTxOutput> {
    let mut identity_ref = self.identity.0.write().await;
    let builder = identity_ref.deactivate_did();
    let builder = if let Some(exp) = self.expiration_epoch {
      builder.expiration_epoch(exp)
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
