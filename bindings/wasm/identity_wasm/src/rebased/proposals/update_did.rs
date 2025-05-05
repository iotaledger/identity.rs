// Copyright 2020-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::rc::Rc;

use identity_iota::iota::rebased::migration::Proposal;
use identity_iota::iota::rebased::proposals::ProposalResult;
use identity_iota::iota::rebased::proposals::ProposalT;
use identity_iota::iota::rebased::proposals::UpdateDidDocument;
use identity_iota::iota::rebased::transaction_builder::Transaction;
use identity_iota::iota::StateMetadataDocument;
use iota_interaction_ts::bindings::WasmIotaTransactionBlockEffects;
use js_sys::Object;
use tokio::sync::RwLock;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::prelude::JsCast;
use wasm_bindgen::JsValue;

use super::StringSet;
use crate::error::Result;
use crate::error::WasmResult;
use crate::iota::WasmIotaDocument;
use crate::rebased::WasmControllerToken;
use crate::rebased::WasmCoreClientReadOnly;
use crate::rebased::WasmIdentityClientReadOnly;
use crate::rebased::WasmManagedCoreClientReadOnly;
use crate::rebased::WasmOnChainIdentity;
use crate::rebased::WasmTransactionBuilder;

#[wasm_bindgen(js_name = UpdateDid)]
pub struct WasmUpdateDid(pub(crate) UpdateDidDocument);

#[wasm_bindgen(js_class = UpdateDid)]
impl WasmUpdateDid {
  #[wasm_bindgen(js_name = isDeactivation)]
  pub fn is_deactivation(&self) -> bool {
    matches!(self.0.did_document_bytes(), Some(&[]))
  }

  #[wasm_bindgen(getter, js_name = didDocument)]
  pub fn did_document(&self) -> Result<Option<WasmIotaDocument>> {
    self
      .0
      .did_document_bytes()
      .filter(|bytes| !bytes.is_empty())
      .map(|did_doc_bytes| {
        StateMetadataDocument::unpack(did_doc_bytes)
          .map(StateMetadataDocument::into_iota_document_with_placeholders)
          .map(WasmIotaDocument::from)
      })
      .transpose()
      .wasm_result()
  }
}

#[wasm_bindgen(js_name = UpdateDidProposal)]
#[derive(Clone)]
pub struct WasmProposalUpdateDid(pub(crate) Rc<RwLock<Proposal<UpdateDidDocument>>>);

#[wasm_bindgen(js_class = UpdatedDidProposal)]
impl WasmProposalUpdateDid {
  fn new(proposal: Proposal<UpdateDidDocument>) -> Self {
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
  pub fn action(&self) -> Result<WasmUpdateDid> {
    self
      .0
      .try_read()
      .wasm_result()
      .map(|proposal| proposal.action().clone())
      .map(WasmUpdateDid)
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

  #[wasm_bindgen(unchecked_return_type = "TransactionBuilder<ApproveProposal>")]
  pub fn approve(
    &self,
    identity: &WasmOnChainIdentity,
    controller_token: &WasmControllerToken,
    identity_client: &WasmIdentityClientReadOnly,
  ) -> Result<WasmTransactionBuilder> {
    let js_tx = JsValue::from(WasmApproveUpdateDidDocumentProposal::new(
      self,
      identity,
      controller_token,
      identity_client,
    ));
    Ok(WasmTransactionBuilder::new(js_tx.unchecked_into()))
  }

  #[wasm_bindgen(
    js_name = intoTx,
    unchecked_return_type = "TransactionBuilder<ExecuteProposal<UpdateDid>>"
  )]
  pub fn into_tx(
    self,
    identity: &WasmOnChainIdentity,
    controller_token: &WasmControllerToken,
    identity_client: &WasmIdentityClientReadOnly,
  ) -> WasmTransactionBuilder {
    let js_tx = JsValue::from(WasmExecuteUpdateDidDocumentProposal::new(
      self,
      identity,
      controller_token,
      identity_client,
    ));
    WasmTransactionBuilder::new(js_tx.unchecked_into())
  }
}

#[wasm_bindgen(js_name = ApproveUpdateDidDocumentProposal)]
pub struct WasmApproveUpdateDidDocumentProposal {
  proposal: WasmProposalUpdateDid,
  identity: WasmOnChainIdentity,
  controller_token: WasmControllerToken,
  identity_client: WasmIdentityClientReadOnly,
}

#[wasm_bindgen(js_class = ApproveUpdateDidDocumentProposal)]
impl WasmApproveUpdateDidDocumentProposal {
  fn new(
    proposal: &WasmProposalUpdateDid,
    identity: &WasmOnChainIdentity,
    controller_token: &WasmControllerToken,
    identity_client: &WasmIdentityClientReadOnly,
  ) -> Self {
    Self {
      proposal: proposal.clone(),
      identity: identity.clone(),
      controller_token: controller_token.clone(),
      identity_client: identity_client.clone(),
    }
  }

  #[wasm_bindgen(js_name = buildProgrammableTransaction)]
  pub async fn build_programmable_transaction(&self, client: &WasmCoreClientReadOnly) -> Result<Vec<u8>> {
    let managed_client = WasmManagedCoreClientReadOnly::from_wasm(client)?;
    let mut proposal = self.proposal.0.write().await;
    let identity = self.identity.0.read().await;
    let tx = proposal
      .approve(&identity, &self.controller_token.0, &self.identity_client.0)
      .wasm_result()?
      .into_inner();
    let pt = tx.build_programmable_transaction(&managed_client).await.wasm_result()?;
    bcs::to_bytes(&pt).wasm_result()
  }

  pub async fn apply(
    &self,
    wasm_effects: &WasmIotaTransactionBlockEffects,
    client: &WasmCoreClientReadOnly,
  ) -> Result<()> {
    let managed_client = WasmManagedCoreClientReadOnly::from_wasm(client)?;
    let mut proposal = self.proposal.0.write().await;
    let identity = self.identity.0.read().await;
    let tx = proposal
      .approve(&identity, &self.controller_token.0, &self.identity_client.0)
      .wasm_result()?
      .into_inner();
    let (apply_result, rem_effects) = tx.apply(wasm_effects.clone().into(), &managed_client).await;
    let wasm_rem_effects = WasmIotaTransactionBlockEffects::from(&rem_effects);
    Object::assign(wasm_effects, &wasm_rem_effects);

    apply_result.wasm_result()
  }
}

#[wasm_bindgen(js_name = ExecuteUpdateDidProposal)]
pub struct WasmExecuteUpdateDidDocumentProposal {
  proposal: WasmProposalUpdateDid,
  identity: WasmOnChainIdentity,
  controller_token: WasmControllerToken,
  identity_client: WasmIdentityClientReadOnly,
}

#[wasm_bindgen(js_class = ExecuteUpdateDidProposal)]
impl WasmExecuteUpdateDidDocumentProposal {
  pub fn new(
    proposal: WasmProposalUpdateDid,
    identity: &WasmOnChainIdentity,
    controller_token: &WasmControllerToken,
    identity_client: &WasmIdentityClientReadOnly,
  ) -> Self {
    Self {
      proposal,
      identity: identity.clone(),
      controller_token: controller_token.clone(),
      identity_client: identity_client.clone(),
    }
  }

  #[wasm_bindgen(js_name = buildProgrammableTransaction)]
  pub async fn build_programmable_transaction(&self, _client: &WasmCoreClientReadOnly) -> Result<Vec<u8>> {
    let proposal = self.proposal.0.read().await.clone();
    let mut identity = self.identity.0.write().await;
    let tx = proposal
      .into_tx(&mut identity, &self.controller_token.0, &self.identity_client.0)
      .await
      .wasm_result()?
      .into_inner();
    bcs::to_bytes(tx.ptb()).wasm_result()
  }

  pub async fn apply(
    self,
    wasm_effects: &WasmIotaTransactionBlockEffects,
    client: &WasmCoreClientReadOnly,
  ) -> Result<()> {
    let managed_client = WasmManagedCoreClientReadOnly::from_wasm(client)?;
    let proposal = self.proposal.0.read().await.clone();
    let mut identity = self.identity.0.write().await;
    let tx = proposal
      .into_tx(&mut identity, &self.controller_token.0, &self.identity_client.0)
      .await
      .wasm_result()?
      .into_inner();
    let (apply_result, rem_effects) = tx.apply(wasm_effects.clone().into(), &managed_client).await;
    let wasm_rem_effects = WasmIotaTransactionBlockEffects::from(&rem_effects);
    Object::assign(wasm_effects, &wasm_rem_effects);

    apply_result.wasm_result()
  }
}

#[wasm_bindgen(js_name = CreateUpdateDidProposal)]
pub struct WasmCreateUpdateDidProposal {
  identity: WasmOnChainIdentity,
  updated_did_doc: Option<WasmIotaDocument>,
  controller_token: WasmControllerToken,
  delete: bool,
  expiration_epoch: Option<u64>,
  identity_client: WasmIdentityClientReadOnly,
}

#[wasm_bindgen(js_class = CreateUpdateDidProposal)]
impl WasmCreateUpdateDidProposal {
  pub(crate) fn new(
    identity: &WasmOnChainIdentity,
    updated_did_doc: WasmIotaDocument,
    controller_token: WasmControllerToken,
    identity_client: &WasmIdentityClientReadOnly,
    expiration_epoch: Option<u64>,
  ) -> Self {
    Self {
      identity: identity.clone(),
      updated_did_doc: Some(updated_did_doc),
      delete: false,
      expiration_epoch,
      controller_token,
      identity_client: identity_client.clone(),
    }
  }

  pub(crate) fn deactivate(
    identity: &WasmOnChainIdentity,
    controller_token: WasmControllerToken,
    identity_client: &WasmIdentityClientReadOnly,
    expiration_epoch: Option<u64>,
  ) -> Self {
    Self {
      identity: identity.clone(),
      expiration_epoch,
      updated_did_doc: None,
      delete: false,
      controller_token,
      identity_client: identity_client.clone(),
    }
  }

  pub(crate) fn delete(
    identity: &WasmOnChainIdentity,
    controller_token: &WasmControllerToken,
    identity_client: &WasmIdentityClientReadOnly,
    expiration_epoch: Option<u64>,
  ) -> Self {
    Self {
      identity: identity.clone(),
      expiration_epoch,
      updated_did_doc: None,
      delete: true,
      controller_token: controller_token.clone(),
      identity_client: identity_client.clone(),
    }
  }

  #[wasm_bindgen(js_name = buildProgrammableTransaction)]
  pub async fn build_programmable_transaction(&self, _client: &WasmCoreClientReadOnly) -> Result<Vec<u8>> {
    let action = if let Some(did_doc) = self.updated_did_doc.as_ref() {
      let did_doc = did_doc.0.read().await.clone();
      UpdateDidDocument::new(did_doc)
    } else if self.delete {
      UpdateDidDocument::delete()
    } else {
      UpdateDidDocument::deactivate()
    };

    let mut identity_lock = self.identity.0.write().await;
    let tx = Proposal::<UpdateDidDocument>::create(
      action,
      self.expiration_epoch,
      &mut identity_lock,
      &self.controller_token.0,
      &self.identity_client.0,
    )
    .await
    .wasm_result()?
    .into_inner();

    bcs::to_bytes(tx.ptb()).wasm_result()
  }

  #[wasm_bindgen(unchecked_return_type = "ProposalResult<UpdateDid>")]
  pub async fn apply(
    self,
    wasm_effects: &WasmIotaTransactionBlockEffects,
    client: &WasmCoreClientReadOnly,
  ) -> Result<Option<WasmProposalUpdateDid>> {
    let managed_client = WasmManagedCoreClientReadOnly::from_wasm(client)?;
    let action = if let Some(did_doc) = self.updated_did_doc.as_ref() {
      let did_doc = did_doc.0.read().await.clone();
      UpdateDidDocument::new(did_doc)
    } else if self.delete {
      UpdateDidDocument::deactivate()
    } else {
      UpdateDidDocument::delete()
    };

    let mut identity_lock = self.identity.0.write().await;
    let tx = Proposal::<UpdateDidDocument>::create(
      action,
      self.expiration_epoch,
      &mut identity_lock,
      &self.controller_token.0,
      &self.identity_client.0,
    )
    .await
    .wasm_result()?
    .into_inner();

    let (apply_result, rem_effects) = tx.apply(wasm_effects.clone().into(), &managed_client).await;
    let wasm_rem_effects = WasmIotaTransactionBlockEffects::from(&rem_effects);
    Object::assign(wasm_effects, &wasm_rem_effects);

    let ProposalResult::Pending(proposal) = apply_result.wasm_result()? else {
      return Ok(None);
    };

    Ok(Some(WasmProposalUpdateDid::new(proposal)))
  }
}
