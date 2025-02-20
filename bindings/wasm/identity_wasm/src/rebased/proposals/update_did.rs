// Copyright 2020-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::rc::Rc;

use identity_iota::iota::rebased::migration::Proposal;
use identity_iota::iota::rebased::proposals::ProposalResult;
use identity_iota::iota::rebased::proposals::ProposalT;
use identity_iota::iota::rebased::proposals::UpdateDidDocument;
use identity_iota::iota::rebased::transaction::TransactionInternal;
use identity_iota::iota::rebased::transaction::TransactionOutputInternal;
use identity_iota::iota::StateMetadataDocument;
use iota_interaction_ts::AdapterNativeResponse;
use tokio::sync::RwLock;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::prelude::JsCast;

use super::StringSet;
use crate::error::Result;
use crate::error::WasmResult;
use crate::iota::WasmIotaDocument;
use crate::rebased::WasmIdentityClient;
use crate::rebased::WasmOnChainIdentity;

#[wasm_bindgen(js_name = UpdateDid)]
pub struct WasmUpdateDid(pub(crate) WasmIotaDocument);

#[wasm_bindgen(js_class = UpdateDid)]
impl WasmUpdateDid {
  #[wasm_bindgen(getter, js_name = didDocument)]
  pub fn did_document(&self) -> Result<WasmIotaDocument> {
    self.0.deep_clone()
  }
}

impl Clone for WasmUpdateDid {
  fn clone(&self) -> Self {
    Self(self.0.shallow_clone())
  }
}

#[wasm_bindgen(js_name = UpdateDidProposal)]
#[derive(Clone)]
pub struct WasmProposalUpdateDid {
  inner_proposal: Rc<RwLock<Proposal<UpdateDidDocument>>>,
  action: WasmUpdateDid,
}

#[wasm_bindgen(js_class = UpdatedDidProposal)]
impl WasmProposalUpdateDid {
  fn new(proposal: Proposal<UpdateDidDocument>) -> Self {
    let updated_iota_document = StateMetadataDocument::unpack(proposal.action().did_document_bytes())
      .expect("valid encoded IOTA DID document")
      .into_iota_document_with_placeholders();
    let action = WasmUpdateDid(updated_iota_document.into());

    Self {
      inner_proposal: Rc::new(RwLock::new(proposal)),
      action,
    }
  }

  #[wasm_bindgen(getter)]
  pub fn id(&self) -> Result<String> {
    self
      .inner_proposal
      .try_read()
      .wasm_result()
      .map(|proposal| proposal.id().to_string())
  }

  #[wasm_bindgen(getter)]
  pub fn action(&self) -> WasmUpdateDid {
    self.action.clone()
  }

  #[wasm_bindgen(getter)]
  pub fn expiration_epoch(&self) -> Result<Option<u64>> {
    self
      .inner_proposal
      .try_read()
      .wasm_result()
      .map(|proposal| proposal.expiration_epoch())
  }

  #[wasm_bindgen(getter)]
  pub fn votes(&self) -> Result<u64> {
    self
      .inner_proposal
      .try_read()
      .wasm_result()
      .map(|proposal| proposal.votes())
  }

  #[wasm_bindgen(getter)]
  pub fn voters(&self) -> Result<StringSet> {
    let js_set = self
      .inner_proposal
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
  pub fn approve(&self, identity: &WasmOnChainIdentity) -> WasmApproveUpdateDidDocumentProposalTx {
    WasmApproveUpdateDidDocumentProposalTx::new(self, identity)
  }

  #[wasm_bindgen(js_name = intoTx)]
  pub fn into_tx(self, identity: &WasmOnChainIdentity) -> WasmExecuteUpdateDidDocumentProposalTx {
    WasmExecuteUpdateDidDocumentProposalTx::new(self, identity)
  }
}

#[wasm_bindgen(js_name = ApproveUpdateDidDocumentProposalTx)]
pub struct WasmApproveUpdateDidDocumentProposalTx {
  proposal: WasmProposalUpdateDid,
  identity: WasmOnChainIdentity,
  gas_budget: Option<u64>,
}

#[wasm_bindgen(js_class = ApproveUpdateDidDocumentProposalTx)]
impl WasmApproveUpdateDidDocumentProposalTx {
  fn new(proposal: &WasmProposalUpdateDid, identity: &WasmOnChainIdentity) -> Self {
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
      .inner_proposal
      .write()
      .await
      .approve(&identity_ref)
      .execute_with_opt_gas_internal(self.gas_budget, &client.0)
      .await
      .wasm_result()
      .map(|tx_output| tx_output.response.clone_native_response())
  }
}

#[wasm_bindgen(js_name = ExecuteUpdateDidProposalTx)]
pub struct WasmExecuteUpdateDidDocumentProposalTx {
  proposal: WasmProposalUpdateDid,
  identity: WasmOnChainIdentity,
  gas_budget: Option<u64>,
}

#[wasm_bindgen(js_class = ExecuteUpdateDidProposalTx)]
impl WasmExecuteUpdateDidDocumentProposalTx {
  fn new(proposal: WasmProposalUpdateDid, identity: &WasmOnChainIdentity) -> Self {
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
    let proposal = Rc::into_inner(self.proposal.inner_proposal)
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

#[wasm_bindgen(js_name = CreateUpdateDidProposalTxOutput, inspectable, getter_with_clone)]
pub struct WasmCreateUpdateDidProposalTxOutput {
  pub output: Option<WasmProposalUpdateDid>,
  pub response: AdapterNativeResponse,
}

impl From<TransactionOutputInternal<ProposalResult<Proposal<UpdateDidDocument>>>>
  for WasmCreateUpdateDidProposalTxOutput
{
  fn from(tx_output: TransactionOutputInternal<ProposalResult<Proposal<UpdateDidDocument>>>) -> Self {
    let output = match tx_output.output {
      ProposalResult::Pending(proposal) => Some(WasmProposalUpdateDid::new(proposal)),
      ProposalResult::Executed(_) => None,
    };
    let response = tx_output.response.clone_native_response();
    Self { output, response }
  }
}

#[wasm_bindgen(js_name = CreateUpdateDidProposalTx)]
pub struct WasmCreateUpdateDidProposalTx {
  identity: WasmOnChainIdentity,
  updated_did_doc: WasmIotaDocument,
  expiration_epoch: Option<u64>,
  gas_budget: Option<u64>,
}

#[wasm_bindgen(js_class = CreateUpdateDidProposalTx)]
impl WasmCreateUpdateDidProposalTx {
  pub(crate) fn new(
    identity: &WasmOnChainIdentity,
    updated_did_doc: WasmIotaDocument,
    expiration_epoch: Option<u64>,
  ) -> Self {
    Self {
      identity: identity.clone(),
      updated_did_doc,
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
  pub async fn execute(self, client: &WasmIdentityClient) -> Result<WasmCreateUpdateDidProposalTxOutput> {
    let updated_did_doc = self.updated_did_doc.0.read().await.clone();
    let mut identity_ref = self.identity.0.write().await;
    let builder = identity_ref.update_did_document(updated_did_doc);
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
