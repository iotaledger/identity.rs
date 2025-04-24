// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::marker::PhantomData;

use identity_iota_interaction::rpc_types::IotaTransactionBlockEffects;
use identity_iota_interaction::IdentityMoveCalls;

use crate::iota_interaction_adapter::IdentityMoveCallsAdapter;
use crate::rebased::client::IdentityClientReadOnly;
use crate::rebased::migration::ControllerToken;
use crate::rebased::transaction_builder::TransactionBuilder;
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

/// Action for upgrading the version of an on-chain identity to the package's version.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct Upgrade;

impl Upgrade {
  /// Creates a new [`Upgrade`] action.
  pub const fn new() -> Self {
    Self
  }
}

impl MoveType for Upgrade {
  fn move_type(package: ObjectID) -> TypeTag {
    format!("{package}::upgrade_proposal::Upgrade")
      .parse()
      .expect("valid utf8")
  }
}

#[cfg_attr(not(feature = "send-sync"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync", async_trait)]
impl ProposalT for Proposal<Upgrade> {
  type Action = Upgrade;
  type Output = ();

  async fn create<'i>(
    _action: Self::Action,
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
    let controller_cap_ref = controller_token.controller_ref(client).await?;
    let sender_vp = identity
      .controller_voting_power(controller_token.controller_id())
      .expect("controller exists");
    let chained_execution = sender_vp >= identity.threshold();
    let tx =
      IdentityMoveCallsAdapter::propose_upgrade(identity_ref, controller_cap_ref, expiration, client.package_id())
        .map_err(|e| Error::TransactionBuildingFailed(e.to_string()))?;

    Ok(TransactionBuilder::new(CreateProposal {
      identity,
      ptb: bcs::from_bytes(&tx)?,
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
    let controller_cap_ref = controller_token.controller_ref(client).await?;

    let tx =
      IdentityMoveCallsAdapter::execute_upgrade(identity_ref, controller_cap_ref, proposal_id, client.package_id())
        .map_err(|e| Error::TransactionBuildingFailed(e.to_string()))?;

    Ok(TransactionBuilder::new(ExecuteProposal {
      identity,
      ptb: bcs::from_bytes(&tx)?,
      _action: PhantomData,
    }))
  }

  fn parse_tx_effects(_effects: &IotaTransactionBlockEffects) -> Result<Self::Output, Error> {
    Ok(())
  }
}
