// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::marker::PhantomData;

use crate::iota_interaction_adapter::AdapterError;
use crate::iota_interaction_adapter::IdentityMoveCallsAdapter;
use crate::iota_interaction_adapter::IotaTransactionBlockResponseAdapter;
use crate::iota_interaction_adapter::NativeTransactionBlockResponse;
use identity_iota_interaction::IdentityMoveCalls;
use identity_iota_interaction::IotaKeySignature;
use identity_iota_interaction::IotaTransactionBlockResponseT;
use identity_iota_interaction::OptionalSync;

use crate::rebased::client::IdentityClient;
use async_trait::async_trait;
use identity_iota_interaction::types::base_types::ObjectID;
use identity_iota_interaction::types::TypeTag;
use secret_storage::Signer;
use serde::Deserialize;
use serde::Serialize;

use crate::rebased::migration::OnChainIdentity;
use crate::rebased::migration::Proposal;
use crate::rebased::Error;
use identity_iota_interaction::MoveType;

use super::CreateProposalTx;
use super::ExecuteProposalTx;
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

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl ProposalT for Proposal<Upgrade> {
  type Action = Upgrade;
  type Output = ();
  type Response = IotaTransactionBlockResponseAdapter;

  async fn create<'i, S>(
    _action: Self::Action,
    expiration: Option<u64>,
    identity: &'i mut OnChainIdentity,
    client: &IdentityClient<S>,
  ) -> Result<CreateProposalTx<'i, Self::Action>, Error>
  where
    S: Signer<IotaKeySignature> + OptionalSync,
  {
    let identity_ref = client
      .get_object_ref_by_id(identity.id())
      .await?
      .expect("identity exists on-chain");
    let controller_cap_ref = identity.get_controller_cap(client).await?;
    let sender_vp = identity
      .controller_voting_power(controller_cap_ref.0)
      .expect("controller exists");
    let chained_execution = sender_vp >= identity.threshold();
    let tx =
      IdentityMoveCallsAdapter::propose_upgrade(identity_ref, controller_cap_ref, expiration, client.package_id())
        .map_err(|e| Error::TransactionBuildingFailed(e.to_string()))?;

    Ok(CreateProposalTx {
      identity,
      tx,
      chained_execution,
      _action: PhantomData,
    })
  }

  async fn into_tx<'i, S>(
    self,
    identity: &'i mut OnChainIdentity,
    client: &IdentityClient<S>,
  ) -> Result<ExecuteProposalTx<'i, Self::Action>, Error>
  where
    S: Signer<IotaKeySignature> + OptionalSync,
  {
    let proposal_id = self.id();
    let identity_ref = client
      .get_object_ref_by_id(identity.id())
      .await?
      .expect("identity exists on-chain");
    let controller_cap_ref = identity.get_controller_cap(client).await?;

    let tx =
      IdentityMoveCallsAdapter::execute_upgrade(identity_ref, controller_cap_ref, proposal_id, client.package_id())
        .map_err(|e| Error::TransactionBuildingFailed(e.to_string()))?;

    Ok(ExecuteProposalTx {
      identity,
      tx,
      _action: PhantomData,
    })
  }

  fn parse_tx_effects_internal(
    _tx_response: &dyn IotaTransactionBlockResponseT<
      Error = AdapterError,
      NativeResponse = NativeTransactionBlockResponse,
    >,
  ) -> Result<Self::Output, Error> {
    Ok(())
  }
}
