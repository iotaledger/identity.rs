// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::marker::PhantomData;

use identity_iota_interaction::rpc_types::IotaTransactionBlockEffects;
use identity_iota_interaction::IdentityMoveCalls;

use crate::iota_interaction_adapter::IdentityMoveCallsAdapter;
use crate::rebased::client::IdentityClientReadOnly;
use crate::rebased::migration::ControllerToken;
use crate::rebased::tx_refactor::TransactionBuilder;
use crate::IotaDocument;
use async_trait::async_trait;
use identity_iota_interaction::types::base_types::ObjectID;
use identity_iota_interaction::types::TypeTag;
use serde::Deserialize;
use serde::Serialize;

use crate::rebased::migration::OnChainIdentity;
use crate::rebased::migration::Proposal;
use crate::rebased::Error;
use identity_iota_interaction::MoveType;

use super::CreateProposal;
use super::ExecuteProposal;
use super::ProposalT;

/// Proposal's action for updating a DID Document.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(into = "UpdateValue::<Option<Vec<u8>>>", from = "UpdateValue::<Option<Vec<u8>>>")]
pub struct UpdateDidDocument(Option<Vec<u8>>);

impl MoveType for UpdateDidDocument {
  fn move_type(package: ObjectID) -> TypeTag {
    use std::str::FromStr;

    TypeTag::from_str(&format!(
      "{package}::update_value_proposal::UpdateValue<0x1::option::Option<vector<u8>>>"
    ))
    .expect("valid TypeTag")
  }
}

impl UpdateDidDocument {
  /// Creates a new [`UpdateDidDocument`] action.
  pub fn new(document: IotaDocument) -> Self {
    Self(Some(document.pack().expect("a valid IotaDocument is packable")))
  }

  /// Creates a new [`UpdateDidDocument`] action to deactivate the DID Document.
  pub fn deactivate() -> Self {
    Self(Some(vec![]))
  }

  /// Creates a new [`UpdateDidDocument`] action to delete the DID Document.
  pub fn delete() -> Self {
    Self(None)
  }

  /// Returns the serialized DID document bytes.
  pub fn did_document_bytes(&self) -> Option<&[u8]> {
    self.0.as_deref()
  }
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl ProposalT for Proposal<UpdateDidDocument> {
  type Action = UpdateDidDocument;
  type Output = ();

  async fn create<'i>(
    action: Self::Action,
    expiration: Option<u64>,
    identity: &'i mut OnChainIdentity,
    controller_token: &ControllerToken,
    client: &IdentityClientReadOnly,
  ) -> Result<TransactionBuilder<CreateProposal<'i, Self::Action>>, Error> {
    if identity.id() != controller_token.controller_of() {
      return Err(Error::Identity(format!(
        "token {} doesn't grant access to identity {}",
        controller_token.id(),
        identity.id()
      )));
    }

    let identity_ref = client
      .get_object_ref_by_id(identity.id())
      .await?
      .expect("identity exists on-chain");
    let controller_cap_ref = client
      .get_object_ref_by_id(controller_token.id())
      .await?
      .expect("token exists")
      .reference
      .to_object_ref();
    let sender_vp = identity
      .controller_voting_power(controller_cap_ref.0)
      .expect("controller exists");
    let chained_execution = sender_vp >= identity.threshold();
    let tx = IdentityMoveCallsAdapter::propose_update(
      identity_ref,
      controller_cap_ref,
      action.0.as_deref(),
      expiration,
      client.package_id(),
    )
    .await
    .map_err(|e| Error::TransactionBuildingFailed(e.to_string()))?;

    let ptb = bcs::from_bytes(&tx)?;

    Ok(TransactionBuilder::new(CreateProposal {
      identity,
      ptb,
      chained_execution,
      _action: PhantomData,
    }))
  }

  async fn into_tx<'i>(
    self,
    identity: &'i mut OnChainIdentity,
    controller_token: &ControllerToken,
    client: &IdentityClientReadOnly,
  ) -> Result<TransactionBuilder<ExecuteProposal<'i, Self::Action>>, Error> {
    if identity.id() != controller_token.controller_of() {
      return Err(Error::Identity(format!(
        "token {} doesn't grant access to identity {}",
        controller_token.id(),
        identity.id()
      )));
    }
    let proposal_id = self.id();
    let identity_ref = client
      .get_object_ref_by_id(identity.id())
      .await?
      .expect("identity exists on-chain");
    let controller_cap_ref = client
      .get_object_ref_by_id(controller_token.id())
      .await?
      .expect("token exists")
      .reference
      .to_object_ref();

    let tx =
      IdentityMoveCallsAdapter::execute_update(identity_ref, controller_cap_ref, proposal_id, client.package_id())
        .map_err(|e| Error::TransactionBuildingFailed(e.to_string()))?;

    let ptb = bcs::from_bytes(&tx)?;

    Ok(TransactionBuilder::new(ExecuteProposal {
      identity,
      ptb,
      _action: PhantomData,
    }))
  }

  fn parse_tx_effects(_tx_response: &IotaTransactionBlockEffects) -> Result<Self::Output, Error> {
    Ok(())
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UpdateValue<V> {
  new_value: V,
}

impl From<UpdateDidDocument> for UpdateValue<Option<Vec<u8>>> {
  fn from(value: UpdateDidDocument) -> Self {
    Self { new_value: value.0 }
  }
}

impl From<UpdateValue<Option<Vec<u8>>>> for UpdateDidDocument {
  fn from(value: UpdateValue<Option<Vec<u8>>>) -> Self {
    UpdateDidDocument(value.new_value)
  }
}
