// Copyright 2020-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::rc::Rc;

use identity_iota::iota::rebased::migration::Proposal;
use identity_iota::iota::rebased::proposals::ProposalResult;
use identity_iota::iota::rebased::proposals::ProposalT;
use identity_iota::iota::rebased::proposals::SendAction;
use identity_iota::iota::rebased::transaction_builder::Transaction;
use iota_interaction_ts::bindings::WasmIotaTransactionBlockEffects;
use js_sys::Object;
use tokio::sync::RwLock;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::prelude::JsCast;
use wasm_bindgen::JsValue;

use super::StringCouple;
use super::StringSet;
use crate::error::Result;
use crate::error::WasmResult;
use crate::rebased::WasmControllerToken;
use crate::rebased::WasmIdentityClientReadOnly;
use crate::rebased::WasmOnChainIdentity;
use crate::rebased::WasmTransactionBuilder;

#[wasm_bindgen(js_name = SendAction)]
#[derive(Clone)]
pub struct WasmSendAction(pub(crate) SendAction);

#[wasm_bindgen(js_class = SendAction)]
impl WasmSendAction {
  #[wasm_bindgen(getter, js_name = objectRecipientMap)]
  pub fn object_recipient_map(&self) -> Vec<StringCouple> {
    self
      .0
      .as_ref()
      .iter()
      .map(|(obj, rec)| (obj.to_string(), rec.to_string()).into())
      .collect()
  }
}

#[wasm_bindgen(js_name = SendProposal)]
#[derive(Clone)]
pub struct WasmProposalSend(pub(crate) Rc<RwLock<Proposal<SendAction>>>);

#[wasm_bindgen(js_class = SendProposal)]
impl WasmProposalSend {
  fn new(proposal: Proposal<SendAction>) -> Self {
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
  pub fn action(&self) -> Result<WasmSendAction> {
    self
      .0
      .try_read()
      .wasm_result()
      .map(|proposal| WasmSendAction(proposal.action().clone()))
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
  ) -> WasmTransactionBuilder {
    let tx = WasmApproveSendProposal::new(self, identity, controller_token);
    WasmTransactionBuilder::new(JsValue::from(tx).unchecked_into())
  }

  #[wasm_bindgen(js_name = intoTx)]
  pub fn into_tx(
    self,
    identity: &WasmOnChainIdentity,
    controller_token: &WasmControllerToken,
  ) -> WasmTransactionBuilder {
    let tx = WasmExecuteSendProposal::new(self, identity, controller_token);
    WasmTransactionBuilder::new(JsValue::from(tx).unchecked_into())
  }
}

#[wasm_bindgen(js_name = ApproveSendProposal)]
pub struct WasmApproveSendProposal {
  proposal: WasmProposalSend,
  identity: WasmOnChainIdentity,
  controller_token: WasmControllerToken,
}

#[wasm_bindgen(js_class = ApproveSendProposal)]
impl WasmApproveSendProposal {
  fn new(proposal: &WasmProposalSend, identity: &WasmOnChainIdentity, controller_token: &WasmControllerToken) -> Self {
    Self {
      proposal: proposal.clone(),
      identity: identity.clone(),
      controller_token: controller_token.clone(),
    }
  }

  #[wasm_bindgen(js_name = buildProgrammableTransaction)]
  pub async fn build_programmable_transaction(self, client: &WasmIdentityClientReadOnly) -> Result<Vec<u8>> {
    let identity_ref = self.identity.0.read().await;
    let mut proposal_ref = self.proposal.0.write().await;
    let tx = proposal_ref
      .approve(&identity_ref, &self.controller_token.0)
      .wasm_result()?
      .into_inner();

    let pt = tx.build_programmable_transaction(&client.0).await.wasm_result()?;
    bcs::to_bytes(&pt).wasm_result()
  }

  #[wasm_bindgen]
  pub async fn apply(
    self,
    wasm_effects: &WasmIotaTransactionBlockEffects,
    client: &WasmIdentityClientReadOnly,
  ) -> Result<()> {
    let mut identity_ref = self.identity.0.write().await;
    let tx = self
      .proposal
      .0
      .write()
      .await
      .clone()
      .into_tx(&mut identity_ref, &self.controller_token.0, &client.0)
      .await
      .wasm_result()?
      .into_inner();
    let (apply_result, rem_effects) = tx.apply(wasm_effects.clone().into(), &client.0).await;
    let wasm_rem_effects = WasmIotaTransactionBlockEffects::from(&rem_effects);
    Object::assign(&wasm_effects, &wasm_rem_effects);

    apply_result.wasm_result()
  }
}

#[wasm_bindgen(js_name = ExecuteSendProposal)]
pub struct WasmExecuteSendProposal {
  proposal: WasmProposalSend,
  identity: WasmOnChainIdentity,
  controller_token: WasmControllerToken,
}

#[wasm_bindgen(js_class = ExecuteSendProposal)]
impl WasmExecuteSendProposal {
  fn new(proposal: WasmProposalSend, identity: &WasmOnChainIdentity, controller_token: &WasmControllerToken) -> Self {
    Self {
      proposal,
      identity: identity.clone(),
      controller_token: controller_token.clone(),
    }
  }

  #[wasm_bindgen(js_name = buildProgrammableTransaction)]
  pub async fn build_programmable_transaction(self, client: &WasmIdentityClientReadOnly) -> Result<Vec<u8>> {
    let mut identity_ref = self.identity.0.write().await;
    let proposal = self.proposal.0.read().await.clone();

    let pt = proposal
      .into_tx(&mut identity_ref, &self.controller_token.0, &client.0)
      .await
      .wasm_result()?
      .into_inner()
      .build_programmable_transaction(&client.0)
      .await
      .wasm_result()?;

    bcs::to_bytes(&pt).wasm_result()
  }

  #[wasm_bindgen]
  pub async fn apply(
    self,
    wasm_effects: &WasmIotaTransactionBlockEffects,
    client: &WasmIdentityClientReadOnly,
  ) -> Result<()> {
    let mut identity_ref = self.identity.0.write().await;
    let proposal = self.proposal.0.read().await.clone();

    let (apply_result, rem_effects) = proposal
      .into_tx(&mut identity_ref, &self.controller_token.0, &client.0)
      .await
      .wasm_result()?
      .into_inner()
      .apply(wasm_effects.clone().into(), &client.0)
      .await;

    let wasm_rem_effects = WasmIotaTransactionBlockEffects::from(&rem_effects);
    Object::assign(&wasm_effects, &wasm_rem_effects);

    apply_result.wasm_result()
  }
}

#[wasm_bindgen(js_name = CreateSendProposal)]
pub struct WasmCreateSendProposal {
  identity: WasmOnChainIdentity,
  action: SendAction,
  expiration_epoch: Option<u64>,
  controller_token: WasmControllerToken,
}

#[wasm_bindgen(js_class = CreateSendProposal)]
impl WasmCreateSendProposal {
  pub(crate) fn new(
    identity: &WasmOnChainIdentity,
    controller_token: &WasmControllerToken,
    object_recipient_map: Vec<StringCouple>,
    expiration_epoch: Option<u64>,
  ) -> Result<Self> {
    let action = object_recipient_map
      .into_iter()
      .map(Into::into)
      .map(|(obj_id_str, address_str)| {
        let obj_id = obj_id_str
          .parse()
          .map_err(|_| js_sys::TypeError::new("invalid object ID"))?;
        let address = address_str
          .parse()
          .map_err(|_| js_sys::TypeError::new("invalid IOTA address"))?;

        Result::Ok((obj_id, address))
      })
      .try_fold(SendAction::default(), |mut action, maybe_obj_address| {
        let (obj_id, address) = maybe_obj_address?;
        action.send_object(obj_id, address);

        Result::Ok(action)
      })?;
    Ok(Self {
      identity: identity.clone(),
      action,
      expiration_epoch,
      controller_token: controller_token.clone(),
    })
  }

  #[wasm_bindgen(js_name = buildProgrammableTransaction)]
  pub async fn build_programmable_transaction(&self, client: &WasmIdentityClientReadOnly) -> Result<Vec<u8>> {
    let mut identity_ref = self.identity.0.write().await;
    let tx = Proposal::<SendAction>::create(
      self.action.clone(),
      self.expiration_epoch,
      &mut identity_ref,
      &self.controller_token.0,
      &client.0,
    )
    .await
    .wasm_result()?
    .into_inner();
    let pt = tx.build_programmable_transaction(&client.0).await.wasm_result()?;

    bcs::to_bytes(&pt).wasm_result()
  }

  #[wasm_bindgen(unchecked_return_type = "ProposalResult<SendProposal>")]
  pub async fn apply(
    self,
    wasm_effects: &WasmIotaTransactionBlockEffects,
    client: &WasmIdentityClientReadOnly,
  ) -> Result<Option<WasmProposalSend>> {
    let mut identity_ref = self.identity.0.write().await;
    let tx = Proposal::<SendAction>::create(
      self.action.clone(),
      self.expiration_epoch,
      &mut identity_ref,
      &self.controller_token.0,
      &client.0,
    )
    .await
    .wasm_result()?
    .into_inner();

    let (apply_result, rem_effects) = tx.apply(wasm_effects.clone().into(), &client.0).await;
    let wasm_rem_effects = WasmIotaTransactionBlockEffects::from(&rem_effects);
    Object::assign(&wasm_effects, &wasm_rem_effects);

    match apply_result.wasm_result()? {
      ProposalResult::Pending(proposal) => Ok(Some(WasmProposalSend::new(proposal))),
      ProposalResult::Executed(_) => Ok(None),
    }
  }
}
