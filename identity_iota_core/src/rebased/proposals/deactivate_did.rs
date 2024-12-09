// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::marker::PhantomData;

use crate::rebased::client::IdentityClient;
use crate::rebased::client::IotaKeySignature;
use crate::rebased::iota::move_calls;
use async_trait::async_trait;
use identity_iota_interaction::rpc_types::IotaTransactionBlockResponse;
use identity_iota_interaction::types::base_types::ObjectID;
use identity_iota_interaction::types::TypeTag;
use secret_storage::Signer;
use serde::Deserialize;
use serde::Serialize;

use crate::rebased::migration::OnChainIdentity;
use crate::rebased::migration::Proposal;
use identity_iota_interaction::MoveType;
use crate::rebased::Error;

use super::CreateProposalTx;
use super::ExecuteProposalTx;
use super::ProposalT;

/// Action for deactivating a DID Document inside an Identity.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct DeactivateDid;

impl DeactivateDid {
  /// Creates a new [`DeactivateDid`] action.
  pub const fn new() -> Self {
    Self
  }
}

impl MoveType for DeactivateDid {
  fn move_type(package: ObjectID) -> TypeTag {
    use std::str::FromStr;

    TypeTag::from_str(&format!("{package}::identity::DeactivateDid")).expect("valid utf8")
  }
}

#[async_trait]
impl ProposalT for Proposal<DeactivateDid> {
  type Action = DeactivateDid;
  type Output = ();

  async fn create<'i, S>(
    _action: Self::Action,
    expiration: Option<u64>,
    identity: &'i mut OnChainIdentity,
    client: &IdentityClient<S>,
  ) -> Result<CreateProposalTx<'i, Self::Action>, Error>
  where
    S: Signer<IotaKeySignature> + Sync,
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
      move_calls::identity::propose_deactivation(identity_ref, controller_cap_ref, expiration, client.package_id())
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
    S: Signer<IotaKeySignature> + Sync,
  {
    let proposal_id = self.id();
    let identity_ref = client
      .get_object_ref_by_id(identity.id())
      .await?
      .expect("identity exists on-chain");
    let controller_cap_ref = identity.get_controller_cap(client).await?;

    let tx =
      move_calls::identity::execute_deactivation(identity_ref, controller_cap_ref, proposal_id, client.package_id())
        .map_err(|e| Error::TransactionBuildingFailed(e.to_string()))?;

    Ok(ExecuteProposalTx {
      identity,
      tx,
      _action: PhantomData,
    })
  }

  fn parse_tx_effects(_tx_response: &IotaTransactionBlockResponse) -> Result<Self::Output, Error> {
    Ok(())
  }
}
